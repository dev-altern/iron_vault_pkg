use crate::api::types::*;
use crate::engine::{audit, backup, connection, convert, crypto, export, migration, notifier, query_builder, search, transaction, write_ops};
use anyhow::{anyhow, Context, Result};
use r2d2::Pool;
use r2d2_sqlite::SqliteConnectionManager;
use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use zeroize::Zeroizing;

// ─── IronVaultDb ─────────────────────────────────────────────────────

/// The main IronVault database handle.
///
/// Holds separate read and write connection pools to an encrypted SQLite
/// database (SQLCipher). All connections are configured with encryption
/// and tuned via `VaultConfig` PRAGMAs.
///
/// - Write pool: size 1 — serializes all writes, avoids lock contention.
/// - Read pool: size N — WAL mode allows concurrent readers.
///
/// Created via `IronVaultDb::open()`. Call `close()` before disposing,
/// or let the Dart GC handle cleanup.
pub struct IronVaultDb {
    write_pool: Pool<SqliteConnectionManager>,
    read_pool: Pool<SqliteConnectionManager>,
    tenant_id: String,
    #[allow(dead_code)]
    config: VaultConfig,
    path: String,
    closed: bool,
    /// Master encryption key (Zeroizing — zeroed on drop).
    /// Used to derive tenant-scoped field encryption keys via HKDF.
    encryption_key: Zeroizing<Vec<u8>>,
    /// Broadcast notifier for reactive streams.
    notifier: Arc<notifier::ChangeNotifier>,
    /// Current actor ID for audit logging (default: "system").
    actor_id: Mutex<String>,
    /// Full-text search engine (Tantivy).
    search_engine: Arc<search::SearchEngine>,
}

impl std::fmt::Debug for IronVaultDb {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("IronVaultDb")
            .field("path", &self.path)
            .field("tenant_id", &self.tenant_id)
            .field("closed", &self.closed)
            .finish()
    }
}

impl IronVaultDb {
    // ── Factory ──────────────────────────────────────────────────

    /// Open or create an encrypted database.
    ///
    /// - `path`: absolute filesystem path to the `.db` file.
    ///   Parent directory is created if it doesn't exist.
    /// - `encryption_key`: exactly 32 bytes (AES-256 key).
    ///   Derive with Argon2id; zero the Dart-side copy after this call.
    /// - `tenant_id`: bound for the lifetime of this instance.
    ///   Injected into every query for multi-tenant isolation.
    /// - `config`: pool sizes and PRAGMA tuning.
    pub fn open(
        path: String,
        encryption_key: Vec<u8>,
        tenant_id: String,
        config: VaultConfig,
    ) -> Result<IronVaultDb> {
        if encryption_key.len() != 32 {
            return Err(anyhow!(
                "VaultOpenException: encryption key must be exactly 32 bytes, got {}",
                encryption_key.len()
            ));
        }

        // Ensure parent directory exists
        if let Some(parent) = std::path::Path::new(&path).parent() {
            if !parent.as_os_str().is_empty() {
                std::fs::create_dir_all(parent).with_context(|| {
                    format!("VaultOpenException: cannot create directory {:?}", parent)
                })?;
            }
        }

        let key_hex = Zeroizing::new(hex::encode(&encryption_key));

        let (write_pool, read_pool) = connection::build_pools(&path, key_hex, &config)
            .map_err(|e| anyhow!("VaultOpenException: {}", e))?;

        // Eagerly verify the database is accessible (catches wrong-key early)
        write_pool.get().map_err(|e| {
            anyhow!(
                "VaultOpenException: cannot open database \
                 (wrong encryption key or corrupted file): {}",
                e
            )
        })?;

        let search_engine = Arc::new(search::SearchEngine::new(&path));
        Ok(IronVaultDb {
            write_pool,
            read_pool,
            tenant_id,
            config,
            path,
            closed: false,
            encryption_key: Zeroizing::new(encryption_key),
            notifier: Arc::new(notifier::ChangeNotifier::new()),
            actor_id: Mutex::new("system".to_string()),
            search_engine,
        })
    }

    // ── Lifecycle ────────────────────────────────────────────────

    /// Checkpoint WAL and mark the database as closed.
    ///
    /// After this call, all methods will return an error.
    /// Connection cleanup happens when the Dart GC drops this object.
    pub fn close(&mut self) -> Result<()> {
        if self.closed {
            return Ok(());
        }
        let _ = self.checkpoint_internal(CheckpointMode::Passive);
        self.notifier.close(); // Wake all watchers so they exit
        self.closed = true;
        Ok(())
    }

    /// Get the filesystem path of the database file.
    pub fn get_path(&self) -> Result<String> {
        Ok(self.path.clone())
    }

    /// Get the tenant ID bound to this database instance.
    pub fn get_tenant_id(&self) -> Result<String> {
        Ok(self.tenant_id.clone())
    }

    // ── Raw SQL Execution ────────────────────────────────────────

