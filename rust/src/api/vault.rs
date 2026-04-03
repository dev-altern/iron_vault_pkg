use crate::api::types::*;
use crate::engine::{connection, convert};
use anyhow::{anyhow, Context, Result};
use r2d2::Pool;
use r2d2_sqlite::SqliteConnectionManager;
use std::collections::HashMap;
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

        Ok(IronVaultDb {
            write_pool,
            read_pool,
            tenant_id,
            config,
            path,
            closed: false,
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
                 WHERE type = 'table' AND name NOT LIKE 'sqlite_%'",
                [],
                |row| row.get(0),
            )
            .unwrap_or(0);

        Ok(VaultStats {
            db_size_bytes,
            wal_size_bytes,
            total_tables,
            migration_version: 0, // Populated in Phase 3
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
