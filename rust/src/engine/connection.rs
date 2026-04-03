use crate::api::types::VaultConfig;
use r2d2::Pool;
use r2d2_sqlite::SqliteConnectionManager;
use zeroize::Zeroizing;

/// Applies SQLCipher key + PRAGMA chain on every new pooled connection.
///
/// Stored inside each r2d2 pool and called automatically whenever a
/// fresh connection is acquired. Guarantees every connection has the
/// correct encryption key and consistent SQLite settings.
#[derive(Debug)]
struct PragmaCustomizer {
    /// Hex-encoded 32-byte encryption key, zeroed on drop.
    key_hex: Zeroizing<String>,
    /// Configuration for SQLite PRAGMAs.
    config: VaultConfig,
}

impl r2d2::CustomizeConnection<rusqlite::Connection, rusqlite::Error>
    for PragmaCustomizer
{
    fn on_acquire(
        &self,
        conn: &mut rusqlite::Connection,
    ) -> Result<(), rusqlite::Error> {
        // ── Step 1: Set SQLCipher encryption key ──
        // MUST be the very first statement on a new connection.
        conn.execute_batch(&format!(
            "PRAGMA key = \"x'{}'\";",
            *self.key_hex
        ))?;

        // SQLCipher tuning: we use our own KDF (Argon2id), so disable
        // SQLCipher's internal PBKDF2 iterations.
        conn.execute_batch(
            "PRAGMA cipher_page_size = 4096;\
             PRAGMA kdf_iter = 1;",
        )?;

        // ── Step 2: Verify the key is correct ──
        // On an existing database, this will fail if the key is wrong.
        // On a new database, this succeeds and initializes the schema.
        conn.query_row("SELECT count(*) FROM sqlite_master", [], |_| Ok(()))?;

        // ── Step 3: SQLite tuning PRAGMAs ──
        if self.config.wal_mode {
            conn.pragma_update(None, "journal_mode", "wal")?;
        }
        conn.execute_batch(&format!(
            "PRAGMA foreign_keys = {};\
             PRAGMA busy_timeout = {};\
             PRAGMA cache_size = -{};\
             PRAGMA mmap_size = {};\
             PRAGMA journal_size_limit = {};",
            if self.config.foreign_keys { "ON" } else { "OFF" },
            self.config.busy_timeout_ms,
            self.config.cache_size_kb,
            self.config.mmap_size_bytes,
            self.config.journal_size_limit_bytes,
        ))?;

        Ok(())
    }
}

/// Build a pair of (write_pool, read_pool) for the given path and config.
///
/// The write pool has exactly `max(1, config.write_pool_size)` connections.
/// The read pool has `max(1, config.read_pool_size)` connections.
/// Both pools run the full SQLCipher + PRAGMA chain on every new connection.
pub(crate) fn build_pools(
    path: &str,
    key_hex: Zeroizing<String>,
    config: &VaultConfig,
) -> Result<
    (
        Pool<SqliteConnectionManager>,
        Pool<SqliteConnectionManager>,
    ),
    String,
> {
    let write_pool = Pool::builder()
        .max_size(config.write_pool_size.max(1))
        .connection_customizer(Box::new(PragmaCustomizer {
            key_hex: key_hex.clone(),
            config: config.clone(),
        }))
        .build(SqliteConnectionManager::file(path))
        .map_err(|e| format!("write pool failed: {}", e))?;

    let read_pool = Pool::builder()
        .max_size(config.read_pool_size.max(1))
        .connection_customizer(Box::new(PragmaCustomizer {
            key_hex,
            config: config.clone(),
        }))
        .build(SqliteConnectionManager::file(path))
        .map_err(|e| format!("read pool failed: {}", e))?;

    Ok((write_pool, read_pool))
}