    /// Execute a write statement (INSERT, UPDATE, DELETE, DDL).
    ///
    /// Uses the **write** connection pool (serialized).
    /// Returns the number of rows affected.
    /// Parameters are bound positionally (`?1`, `?2`, ...).
    pub fn execute_raw(&self, sql: String, params: Vec<SqlValue>) -> Result<u64> {
        self.ensure_open()?;
        let conn = self.acquire_writer()?;
        let values: Vec<rusqlite::types::Value> =
            params.iter().map(convert::to_rusqlite).collect();
        let affected = conn
            .execute(&sql, rusqlite::params_from_iter(values))
            .with_context(|| format!("ExecuteException: {}", sql))?;
        Ok(affected as u64)
    }

    /// Execute a read query (SELECT).
    ///
    /// Uses the **read** connection pool (concurrent via WAL).
    /// Returns rows as a list of column-name → value maps.
    /// Parameters are bound positionally (`?1`, `?2`, ...).
    pub fn query_raw(
        &self,
        sql: String,
        params: Vec<SqlValue>,
    ) -> Result<Vec<HashMap<String, SqlValue>>> {
        self.ensure_open()?;
        let conn = self.acquire_reader()?;
        let values: Vec<rusqlite::types::Value> =
            params.iter().map(convert::to_rusqlite).collect();
        let mut stmt = conn
            .prepare(&sql)
            .with_context(|| format!("QueryException: failed to prepare: {}", sql))?;

        let column_names: Vec<String> =
            stmt.column_names().iter().map(|c| c.to_string()).collect();
        let column_count = column_names.len();

        let rows = stmt
            .query_map(rusqlite::params_from_iter(values), |row| {
                let mut map = HashMap::with_capacity(column_count);
                for i in 0..column_count {
                    let value: rusqlite::types::Value = row.get(i)?;
                    map.insert(column_names[i].clone(), convert::from_rusqlite(value));
                }
                Ok(map)
            })
            .with_context(|| "QueryException: failed to execute query")?;

        let mut result = Vec::new();
        for row in rows {
            result.push(row.with_context(|| "QueryException: failed to read row")?);
        }
        Ok(result)
    }

    // ── Database Operations ──────────────────────────────────────

    /// Trigger a WAL checkpoint.
    ///
    /// Returns the number of WAL pages and how many were successfully
    /// checkpointed. Call periodically (every ~15 min) and on app
    /// background to keep the WAL file from growing unbounded.
    pub fn checkpoint(&self, mode: CheckpointMode) -> Result<CheckpointResult> {
        self.ensure_open()?;
        self.checkpoint_internal(mode)
    }

    /// Return a snapshot of database statistics.
    pub fn stats(&self) -> Result<VaultStats> {
        self.ensure_open()?;
        let conn = self.acquire_reader()?;

        let page_count: i64 = conn
            .query_row("PRAGMA page_count", [], |row| row.get(0))
            .unwrap_or(0);
        let page_size: i64 = conn
            .query_row("PRAGMA page_size", [], |row| row.get(0))
            .unwrap_or(4096);
        let db_size_bytes = page_count * page_size;

        let wal_path = format!("{}-wal", self.path);
        let wal_size_bytes = std::fs::metadata(&wal_path)
            .map(|m| m.len() as i64)
            .unwrap_or(0);

        let total_tables: i32 = conn
            .query_row(
                "SELECT count(*) FROM sqlite_master \
                 WHERE type = 'table' AND name NOT LIKE 'sqlite_%' \
                 AND name NOT LIKE '\\_%' ESCAPE '\\'",
                [],
                |row| row.get(0),
            )
            .unwrap_or(0);

        let migration_version: i32 = conn
            .query_row(
                "SELECT MAX(version) FROM _migrations",
                [],
                |row| row.get(0),
            )
            .unwrap_or(0);

        Ok(VaultStats {
            db_size_bytes,
            wal_size_bytes,
            total_tables,
            migration_version,
            page_count,
            page_size,
        })
    }

    /// Run SQLite integrity check.
    ///
    /// Scans the entire database for corruption. Returns clean=true if
    /// no issues found. Can be slow on large databases — run sparingly.
    pub fn integrity_check(&self) -> Result<IntegrityReport> {
        self.ensure_open()?;
        let conn = self.acquire_reader()?;

        let mut stmt = conn.prepare("PRAGMA integrity_check")?;
        let rows = stmt.query_map([], |row| {
            let msg: String = row.get(0)?;
            Ok(msg)
        })?;

        let mut errors = Vec::new();
        let mut is_clean = true;
        for row in rows {
            let msg = row?;
            if msg != "ok" {
                is_clean = false;
                errors.push(msg);
            }
        }

        Ok(IntegrityReport { is_clean, errors })
    }

    /// Reclaim disk space from deleted rows.
    ///
    /// Rewrites the entire database file. Can be very slow on large
    /// databases — prefer `incremental_vacuum` when available (Phase 12).
    pub fn vacuum(&self) -> Result<()> {
        self.ensure_open()?;
        let conn = self.acquire_writer()?;
        conn.execute_batch("VACUUM")
            .context("Failed to vacuum database")?;
        Ok(())
    }

    // ── Query Builder: Read Operations ─────────────────────────

    /// Execute a query and return all matching rows.
    ///
    /// Tenant isolation and soft-delete guard are auto-injected.
    pub fn query_get(
        &self,
        spec: QuerySpec,
    ) -> Result<Vec<HashMap<String, SqlValue>>> {
        self.ensure_open()?;
        let (sql, params) = query_builder::build_select(&spec, &self.tenant_id)?;
        self.execute_read_query(&sql, params)
    }

