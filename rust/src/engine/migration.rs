use crate::api::types::{MigrationRecord, MigrationReport, VaultMigration};
use anyhow::{anyhow, Context, Result};
use r2d2::PooledConnection;
use r2d2_sqlite::SqliteConnectionManager;
use sha2::{Digest, Sha256};

/// Compute SHA-256 hex digest of a migration's SQL.
pub(crate) fn checksum(sql: &str) -> String {
    let mut hasher = Sha256::new();
    hasher.update(sql.trim().as_bytes());
    hex::encode(hasher.finalize())
}

/// Ensure the `_migrations` tracking table exists.
pub(crate) fn ensure_table(
    conn: &PooledConnection<SqliteConnectionManager>,
) -> Result<()> {
    conn.execute_batch(
        "CREATE TABLE IF NOT EXISTS _migrations (\
            version      INTEGER PRIMARY KEY,\
            name         TEXT NOT NULL,\
            checksum     TEXT NOT NULL,\
            applied_at   INTEGER NOT NULL,\
            duration_ms  INTEGER NOT NULL,\
            applied_by   TEXT\
        )",
    )
    .context("Failed to create _migrations table")?;
    Ok(())
}

/// Read the current max version from `_migrations`, or 0 if empty.
pub(crate) fn current_version(
    conn: &PooledConnection<SqliteConnectionManager>,
) -> Result<u32> {
    let version: Option<u32> = conn
        .query_row(
            "SELECT MAX(version) FROM _migrations",
            [],
            |row| row.get(0),
        )
        .unwrap_or(None);
    Ok(version.unwrap_or(0))
}

/// Get all applied migration records, ordered by version.
pub(crate) fn get_applied(
    conn: &PooledConnection<SqliteConnectionManager>,
) -> Result<Vec<MigrationRecord>> {
    let mut stmt = conn.prepare(
        "SELECT version, name, checksum, applied_at, duration_ms \
         FROM _migrations ORDER BY version ASC",
    )?;
    let rows = stmt.query_map([], |row| {
        Ok(MigrationRecord {
            version: row.get(0)?,
            name: row.get(1)?,
            checksum: row.get(2)?,
            applied_at: row.get(3)?,
            duration_ms: row.get(4)?,
        })
    })?;

    let mut records = Vec::new();
    for row in rows {
        records.push(row?);
    }
    Ok(records)
}

/// Run forward migrations.
///
/// For each migration with version > current:
/// 1. Verify checksum if already applied (detect tampered SQL)
/// 2. Begin transaction
/// 3. Execute `up` SQL
/// 4. Insert migration record with checksum + duration
/// 5. Commit
pub(crate) fn migrate(
    conn: &PooledConnection<SqliteConnectionManager>,
    migrations: &[VaultMigration],
) -> Result<MigrationReport> {
    ensure_table(conn)?;

    let applied_records = get_applied(conn)?;
    let applied_versions: std::collections::HashMap<u32, String> = applied_records
        .iter()
        .map(|r| (r.version, r.checksum.clone()))
        .collect();

    let cur_version = current_version(conn)?;
    let mut report = MigrationReport {
        applied: Vec::new(),
        skipped: Vec::new(),
        current_version: cur_version,
    };

    // Sort migrations by version
    let mut sorted: Vec<&VaultMigration> = migrations.iter().collect();
    sorted.sort_by_key(|m| m.version);

    for migration in sorted {
        if migration.version == 0 {
            return Err(anyhow!(
                "MigrationException: version must be > 0, got 0 for '{}'",
                migration.name
            ));
        }

        let expected_checksum = checksum(&migration.up);

        // If already applied, verify checksum
        if let Some(stored_checksum) = applied_versions.get(&migration.version) {
            if *stored_checksum != expected_checksum {
                return Err(anyhow!(
                    "MigrationChecksumException: migration v{} '{}' has been modified \
                     (stored: {}, computed: {}). Never edit applied migrations.",
                    migration.version,
                    migration.name,
                    stored_checksum,
                    expected_checksum,
                ));
            }
            report.skipped.push(migration.version);
            continue;
        }

        // Apply this migration
        let start = std::time::Instant::now();

        conn.execute_batch("BEGIN IMMEDIATE")
            .with_context(|| {
                format!(
                    "MigrationFailedException: failed to begin transaction for v{} '{}'",
                    migration.version, migration.name
                )
            })?;

        match conn.execute_batch(&migration.up) {
            Ok(()) => {}
            Err(e) => {
                let _ = conn.execute_batch("ROLLBACK");
                return Err(anyhow!(
                    "MigrationFailedException: v{} '{}' failed — rolled back: {}",
                    migration.version,
                    migration.name,
                    e,
                ));
            }
        }

        let duration_ms = start.elapsed().as_millis() as i64;
        let now_ms = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_millis() as i64;

        match conn.execute(
            "INSERT INTO _migrations (version, name, checksum, applied_at, duration_ms) \
             VALUES (?1, ?2, ?3, ?4, ?5)",
            rusqlite::params![
                migration.version,
                migration.name,
                expected_checksum,
                now_ms,
                duration_ms,
            ],
        ) {
            Ok(_) => {}
            Err(e) => {
                let _ = conn.execute_batch("ROLLBACK");
                return Err(anyhow!(
                    "MigrationFailedException: v{} '{}' — failed to record migration: {}",
                    migration.version,
                    migration.name,
                    e,
                ));
            }
        }

        conn.execute_batch("COMMIT").with_context(|| {
            format!(
                "MigrationFailedException: failed to commit v{} '{}'",
                migration.version, migration.name
            )
        })?;

        report.applied.push(migration.version);
        report.current_version = migration.version;
    }

    // Update current_version to actual max in case skipped versions exist
    report.current_version = current_version(conn)?;
    Ok(report)
}

