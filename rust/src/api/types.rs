/// Configuration for the IronVault database engine.
///
/// Controls connection pool sizes, cache settings, and SQLite PRAGMAs.
/// Use factory constructors for common configurations.
#[derive(Debug, Clone)]
pub struct VaultConfig {
    /// Number of concurrent read connections (default: 7).
    pub read_pool_size: u32,
    /// Number of write connections (always 1 — do not increase).
    pub write_pool_size: u32,
    /// Milliseconds to wait for a busy connection (default: 5000).
    pub busy_timeout_ms: u32,
    /// SQLite page cache size in KB (default: 65536 = 64MB).
    pub cache_size_kb: u32,
    /// Memory-mapped I/O size in bytes (default: 256MB).
    pub mmap_size_bytes: u64,
    /// Maximum WAL journal size in bytes (default: 64MB).
    pub journal_size_limit_bytes: u64,
    /// Enable foreign key constraints (default: true).
    pub foreign_keys: bool,
    /// Enable WAL mode for concurrent reads (default: true).
    pub wal_mode: bool,
}

impl VaultConfig {
    /// Production defaults — 64MB cache, 7 readers, WAL mode.
    #[flutter_rust_bridge::frb(sync)]
    pub fn production() -> Self {
        Self {
            read_pool_size: 7,
            write_pool_size: 1,
            busy_timeout_ms: 5000,
            cache_size_kb: 65536,
            mmap_size_bytes: 268_435_456,
            journal_size_limit_bytes: 67_108_864,
            foreign_keys: true,
            wal_mode: true,
        }
    }

    /// Development defaults — smaller cache, suitable for debugging.
    #[flutter_rust_bridge::frb(sync)]
    pub fn development() -> Self {
        Self {
            read_pool_size: 4,
            write_pool_size: 1,
            busy_timeout_ms: 5000,
            cache_size_kb: 16384,
            mmap_size_bytes: 67_108_864,
            journal_size_limit_bytes: 33_554_432,
            foreign_keys: true,
            wal_mode: true,
        }
    }

    /// Low-memory defaults — 8MB cache, 3 readers. For budget devices.
    #[flutter_rust_bridge::frb(sync)]
    pub fn low_memory() -> Self {
        Self {
            read_pool_size: 3,
            write_pool_size: 1,
            busy_timeout_ms: 5000,
            cache_size_kb: 8192,
            mmap_size_bytes: 67_108_864,
            journal_size_limit_bytes: 16_777_216,
            foreign_keys: true,
            wal_mode: true,
        }
    }

    /// Test defaults — small pool, minimal cache, no WAL.
    #[flutter_rust_bridge::frb(sync)]
    pub fn test_config() -> Self {
        Self {
            read_pool_size: 2,
            write_pool_size: 1,
            busy_timeout_ms: 1000,
            cache_size_kb: 2048,
            mmap_size_bytes: 0,
            journal_size_limit_bytes: 4_194_304,
            foreign_keys: true,
            wal_mode: false,
        }
    }
}

impl Default for VaultConfig {
    fn default() -> Self {
        Self::production()
    }
}

/// A dynamic SQL value — represents any SQLite column type.
#[derive(Debug, Clone)]
pub enum SqlValue {
    /// SQL NULL.
    Null,
    /// 64-bit signed integer.
    Integer(i64),
    /// 64-bit floating point.
    Real(f64),
    /// UTF-8 text string.
    Text(String),
    /// Binary blob.
    Blob(Vec<u8>),
}

/// WAL checkpoint mode.
#[derive(Debug, Clone)]
pub enum CheckpointMode {
    /// Checkpoint as many WAL pages as possible without waiting.
    Passive,
    /// Checkpoint all WAL pages, waiting for readers to finish.
    Full,
    /// Like Full, then restart the WAL file from the beginning.
    Restart,
    /// Like Restart, then truncate the WAL file to zero bytes.
    Truncate,
}

/// Result of a WAL checkpoint operation.
#[derive(Debug, Clone)]
pub struct CheckpointResult {
    /// Total number of WAL pages.
    pub wal_pages: i32,
    /// Number of pages successfully checkpointed.
    pub checkpointed_pages: i32,
}

/// Database statistics snapshot.
#[derive(Debug, Clone)]
pub struct VaultStats {
    /// Total database file size in bytes.
    pub db_size_bytes: i64,
    /// WAL file size in bytes (0 if no WAL).
    pub wal_size_bytes: i64,
    /// Number of user tables (excludes sqlite_* internal tables).
    pub total_tables: i32,
    /// Current migration version (0 if no migrations applied yet).
    pub migration_version: i32,
    /// Database page count.
    pub page_count: i64,
    /// Database page size in bytes.
    pub page_size: i64,
}

/// Result of a database integrity check.
#[derive(Debug, Clone)]
pub struct IntegrityReport {
    /// True if no corruption detected.
    pub is_clean: bool,
    /// List of error messages (empty if clean).
    pub errors: Vec<String>,
}