    /// Execute a query and return the first matching row (or None).
    pub fn query_first(
        &self,
        spec: QuerySpec,
    ) -> Result<Option<HashMap<String, SqlValue>>> {
        self.ensure_open()?;
        let mut limited = spec;
        limited.limit = Some(1);
        let (sql, params) = query_builder::build_select(&limited, &self.tenant_id)?;
        let rows = self.execute_read_query(&sql, params)?;
        Ok(rows.into_iter().next())
    }

    /// Count matching rows.
    pub fn query_count(&self, spec: QuerySpec) -> Result<u64> {
        self.ensure_open()?;
        let (sql, params) = query_builder::build_count(&spec, &self.tenant_id)?;
        let conn = self.acquire_reader()?;
        let values: Vec<rusqlite::types::Value> = params;
        let count: i64 = conn
            .query_row(&sql, rusqlite::params_from_iter(values), |row| row.get(0))
            .context("Failed to count rows")?;
        Ok(count as u64)
    }

    /// Check if any rows match.
    pub fn query_exists(&self, spec: QuerySpec) -> Result<bool> {
        Ok(self.query_count(spec)? > 0)
    }

    /// Execute a paginated query.
    ///
    /// `page` is 0-based. Returns a `Page` with items, total, and metadata.
    pub fn query_paginate(
        &self,
        spec: QuerySpec,
        page: u32,
        page_size: u32,
    ) -> Result<Page> {
        self.ensure_open()?;
        if page_size == 0 {
            return Err(anyhow!("page_size must be > 0"));
        }

        // Get total count (without limit/offset)
        let total = self.query_count(spec.clone())?;
        let total_pages = if total == 0 {
            0
        } else {
            ((total as u32) + page_size - 1) / page_size
        };

        // Get page items
        let mut paged = spec;
        paged.limit = Some(page_size);
        paged.offset = Some(page * page_size);
        let (sql, params) = query_builder::build_select(&paged, &self.tenant_id)?;
        let items = self.execute_read_query(&sql, params)?;

        Ok(Page {
            items,
            total,
            total_pages,
            page,
            page_size,
        })
    }

    /// Execute an aggregate query (COUNT, SUM, AVG, MIN, MAX).
    pub fn query_aggregate(
        &self,
        spec: QuerySpec,
        expressions: Vec<AggExpr>,
    ) -> Result<HashMap<String, SqlValue>> {
        self.ensure_open()?;
        let (sql, params) =
            query_builder::build_aggregate(&spec, &expressions, &self.tenant_id)?;
        let rows = self.execute_read_query(&sql, params)?;
        Ok(rows.into_iter().next().unwrap_or_default())
    }

    // ── Query Builder: Write Operations ──────────────────────────

    /// Insert a row. Returns the generated or provided id.
    ///
    /// `tenant_id` is auto-injected. `id` is auto-generated (UUID) if not in data.
    /// `created_at` and `updated_at` are auto-set if not provided.
    pub fn query_insert(
        &self,
        table: String,
        data: HashMap<String, SqlValue>,
    ) -> Result<String> {
        self.ensure_open()?;
        let (sql, params, id) =
            write_ops::build_insert(&table, data, &self.tenant_id)?;
        let conn = self.acquire_writer()?;
        conn.execute(&sql, rusqlite::params_from_iter(params))
            .with_context(|| format!("InsertException: {}", table))?;
        self.notifier.notify(&table, &self.tenant_id);
        Ok(id)
    }

    /// Update a row by id. Returns the number of rows affected (0 or 1).
    ///
    /// Tenant isolation and soft-delete guard are enforced.
    /// `updated_at` is auto-set if not in data.
    pub fn query_update(
        &self,
        table: String,
        id: String,
        data: HashMap<String, SqlValue>,
    ) -> Result<u64> {
        self.ensure_open()?;
        let (sql, params) =
            write_ops::build_update(&table, &id, data, &self.tenant_id)?;
        let conn = self.acquire_writer()?;
        let affected = conn
            .execute(&sql, rusqlite::params_from_iter(params))
            .with_context(|| format!("UpdateException: {} id={}", table, id))?;
        if affected > 0 {
            self.notifier.notify(&table, &self.tenant_id);
        }
        Ok(affected as u64)
    }

    /// Upsert (insert or update on conflict).
    ///
    /// Returns the id of the inserted/updated row.
    pub fn query_upsert(
        &self,
        table: String,
        data: HashMap<String, SqlValue>,
        conflict_column: String,
    ) -> Result<String> {
        self.ensure_open()?;
        let (sql, params, id) =
            write_ops::build_upsert(&table, data, &conflict_column, &self.tenant_id)?;
        let conn = self.acquire_writer()?;
        conn.execute(&sql, rusqlite::params_from_iter(params))
            .with_context(|| format!("UpsertException: {}", table))?;
        self.notifier.notify(&table, &self.tenant_id);
        Ok(id)
    }