/// Rollback migrations to a target version (exclusive).
///
/// Rolls back all applied migrations with version > target_version,
/// in reverse order. Each rollback executes the `down` SQL in a transaction.
pub(crate) fn rollback_to(
    conn: &PooledConnection<SqliteConnectionManager>,
    target_version: u32,
    migrations: &[VaultMigration],
) -> Result<MigrationReport> {
    ensure_table(conn)?;

    let cur = current_version(conn)?;
    if target_version >= cur {
        return Ok(MigrationReport {
            applied: Vec::new(),
            skipped: Vec::new(),
            current_version: cur,
        });
    }

    // Build a map of version → migration for lookup
    let migration_map: std::collections::HashMap<u32, &VaultMigration> =
        migrations.iter().map(|m| (m.version, m)).collect();

    // Get applied migrations to roll back (version > target, descending)
    let applied = get_applied(conn)?;
    let mut to_rollback: Vec<&MigrationRecord> = applied
        .iter()
        .filter(|r| r.version > target_version)
        .collect();
    to_rollback.sort_by(|a, b| b.version.cmp(&a.version)); // descending

    let mut rolled_back = Vec::new();

    for record in to_rollback {
        let migration = migration_map.get(&record.version).ok_or_else(|| {
            anyhow!(
                "MigrationException: cannot rollback v{} '{}' — migration definition not found",
                record.version,
                record.name,
            )
        })?;

        let down_sql = migration.down.as_ref().ok_or_else(|| {
            anyhow!(
                "MigrationNoRollbackException: v{} '{}' has no `down` SQL — cannot rollback",
                migration.version,
                migration.name,
            )
        })?;

        conn.execute_batch("BEGIN IMMEDIATE")
            .with_context(|| {
                format!(
                    "MigrationFailedException: failed to begin rollback for v{} '{}'",
                    migration.version, migration.name
                )
            })?;

        match conn.execute_batch(down_sql) {
            Ok(()) => {}
            Err(e) => {
                let _ = conn.execute_batch("ROLLBACK");
                return Err(anyhow!(
                    "MigrationFailedException: rollback of v{} '{}' failed: {}",
                    migration.version,
                    migration.name,
                    e,
                ));
            }
        }

        match conn.execute(
            "DELETE FROM _migrations WHERE version = ?1",
            rusqlite::params![record.version],
        ) {
            Ok(_) => {}
            Err(e) => {
                let _ = conn.execute_batch("ROLLBACK");
                return Err(anyhow!(
                    "MigrationFailedException: rollback of v{} '{}' — failed to delete record: {}",
                    migration.version,
                    migration.name,
                    e,
                ));
            }
        }

        conn.execute_batch("COMMIT").with_context(|| {
            format!(
                "MigrationFailedException: failed to commit rollback of v{} '{}'",
                migration.version, migration.name
            )
        })?;

        rolled_back.push(record.version);
    }

    let new_version = current_version(conn)?;
    Ok(MigrationReport {
        applied: rolled_back,
        skipped: Vec::new(),
        current_version: new_version,
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn checksum_deterministic() {
        let sql = "CREATE TABLE foo (id INTEGER PRIMARY KEY)";
        assert_eq!(checksum(sql), checksum(sql));
    }

    #[test]
    fn checksum_trims_whitespace() {
        let sql1 = "  CREATE TABLE foo (id INTEGER)  ";
        let sql2 = "CREATE TABLE foo (id INTEGER)";
        assert_eq!(checksum(sql1), checksum(sql2));
    }

    #[test]
    fn checksum_differs_for_different_sql() {
        let a = checksum("CREATE TABLE a (id INTEGER)");
        let b = checksum("CREATE TABLE b (id INTEGER)");
        assert_ne!(a, b);
    }

    #[test]
    fn checksum_is_64_hex_chars() {
        let cs = checksum("SELECT 1");
        assert_eq!(cs.len(), 64); // SHA-256 = 32 bytes = 64 hex chars
        assert!(cs.chars().all(|c| c.is_ascii_hexdigit()));
    }
}
