use crate::api::types::*;
use anyhow::{anyhow, Context, Result};
use r2d2::PooledConnection;
use r2d2_sqlite::SqliteConnectionManager;

/// Ensure sync tables exist.
pub(crate) fn ensure_tables(conn: &PooledConnection<SqliteConnectionManager>) -> Result<()> {
    conn.execute_batch(
        "CREATE TABLE IF NOT EXISTS _sync_outbox (\
            id TEXT PRIMARY KEY, table_name TEXT NOT NULL, row_id TEXT NOT NULL,\
            operation TEXT NOT NULL, payload TEXT NOT NULL, vector_clock TEXT NOT NULL,\
            created_at INTEGER NOT NULL, synced_at INTEGER, attempts INTEGER DEFAULT 0,\
            next_retry INTEGER NOT NULL, tenant_id TEXT NOT NULL\
        );\
        CREATE INDEX IF NOT EXISTS idx_outbox_pending ON _sync_outbox(tenant_id, synced_at, next_retry) WHERE synced_at IS NULL;\
        CREATE TABLE IF NOT EXISTS _sync_conflicts (\
            id TEXT PRIMARY KEY, table_name TEXT NOT NULL, row_id TEXT NOT NULL,\
            local_data TEXT NOT NULL, remote_data TEXT NOT NULL,\
            local_clock TEXT NOT NULL, remote_clock TEXT NOT NULL,\
            detected_at INTEGER NOT NULL, resolved INTEGER DEFAULT 0,\
            resolution TEXT, resolved_at INTEGER, resolved_by TEXT\
        );\
        CREATE TABLE IF NOT EXISTS _sync_dead_letter (\
            id TEXT PRIMARY KEY, table_name TEXT NOT NULL, row_id TEXT NOT NULL,\
            operation TEXT NOT NULL, payload TEXT NOT NULL, vector_clock TEXT NOT NULL,\
            created_at INTEGER NOT NULL, attempts INTEGER, error_message TEXT,\
            tenant_id TEXT NOT NULL\
        );",
    )
    .context("Failed to create sync tables")?;
    Ok(())
}

/// Add a record to the sync outbox.
pub(crate) fn add_to_outbox(
    conn: &PooledConnection<SqliteConnectionManager>,
    table_name: &str,
    row_id: &str,
    operation: &str,
    payload: &str,
    vector_clock: &VectorClock,
    tenant_id: &str,
) -> Result<String> {
    ensure_tables(conn)?;
    let id = uuid::Uuid::new_v4().to_string();
    let now = crate::engine::write_ops::current_epoch_ms();
    let vc_json = vector_clock.to_json();
    conn.execute(
        "INSERT INTO _sync_outbox (id, table_name, row_id, operation, payload, vector_clock, created_at, next_retry, tenant_id) \
         VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9)",
        rusqlite::params![id, table_name, row_id, operation, payload, vc_json, now, now, tenant_id],
    )?;
    Ok(id)
}

/// Get pending outbox records (not yet synced).
pub(crate) fn get_delta(
    conn: &PooledConnection<SqliteConnectionManager>,
    tenant_id: &str,
    since_seq: i64,
    limit: u32,
) -> Result<SyncDelta> {
    ensure_tables(conn)?;
    let mut stmt = conn.prepare(
        "SELECT id, table_name, row_id, operation, payload, vector_clock, created_at, synced_at, attempts, tenant_id \
         FROM _sync_outbox WHERE tenant_id = ?1 AND synced_at IS NULL AND created_at > ?2 \
         ORDER BY created_at ASC LIMIT ?3",
    )?;
    let rows = stmt.query_map(rusqlite::params![tenant_id, since_seq, limit], |row| {
        Ok(SyncRecord {
            id: row.get(0)?, table_name: row.get(1)?, row_id: row.get(2)?,
            operation: row.get(3)?, payload: row.get(4)?, vector_clock: row.get(5)?,
            created_at: row.get(6)?, synced_at: row.get(7)?, attempts: row.get(8)?,
            tenant_id: row.get(9)?,
        })
    })?;
    let mut records = Vec::new();
    for r in rows { records.push(r?); }
    Ok(SyncDelta { records })
}

/// Mark outbox records as synced.
pub(crate) fn mark_synced(
    conn: &PooledConnection<SqliteConnectionManager>,
    record_ids: &[String],
) -> Result<u32> {
    ensure_tables(conn)?;
    let now = crate::engine::write_ops::current_epoch_ms();
    let mut count = 0u32;
    for id in record_ids {
        let affected = conn.execute(
            "UPDATE _sync_outbox SET synced_at = ?1 WHERE id = ?2",
            rusqlite::params![now, id],
        )?;
        count += affected as u32;
    }
    Ok(count)
}