    /// Soft-delete a row (sets `deleted_at` to current timestamp).
    ///
    /// Returns rows affected (0 if not found or already deleted).
    pub fn query_delete(&self, table: String, id: String) -> Result<u64> {
        self.ensure_open()?;
        let (sql, params) =
            write_ops::build_soft_delete(&table, &id, &self.tenant_id)?;
        let conn = self.acquire_writer()?;
        let affected = conn
            .execute(&sql, rusqlite::params_from_iter(params))
            .with_context(|| format!("DeleteException: {} id={}", table, id))?;
        if affected > 0 {
            self.notifier.notify(&table, &self.tenant_id);
        }
        Ok(affected as u64)
    }

    /// Permanently delete a row (irreversible).
    ///
    /// Tenant isolation is enforced. Returns rows affected.
    pub fn query_hard_delete(&self, table: String, id: String) -> Result<u64> {
        self.ensure_open()?;
        let (sql, params) =
            write_ops::build_hard_delete(&table, &id, &self.tenant_id)?;
        let conn = self.acquire_writer()?;
        let affected = conn
            .execute(&sql, rusqlite::params_from_iter(params))
            .with_context(|| format!("HardDeleteException: {} id={}", table, id))?;
        if affected > 0 {
            self.notifier.notify(&table, &self.tenant_id);
        }
        Ok(affected as u64)
    }

    /// Insert multiple rows in a single transaction. Returns list of ids.
    pub fn query_insert_batch(
        &self,
        table: String,
        rows: Vec<HashMap<String, SqlValue>>,
    ) -> Result<Vec<String>> {
        self.ensure_open()?;
        let conn = self.acquire_writer()?;
        let mut ids = Vec::with_capacity(rows.len());

        conn.execute_batch("BEGIN IMMEDIATE")
            .context("Failed to begin transaction")?;

        for row_data in rows {
            let (sql, params, id) =
                write_ops::build_insert(&table, row_data, &self.tenant_id)?;
            match conn.execute(&sql, rusqlite::params_from_iter(params)) {
                Ok(_) => ids.push(id),
                Err(e) => {
                    let _ = conn.execute_batch("ROLLBACK");
                    return Err(anyhow!(
                        "BatchInsertException: {} — rolled back: {}",
                        table,
                        e
                    ));
                }
            }
        }

        conn.execute_batch("COMMIT")
            .context("Failed to commit batch insert")?;
        if !ids.is_empty() {
            self.notifier.notify(&table, &self.tenant_id);
        }
        Ok(ids)
    }

    /// Update multiple rows in a single transaction. Returns total rows affected.
    pub fn query_update_batch(
        &self,
        table: String,
        updates: Vec<UpdateEntry>,
    ) -> Result<u64> {
        self.ensure_open()?;
        let conn = self.acquire_writer()?;
        let mut total_affected = 0u64;

        conn.execute_batch("BEGIN IMMEDIATE")
            .context("Failed to begin transaction")?;

        for entry in updates {
            let (sql, params) =
                write_ops::build_update(&table, &entry.id, entry.data, &self.tenant_id)?;
            match conn.execute(&sql, rusqlite::params_from_iter(params)) {
                Ok(n) => total_affected += n as u64,
                Err(e) => {
                    let _ = conn.execute_batch("ROLLBACK");
                    return Err(anyhow!(
                        "BatchUpdateException: {} id={} — rolled back: {}",
                        table,
                        entry.id,
                        e
                    ));
                }
            }
        }

        conn.execute_batch("COMMIT")
            .context("Failed to commit batch update")?;
        if total_affected > 0 {
            self.notifier.notify(&table, &self.tenant_id);
        }
        Ok(total_affected)
    }

    /// Soft-delete multiple rows in a single transaction. Returns total rows affected.
    pub fn query_delete_batch(
        &self,
        table: String,
        ids: Vec<String>,
    ) -> Result<u64> {
        self.ensure_open()?;
        let conn = self.acquire_writer()?;
        let mut total_affected = 0u64;

        conn.execute_batch("BEGIN IMMEDIATE")
            .context("Failed to begin transaction")?;

        for id in &ids {
            let (sql, params) =
                write_ops::build_soft_delete(&table, id, &self.tenant_id)?;
            match conn.execute(&sql, rusqlite::params_from_iter(params)) {
                Ok(n) => total_affected += n as u64,
                Err(e) => {
                    let _ = conn.execute_batch("ROLLBACK");
                    return Err(anyhow!(
                        "BatchDeleteException: {} id={} — rolled back: {}",
                        table,
                        id,
                        e
                    ));
                }
            }
        }

        conn.execute_batch("COMMIT")
            .context("Failed to commit batch delete")?;
        if total_affected > 0 {
            self.notifier.notify(&table, &self.tenant_id);
        }
        Ok(total_affected)
    }

    // ── Migrations ────────────────────────────────────────────────

    /// Run forward migrations.
    ///
    /// Applies all pending migrations (version > current) in version order.
    /// Each migration runs in its own transaction. If a migration fails,
    /// it is rolled back and an error is returned (previously-applied
    /// migrations remain intact).
    ///
    /// **Checksum protection:** if a previously-applied migration's SQL
    /// has been modified, returns `MigrationChecksumException`.
    pub fn migrate(&self, migrations: Vec<VaultMigration>) -> Result<MigrationReport> {
        self.ensure_open()?;
        let conn = self.acquire_writer()?;
        migration::migrate(&conn, &migrations)
    }

