use crate::api::types::{AuditEntry, AuditIntegrityReport};
use anyhow::{anyhow, Context, Result};
use hmac::{Hmac, Mac};
use r2d2::PooledConnection;
use r2d2_sqlite::SqliteConnectionManager;
use sha2::Sha256;

type HmacSha256 = Hmac<Sha256>;

/// Ensure the `_audit_log` table and indexes exist.
pub(crate) fn ensure_table(
    conn: &PooledConnection<SqliteConnectionManager>,
) -> Result<()> {
    conn.execute_batch(
        "CREATE TABLE IF NOT EXISTS _audit_log (\
            id              TEXT PRIMARY KEY,\
            table_name      TEXT NOT NULL,\
            row_id          TEXT NOT NULL,\
            operation       TEXT NOT NULL,\
            actor_id        TEXT NOT NULL,\
            tenant_id       TEXT NOT NULL,\
            before_json     TEXT,\
            after_json      TEXT,\
            changed_fields  TEXT,\
            timestamp       INTEGER NOT NULL,\
            checksum        TEXT NOT NULL\
        );\
        CREATE INDEX IF NOT EXISTS idx_audit_row    ON _audit_log(table_name, row_id, timestamp DESC);\
        CREATE INDEX IF NOT EXISTS idx_audit_actor  ON _audit_log(actor_id, timestamp DESC);\
        CREATE INDEX IF NOT EXISTS idx_audit_tenant ON _audit_log(tenant_id, timestamp DESC);\
        CREATE INDEX IF NOT EXISTS idx_audit_ts     ON _audit_log(timestamp DESC);",
    )
    .context("Failed to create _audit_log table")?;
    Ok(())
}

/// Compute HMAC-SHA256 checksum for an audit entry.
pub(crate) fn compute_checksum(
    table_name: &str,
    row_id: &str,
    operation: &str,
    actor_id: &str,
    tenant_id: &str,
    before_json: Option<&str>,
    after_json: Option<&str>,
    hmac_key: &[u8],
) -> Result<String> {
    let payload = format!(
        "{}:{}:{}:{}:{}:{}:{}",
        table_name, row_id, operation, actor_id, tenant_id,
        before_json.unwrap_or(""),
        after_json.unwrap_or(""),
    );
    let mut mac = HmacSha256::new_from_slice(hmac_key)
        .map_err(|e| anyhow!("EncryptionException: invalid HMAC key: {}", e))?;
    mac.update(payload.as_bytes());
    Ok(hex::encode(mac.finalize().into_bytes()))
}

/// Record a single audit entry.
pub(crate) fn record(
    conn: &PooledConnection<SqliteConnectionManager>,
    table_name: &str,
    row_id: &str,
    operation: &str,
    actor_id: &str,
    tenant_id: &str,
    before_json: Option<&str>,
    after_json: Option<&str>,
    changed_fields: Option<&str>,
    hmac_key: &[u8],
) -> Result<String> {
    ensure_table(conn)?;
    let id = uuid::Uuid::new_v4().to_string();
    let timestamp = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap()
        .as_millis() as i64;
    let checksum = compute_checksum(
        table_name, row_id, operation, actor_id, tenant_id,
        before_json, after_json, hmac_key,
    )?;
    conn.execute(
        "INSERT INTO _audit_log (id, table_name, row_id, operation, actor_id, \
         tenant_id, before_json, after_json, changed_fields, timestamp, checksum) \
         VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10, ?11)",
        rusqlite::params![
            id, table_name, row_id, operation, actor_id, tenant_id,
            before_json, after_json, changed_fields, timestamp, checksum,
        ],
    )
    .context("Failed to insert audit entry")?;
    Ok(id)
}

/// Get audit history for a specific row.
pub(crate) fn get_history(
    conn: &PooledConnection<SqliteConnectionManager>,
    table_name: &str, row_id: &str, tenant_id: &str, limit: u32,
) -> Result<Vec<AuditEntry>> {
    ensure_table(conn)?;
    let mut stmt = conn.prepare(
        "SELECT id, table_name, row_id, operation, actor_id, tenant_id, \
         before_json, after_json, changed_fields, timestamp, checksum \
         FROM _audit_log WHERE table_name = ?1 AND row_id = ?2 AND tenant_id = ?3 \
         ORDER BY timestamp DESC LIMIT ?4",
    )?;
    read_entries(&mut stmt, rusqlite::params![table_name, row_id, tenant_id, limit])
}

/// Get audit history for a specific actor.
pub(crate) fn get_actor_history(
    conn: &PooledConnection<SqliteConnectionManager>,
    actor_id: &str, tenant_id: &str,
    from: Option<i64>, to: Option<i64>, limit: u32,
) -> Result<Vec<AuditEntry>> {
    ensure_table(conn)?;
    let mut stmt = conn.prepare(
        "SELECT id, table_name, row_id, operation, actor_id, tenant_id, \
         before_json, after_json, changed_fields, timestamp, checksum \
         FROM _audit_log WHERE actor_id = ?1 AND tenant_id = ?2 \
         AND timestamp >= ?3 AND timestamp <= ?4 ORDER BY timestamp DESC LIMIT ?5",
    )?;
    read_entries(&mut stmt, rusqlite::params![
        actor_id, tenant_id, from.unwrap_or(0), to.unwrap_or(i64::MAX), limit
    ])
}