/// Apply incoming sync records with conflict detection.
pub(crate) fn apply_delta(
    conn: &PooledConnection<SqliteConnectionManager>,
    delta: &SyncDelta,
    resolution: &ConflictResolution,
    tenant_id: &str,
) -> Result<SyncApplyResult> {
    ensure_tables(conn)?;
    let mut applied = 0u32;
    let mut conflicts = 0u32;
    let mut skipped = 0u32;

    for record in &delta.records {
        let remote_clock = VectorClock::from_json(&record.vector_clock);

        // Check if we have a local outbox record for the same row
        let local_record: Option<(String, String)> = conn
            .query_row(
                "SELECT vector_clock, payload FROM _sync_outbox \
                 WHERE table_name = ?1 AND row_id = ?2 AND tenant_id = ?3 AND synced_at IS NULL \
                 ORDER BY created_at DESC LIMIT 1",
                rusqlite::params![record.table_name, record.row_id, tenant_id],
                |row| Ok((row.get(0)?, row.get(1)?)),
            )
            .ok();

        if let Some((local_vc_json, local_payload)) = local_record {
            let local_clock = VectorClock::from_json(&local_vc_json);

            if remote_clock.happens_before(&local_clock) {
                // Local is ahead — skip remote
                skipped += 1;
            } else if local_clock.happens_before(&remote_clock) {
                // Remote is ahead — apply
                applied += 1;
            } else if local_clock.is_concurrent_with(&remote_clock) {
                // Concurrent — conflict!
                match resolution {
                    ConflictResolution::LastWriteWins => {
                        // Compare timestamps: higher created_at wins
                        if record.created_at >= crate::engine::write_ops::current_epoch_ms() {
                            applied += 1; // remote wins
                        } else {
                            skipped += 1; // local wins
                        }
                    }
                    ConflictResolution::LocalWins => {
                        skipped += 1;
                    }
                    ConflictResolution::RemoteWins => {
                        applied += 1;
                    }
                }

                // Always record the conflict for audit trail
                let conflict_id = uuid::Uuid::new_v4().to_string();
                let now = crate::engine::write_ops::current_epoch_ms();
                conn.execute(
                    "INSERT INTO _sync_conflicts (id, table_name, row_id, local_data, remote_data, \
                     local_clock, remote_clock, detected_at, resolved) \
                     VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, 0)",
                    rusqlite::params![
                        conflict_id, record.table_name, record.row_id,
                        local_payload, record.payload,
                        local_vc_json, record.vector_clock,
                        now,
                    ],
                )?;
                conflicts += 1;
            } else {
                // Equal clocks — skip (already in sync)
                skipped += 1;
            }
        } else {
            // No local record — apply directly
            applied += 1;
        }
    }

    Ok(SyncApplyResult { applied, conflicts, skipped })
}

/// Get unresolved sync conflicts.
pub(crate) fn get_pending_conflicts(
    conn: &PooledConnection<SqliteConnectionManager>,
    _tenant_id: &str,
) -> Result<Vec<SyncConflict>> {
    ensure_tables(conn)?;
    let mut stmt = conn.prepare(
        "SELECT id, table_name, row_id, local_data, remote_data, local_clock, remote_clock, detected_at, resolved \
         FROM _sync_conflicts WHERE resolved = 0 \
         ORDER BY detected_at DESC",
    )?;
    let rows = stmt.query_map([], |row| {
        Ok(SyncConflict {
            id: row.get(0)?, table_name: row.get(1)?, row_id: row.get(2)?,
            local_data: row.get(3)?, remote_data: row.get(4)?,
            local_clock: row.get(5)?, remote_clock: row.get(6)?,
            detected_at: row.get(7)?, resolved: row.get::<_, i32>(8)? != 0,
        })
    })?;
    let mut result = Vec::new();
    for r in rows { result.push(r?); }
    Ok(result)
}

/// Resolve a sync conflict.
pub(crate) fn resolve_conflict(
    conn: &PooledConnection<SqliteConnectionManager>,
    conflict_id: &str,
    resolution: &str,
    resolved_by: &str,
) -> Result<()> {
    ensure_tables(conn)?;
    let now = crate::engine::write_ops::current_epoch_ms();
    let affected = conn.execute(
        "UPDATE _sync_conflicts SET resolved = 1, resolution = ?1, resolved_at = ?2, resolved_by = ?3 WHERE id = ?4",
        rusqlite::params![resolution, now, resolved_by, conflict_id],
    )?;
    if affected == 0 {
        return Err(anyhow!("SyncException: conflict '{}' not found", conflict_id));
    }
    Ok(())
}

/// Move failed outbox records to dead-letter queue.
pub(crate) fn move_to_dead_letter(
    conn: &PooledConnection<SqliteConnectionManager>,
    record_id: &str,
    error_message: &str,
) -> Result<()> {
    ensure_tables(conn)?;
    conn.execute(
        "INSERT INTO _sync_dead_letter (id, table_name, row_id, operation, payload, vector_clock, created_at, attempts, error_message, tenant_id) \
         SELECT id, table_name, row_id, operation, payload, vector_clock, created_at, attempts, ?1, tenant_id \
         FROM _sync_outbox WHERE id = ?2",
        rusqlite::params![error_message, record_id],
    )?;
    conn.execute("DELETE FROM _sync_outbox WHERE id = ?1", rusqlite::params![record_id])?;
    Ok(())
}

/// Increment retry attempt on an outbox record with exponential backoff.
pub(crate) fn increment_retry(
    conn: &PooledConnection<SqliteConnectionManager>,
    record_id: &str,
    max_attempts: i32,
    error_message: &str,
) -> Result<bool> {
    ensure_tables(conn)?;
    let record: (i32,) = conn.query_row(
        "SELECT attempts FROM _sync_outbox WHERE id = ?1",
        rusqlite::params![record_id],
        |row| Ok((row.get(0)?,)),
    )?;
    let attempts = record.0 + 1;

    if attempts >= max_attempts {
        move_to_dead_letter(conn, record_id, error_message)?;
        return Ok(false); // moved to dead-letter
    }

    // Exponential backoff: 1s, 2s, 4s, 8s, 16s, 32s, 60s max
    let base_delay_ms = 1000i64;
    let delay = (base_delay_ms * 2i64.pow(attempts as u32)).min(60_000);
    let now = crate::engine::write_ops::current_epoch_ms();
    let next_retry = now + delay;

    conn.execute(
        "UPDATE _sync_outbox SET attempts = ?1, next_retry = ?2 WHERE id = ?3",
        rusqlite::params![attempts, next_retry, record_id],
    )?;
    Ok(true) // still in outbox
}