    /// Rollback to a target version.
    ///
    /// Rolls back all applied migrations with version > `target_version`,
    /// in reverse order. Each rollback executes the `down` SQL.
    ///
    /// Returns `MigrationNoRollbackException` if any migration lacks `down` SQL.
    /// The `migrations` list must include definitions for all versions to roll back.
    pub fn rollback_to(
        &self,
        target_version: u32,
        migrations: Vec<VaultMigration>,
    ) -> Result<MigrationReport> {
        self.ensure_open()?;
        let conn = self.acquire_writer()?;
        migration::rollback_to(&conn, target_version, &migrations)
    }

    /// Get all applied migration records.
    pub fn get_migrations(&self) -> Result<Vec<MigrationRecord>> {
        self.ensure_open()?;
        let conn = self.acquire_reader()?;
        migration::ensure_table(&conn)?;
        migration::get_applied(&conn)
    }

    // ── Transactions ─────────────────────────────────────────────

    /// Execute multiple operations in a single ACID transaction.
    ///
    /// All operations run inside `BEGIN IMMEDIATE ... COMMIT`.
    /// If any operation fails, the entire transaction is rolled back.
    /// Savepoints provide partial rollback within the transaction.
    ///
    /// Tenant isolation is enforced on all operations automatically.
    pub fn transaction(&self, ops: Vec<Op>) -> Result<TransactionResult> {
        self.ensure_open()?;
        let conn = self.acquire_writer()?;
        let result = transaction::execute_transaction(&conn, &ops, &self.tenant_id)?;
        // Notify for each affected table
        for table in &result.affected_tables {
            self.notifier.notify(table, &self.tenant_id);
        }
        Ok(result)
    }

    /// Update a row with optimistic locking.
    ///
    /// Succeeds only if the row's `version` column matches `expected_version`.
    /// On success, `version` is incremented by 1 atomically.
    /// On mismatch (another writer updated first), returns
    /// `OptimisticLockException`.
    ///
    /// The table must have a `version INTEGER NOT NULL DEFAULT 1` column.
    pub fn update_with_version(
        &self,
        table: String,
        id: String,
        expected_version: i64,
        data: HashMap<String, SqlValue>,
    ) -> Result<()> {
        self.ensure_open()?;
        let conn = self.acquire_writer()?;
        transaction::update_with_version(
            &conn,
            &table,
            &id,
            expected_version,
            data,
            &self.tenant_id,
        )?;
        self.notifier.notify(&table, &self.tenant_id);
        Ok(())
    }

    // ── Encryption ───────────────────────────────────────────────

    /// Encrypt a plaintext string using AES-256-GCM.
    ///
    /// Uses a tenant-scoped key derived via HKDF from the master key.
    /// Each call produces different ciphertext (random 12-byte nonce).
    /// Returns JSON: `{"ct":"<base64>","nonce":"<base64>","kid":"v1"}`.
    pub fn encrypt_field(&self, plaintext: String) -> Result<String> {
        self.ensure_open()?;
        let field_key = crypto::tenant_field_key(&self.encryption_key, &self.tenant_id)?;
        crypto::encrypt_field(&plaintext, &field_key)
    }

    /// Decrypt a ciphertext string produced by `encrypt_field`.
    ///
    /// Uses the same tenant-scoped key. Returns the original plaintext.
    /// Fails if the key is wrong or data has been tampered with.
    pub fn decrypt_field(&self, ciphertext_json: String) -> Result<String> {
        self.ensure_open()?;
        let field_key = crypto::tenant_field_key(&self.encryption_key, &self.tenant_id)?;
        crypto::decrypt_field(&ciphertext_json, &field_key)
    }

    /// Derive an HKDF key for a specific purpose.
    ///
    /// Available purposes: "sqlcipher", "audit_hmac", "backup",
    /// or any custom string. Returns 32 bytes.
    pub fn derive_purpose_key(&self, purpose: String) -> Result<Vec<u8>> {
        self.ensure_open()?;
        crypto::hkdf_derive(&self.encryption_key, &purpose)
    }

    // ── Reactive Streams ──────────────────────────────────────────

    /// Watch a query — emits results whenever the table changes.
    ///
    /// Returns a Dart `Stream<List<Map<String, SqlValue>>>`.
    /// Initial result emitted immediately. Re-executes on every write
    /// to the table. Only emits if the result set actually changed
    /// (distinct emission via hash comparison).
    ///
    /// The stream stops when the Dart side cancels or the DB is closed.
    pub fn watch_query(
        &self,
        spec: QuerySpec,
        sink: crate::frb_generated::StreamSink<Vec<HashMap<String, SqlValue>>>,
    ) -> Result<()> {
        self.ensure_open()?;
        let read_pool = self.read_pool.clone();
        let tenant_id = self.tenant_id.clone();
        let notifier = self.notifier.clone();
        let table = spec.table.clone();

        std::thread::spawn(move || {
            let key = format!("{}:{}", table, tenant_id);

            // Initial emission
            let initial = Self::execute_query_on_pool(&read_pool, &spec, &tenant_id)
                .unwrap_or_default();
            let mut prev_hash = Self::hash_results(&initial);
            let _ = sink.add(initial);

            loop {
                if !notifier.wait(&key) {
                    break; // DB closed
                }

                let new_results = Self::execute_query_on_pool(&read_pool, &spec, &tenant_id)
                    .unwrap_or_default();
                let new_hash = Self::hash_results(&new_results);

                if new_hash != prev_hash {
                    if sink.add(new_results).is_err() {
                        break; // Dart cancelled
                    }
                    prev_hash = new_hash;
                }
            }
        });

        Ok(())
    }