/// Get audit history for an entire table.
pub(crate) fn get_table_history(
    conn: &PooledConnection<SqliteConnectionManager>,
    table_name: &str, tenant_id: &str,
    from: Option<i64>, to: Option<i64>, limit: u32,
) -> Result<Vec<AuditEntry>> {
    ensure_table(conn)?;
    let mut stmt = conn.prepare(
        "SELECT id, table_name, row_id, operation, actor_id, tenant_id, \
         before_json, after_json, changed_fields, timestamp, checksum \
         FROM _audit_log WHERE table_name = ?1 AND tenant_id = ?2 \
         AND timestamp >= ?3 AND timestamp <= ?4 ORDER BY timestamp DESC LIMIT ?5",
    )?;
    read_entries(&mut stmt, rusqlite::params![
        table_name, tenant_id, from.unwrap_or(0), to.unwrap_or(i64::MAX), limit
    ])
}

/// Verify integrity of audit log entries by recomputing HMAC checksums.
pub(crate) fn verify_integrity(
    conn: &PooledConnection<SqliteConnectionManager>,
    tenant_id: &str, hmac_key: &[u8],
    from: Option<i64>, to: Option<i64>,
) -> Result<AuditIntegrityReport> {
    ensure_table(conn)?;
    let mut stmt = conn.prepare(
        "SELECT id, table_name, row_id, operation, actor_id, tenant_id, \
         before_json, after_json, changed_fields, timestamp, checksum \
         FROM _audit_log WHERE tenant_id = ?1 AND timestamp >= ?2 AND timestamp <= ?3 \
         ORDER BY timestamp ASC",
    )?;
    let entries = read_entries(&mut stmt, rusqlite::params![
        tenant_id, from.unwrap_or(0), to.unwrap_or(i64::MAX)
    ])?;
    let mut tampered_ids = Vec::new();
    for entry in &entries {
        let expected = compute_checksum(
            &entry.table_name, &entry.row_id, &entry.operation,
            &entry.actor_id, &entry.tenant_id,
            entry.before_json.as_deref(), entry.after_json.as_deref(),
            hmac_key,
        )?;
        if expected != entry.checksum {
            tampered_ids.push(entry.id.clone());
        }
    }
    Ok(AuditIntegrityReport {
        total_checked: entries.len() as u64,
        is_clean: tampered_ids.is_empty(),
        tampered_ids,
    })
}

fn read_entries(
    stmt: &mut rusqlite::Statement,
    params: impl rusqlite::Params,
) -> Result<Vec<AuditEntry>> {
    let rows = stmt.query_map(params, |row| {
        Ok(AuditEntry {
            id: row.get(0)?, table_name: row.get(1)?, row_id: row.get(2)?,
            operation: row.get(3)?, actor_id: row.get(4)?, tenant_id: row.get(5)?,
            before_json: row.get(6)?, after_json: row.get(7)?,
            changed_fields: row.get(8)?, timestamp: row.get(9)?,
            checksum: row.get(10)?,
        })
    })?;
    let mut entries = Vec::new();
    for row in rows { entries.push(row?); }
    Ok(entries)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn checksum_deterministic() {
        let key = vec![0x42u8; 32];
        let c1 = compute_checksum("t", "r", "INSERT", "a", "t1", None, Some("{}"), &key).unwrap();
        let c2 = compute_checksum("t", "r", "INSERT", "a", "t1", None, Some("{}"), &key).unwrap();
        assert_eq!(c1, c2);
    }

    #[test]
    fn checksum_changes_with_different_data() {
        let key = vec![0x42u8; 32];
        let c1 = compute_checksum("t", "r1", "INSERT", "a", "t1", None, None, &key).unwrap();
        let c2 = compute_checksum("t", "r2", "INSERT", "a", "t1", None, None, &key).unwrap();
        assert_ne!(c1, c2);
    }

    #[test]
    fn checksum_changes_with_different_key() {
        let c1 = compute_checksum("t", "r", "INSERT", "a", "t1", None, None, &[1u8; 32]).unwrap();
        let c2 = compute_checksum("t", "r", "INSERT", "a", "t1", None, None, &[2u8; 32]).unwrap();
        assert_ne!(c1, c2);
    }

    #[test]
    fn checksum_is_64_hex_chars() {
        let cs = compute_checksum("t", "r", "INSERT", "a", "t1", None, None, &[0x42u8; 32]).unwrap();
        assert_eq!(cs.len(), 64);
        assert!(cs.chars().all(|c| c.is_ascii_hexdigit()));
    }
}