    /// Watch a single row — emits the row or None if soft-deleted.
    pub fn watch_row(
        &self,
        table: String,
        id: String,
        sink: crate::frb_generated::StreamSink<Option<HashMap<String, SqlValue>>>,
    ) -> Result<()> {
        self.ensure_open()?;
        let read_pool = self.read_pool.clone();
        let tenant_id = self.tenant_id.clone();
        let notifier = self.notifier.clone();
        let table_clone = table.clone();

        std::thread::spawn(move || {
            let key = format!("{}:{}", table_clone, tenant_id);
            let spec = QuerySpec {
                table: table_clone,
                conditions: vec![Condition::Eq {
                    column: "id".into(),
                    value: SqlValue::Text(id),
                }],
                or_conditions: vec![],
                order_by: vec![],
                limit: Some(1),
                offset: None,
                joins: vec![],
                columns: vec![],
                include_deleted: false,
            };

            let initial = Self::execute_query_on_pool(&read_pool, &spec, &tenant_id)
                .unwrap_or_default();
            let row = initial.into_iter().next();
            let mut prev_hash = format!("{:?}", row);
            let _ = sink.add(row);

            loop {
                if !notifier.wait(&key) {
                    break;
                }

                let results = Self::execute_query_on_pool(&read_pool, &spec, &tenant_id)
                    .unwrap_or_default();
                let new_row = results.into_iter().next();
                let new_hash = format!("{:?}", new_row);

                if new_hash != prev_hash {
                    if sink.add(new_row).is_err() {
                        break;
                    }
                    prev_hash = new_hash;
                }
            }
        });

        Ok(())
    }

    /// Watch aggregate expressions — emits updated aggregates on table changes.
    pub fn watch_aggregate(
        &self,
        spec: QuerySpec,
        expressions: Vec<AggExpr>,
        sink: crate::frb_generated::StreamSink<HashMap<String, SqlValue>>,
    ) -> Result<()> {
        self.ensure_open()?;
        let read_pool = self.read_pool.clone();
        let tenant_id = self.tenant_id.clone();
        let notifier = self.notifier.clone();
        let table = spec.table.clone();

        std::thread::spawn(move || {
            let key = format!("{}:{}", table, tenant_id);

            let exec = || -> HashMap<String, SqlValue> {
                let (sql, params) =
                    query_builder::build_aggregate(&spec, &expressions, &tenant_id)
                        .unwrap_or_default();
                let conn = read_pool.get().ok();
                conn.and_then(|c| {
                    let mut stmt = c.prepare(&sql).ok()?;
                    let cols: Vec<String> = stmt.column_names().iter().map(|c| c.to_string()).collect();
                    let count = cols.len();
                    stmt.query_row(rusqlite::params_from_iter(params), |row| {
                        let mut map = HashMap::with_capacity(count);
                        for i in 0..count {
                            let v: rusqlite::types::Value = row.get(i)?;
                            map.insert(cols[i].clone(), convert::from_rusqlite(v));
                        }
                        Ok(map)
                    }).ok()
                })
                .unwrap_or_default()
            };

            let initial = exec();
            let mut prev_hash = format!("{:?}", initial);
            let _ = sink.add(initial);

            loop {
                if !notifier.wait(&key) {
                    break;
                }

                let new_result = exec();
                let new_hash = format!("{:?}", new_result);

                if new_hash != prev_hash {
                    if sink.add(new_result).is_err() {
                        break;
                    }
                    prev_hash = new_hash;
                }
            }
        });

        Ok(())
    }

    /// Get the notification version for a table channel (for testing).
    pub fn notification_version(&self, table: String) -> Result<u64> {
        self.ensure_open()?;
        let key = format!("{}:{}", table, self.tenant_id);
        Ok(self.notifier.version(&key))
    }

    // ── Audit Log ────────────────────────────────────────────────

    /// Set the actor ID for audit logging.
    ///
    /// The actor is injected into every audit entry automatically.
    /// Call after authentication. Default is "system".
    pub fn set_actor(&self, actor_id: String) -> Result<()> {
        self.ensure_open()?;
        *self.actor_id.lock().unwrap() = actor_id;
        Ok(())
    }

    /// Reset the actor ID to "system".
    pub fn clear_actor(&self) -> Result<()> {
        self.ensure_open()?;
        *self.actor_id.lock().unwrap() = "system".to_string();
        Ok(())
    }

    /// Get the current actor ID.
    pub fn get_actor(&self) -> Result<String> {
        self.ensure_open()?;
        Ok(self.actor_id.lock().unwrap().clone())
    }

    /// Manually write an audit entry.
    ///
    /// For application-level events (login, export, etc.) that aren't
    /// automatic database writes. HMAC-signed with the audit key.
    pub fn write_audit(
        &self,
        table_name: String,
        row_id: String,
        operation: String,
        before_json: Option<String>,
        after_json: Option<String>,
        changed_fields: Option<String>,
    ) -> Result<String> {
        self.ensure_open()?;
        let conn = self.acquire_writer()?;
        let actor = self.actor_id.lock().unwrap().clone();
        let hmac_key = crypto::hkdf_derive(&self.encryption_key, "audit_hmac")?;
        audit::record(
            &conn, &table_name, &row_id, &operation, &actor,
            &self.tenant_id, before_json.as_deref(), after_json.as_deref(),
            changed_fields.as_deref(), &hmac_key,
        )
    }

    /// Get audit history for a specific row.
    pub fn get_history(
        &self,
        table_name: String,
        row_id: String,
        limit: u32,
    ) -> Result<Vec<AuditEntry>> {
        self.ensure_open()?;
        let conn = self.acquire_reader()?;
        audit::get_history(&conn, &table_name, &row_id, &self.tenant_id, limit)
    }

    /// Get audit history for a specific actor.
    pub fn get_actor_history(
        &self,
        actor_id: String,
        from: Option<i64>,
        to: Option<i64>,
        limit: u32,
    ) -> Result<Vec<AuditEntry>> {
        self.ensure_open()?;
        let conn = self.acquire_reader()?;
        audit::get_actor_history(&conn, &actor_id, &self.tenant_id, from, to, limit)
    }

    /// Get audit history for an entire table.
    pub fn get_table_history(
        &self,
        table_name: String,
        from: Option<i64>,
        to: Option<i64>,
        limit: u32,
    ) -> Result<Vec<AuditEntry>> {
        self.ensure_open()?;
        let conn = self.acquire_reader()?;
        audit::get_table_history(&conn, &table_name, &self.tenant_id, from, to, limit)
    }

    /// Verify integrity of audit log entries.
    ///
    /// Recomputes HMAC checksums and reports any tampered entries.
    pub fn verify_audit_integrity(
        &self,
        from: Option<i64>,
        to: Option<i64>,
    ) -> Result<AuditIntegrityReport> {
        self.ensure_open()?;
        let conn = self.acquire_reader()?;
        let hmac_key = crypto::hkdf_derive(&self.encryption_key, "audit_hmac")?;
        audit::verify_integrity(&conn, &self.tenant_id, &hmac_key, from, to)
    }

    // ── Backup & Export ──────────────────────────────────────────

    /// Create a backup of the database.
    ///
    /// Optionally compresses (zstd) and encrypts (AES-256-GCM with
    /// HKDF-derived backup key). Returns the backup path, size, and
    /// BLAKE3 checksum for verification on restore.
    pub fn backup(
        &self,
        output_path: String,
        compress: bool,
        encrypt: bool,
    ) -> Result<BackupResult> {
        self.ensure_open()?;
        // Checkpoint WAL first to flush all changes to the main DB file
        let _ = self.checkpoint_internal(CheckpointMode::Passive);
        let backup_key = if encrypt {
            crypto::hkdf_derive(&self.encryption_key, "backup")?
        } else {
            vec![0u8; 32]
        };
        backup::create_backup(&self.path, &output_path, compress, encrypt, &backup_key)
    }

    /// Verify a backup file without restoring.
    pub fn verify_backup(&self, backup_path: String) -> Result<BackupVerifyReport> {
        self.ensure_open()?;
        let backup_key = crypto::hkdf_derive(&self.encryption_key, "backup")?;
        backup::verify_backup(&backup_path, &backup_key)
    }

    /// Restore a backup to a target path.
    ///
    /// Does NOT require an open database — operates on files directly.
    /// Verifies checksum, decrypts, decompresses, runs integrity_check.
    pub fn restore_backup(
        &self,
        backup_path: String,
        target_path: String,
        expected_checksum: Option<String>,
    ) -> Result<RestoreResult> {
        self.ensure_open()?;
        let backup_key = crypto::hkdf_derive(&self.encryption_key, "backup")?;
        backup::restore_backup(
            &backup_path,
            &target_path,
            &backup_key,
            expected_checksum.as_deref(),
        )
    }

    /// Export a table's data to CSV, JSON, or JSONL.
    ///
    /// Respects tenant isolation and soft-delete guard.
    pub fn export_table(
        &self,
        table: String,
        format: ExportFormat,
        columns: Option<Vec<String>>,
    ) -> Result<Vec<u8>> {
        self.ensure_open()?;
        let conn = self.acquire_reader()?;
        export::export_table(&conn, &table, &self.tenant_id, &format, &columns)
    }

    // ── Full-Text Search ─────────────────────────────────────────

    /// Build or open a Tantivy search index for a table.
    ///
    /// Call after creating the table (via migration or raw SQL).
    /// Fields define which columns are indexed and their weights.
    pub fn build_search_index(
        &self,
        table: String,
        fields: Vec<SearchField>,
    ) -> Result<()> {
        self.ensure_open()?;
        self.search_engine.build_index(&table, &fields)
    }

    /// Index a single row in the search engine.
    ///
    /// `fields` maps column names to their text values.
    /// Call after insert/update to keep the index current.
    pub fn search_index_row(
        &self,
        table: String,
        id: String,
        fields: HashMap<String, String>,
    ) -> Result<()> {
        self.ensure_open()?;
        self.search_engine.index_row(&table, &id, &fields)
    }

    /// Remove a row from the search index.
    ///
    /// Call after soft-delete or hard-delete.
    pub fn search_remove_row(&self, table: String, id: String) -> Result<()> {
        self.ensure_open()?;
        self.search_engine.remove_from_index(&table, &id)
    }

    /// Search the index using Tantivy query syntax.
    ///
    /// Supports: simple words, "phrase search", field:value,
    /// boolean (AND OR NOT), fuzzy~N, wildcards, boost^N.
    /// Returns results ranked by relevance score.
    pub fn search(
        &self,
        table: String,
        query: String,
        limit: u32,
        highlight: bool,
    ) -> Result<Vec<SearchHit>> {
        self.ensure_open()?;
        self.search_engine.search(&table, &query, limit, highlight)
    }

    /// Get statistics about a search index.
    pub fn search_index_stats(&self, table: String) -> Result<IndexStats> {
        self.ensure_open()?;
        self.search_engine.index_stats(&table)
    }

    // ── Private Helpers ──────────────────────────────────────────

    fn ensure_open(&self) -> Result<()> {
        if self.closed {
            Err(anyhow!("VaultClosedException: database is closed"))
        } else {
            Ok(())
        }
    }

    fn acquire_writer(
        &self,
    ) -> Result<r2d2::PooledConnection<SqliteConnectionManager>> {
        self.write_pool
            .get()
            .map_err(|e| anyhow!("PoolExhaustedException: {}", e))
    }

    fn acquire_reader(
        &self,
    ) -> Result<r2d2::PooledConnection<SqliteConnectionManager>> {
        self.read_pool
            .get()
            .map_err(|e| anyhow!("PoolExhaustedException: {}", e))
    }

    /// Execute a read query with pre-built SQL and params, returning rows.
    fn execute_read_query(
        &self,
        sql: &str,
        params: Vec<rusqlite::types::Value>,
    ) -> Result<Vec<HashMap<String, SqlValue>>> {
        let conn = self.acquire_reader()?;
        let mut stmt = conn
            .prepare(sql)
            .with_context(|| format!("QueryException: failed to prepare: {}", sql))?;

        let column_names: Vec<String> =
            stmt.column_names().iter().map(|c| c.to_string()).collect();
        let column_count = column_names.len();

        let rows = stmt
            .query_map(rusqlite::params_from_iter(params), |row| {
                let mut map = HashMap::with_capacity(column_count);
                for i in 0..column_count {
                    let value: rusqlite::types::Value = row.get(i)?;
                    map.insert(column_names[i].clone(), convert::from_rusqlite(value));
                }
                Ok(map)
            })
            .with_context(|| "QueryException: failed to execute query")?;

        let mut result = Vec::new();
        for row in rows {
            result.push(row.with_context(|| "QueryException: failed to read row")?);
        }
        Ok(result)
    }

    /// Execute a query on the read pool (used by watch threads).
    fn execute_query_on_pool(
        pool: &Pool<SqliteConnectionManager>,
        spec: &QuerySpec,
        tenant_id: &str,
    ) -> Result<Vec<HashMap<String, SqlValue>>> {
        let (sql, params) = query_builder::build_select(spec, tenant_id)?;
        let conn = pool.get().map_err(|e| anyhow!("PoolExhaustedException: {}", e))?;
        let mut stmt = conn.prepare(&sql)?;
        let column_names: Vec<String> =
            stmt.column_names().iter().map(|c| c.to_string()).collect();
        let column_count = column_names.len();
        let rows = stmt.query_map(rusqlite::params_from_iter(params), |row| {
            let mut map = HashMap::with_capacity(column_count);
            for i in 0..column_count {
                let value: rusqlite::types::Value = row.get(i)?;
                map.insert(column_names[i].clone(), convert::from_rusqlite(value));
            }
            Ok(map)
        })?;
        let mut result = Vec::new();
        for row in rows {
            result.push(row?);
        }
        Ok(result)
    }

    /// Hash a result set for distinct emission comparison.
    fn hash_results(results: &[HashMap<String, SqlValue>]) -> u64 {
        use std::hash::{Hash, Hasher};
        let mut hasher = std::collections::hash_map::DefaultHasher::new();
        format!("{:?}", results).hash(&mut hasher);
        hasher.finish()
    }

    fn checkpoint_internal(&self, mode: CheckpointMode) -> Result<CheckpointResult> {
        let conn = self.acquire_writer()?;
        let mode_str = match mode {
            CheckpointMode::Passive => "PASSIVE",
            CheckpointMode::Full => "FULL",
            CheckpointMode::Restart => "RESTART",
            CheckpointMode::Truncate => "TRUNCATE",
        };
        let result = conn
            .query_row(
                &format!("PRAGMA wal_checkpoint({})", mode_str),
                [],
                |row| {
                    Ok(CheckpointResult {
                        wal_pages: row.get(1).unwrap_or(0),
                        checkpointed_pages: row.get(2).unwrap_or(0),
                    })
                },
            )
            .context("Failed to checkpoint WAL")?;
        Ok(result)
    }
}
