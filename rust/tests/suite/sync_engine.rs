use crate::common::*;
use iron_vault_core::api::types::*;

fn make_clock(pairs: &[(&str, u64)]) -> VectorClock {
    VectorClock {
        clocks: pairs.iter().map(|(k, v)| (k.to_string(), *v)).collect(),
    }
}

// ─── Outbox ──────────────────────────────────────────────────────────

#[test]
fn add_to_outbox_and_get_delta() {
    let dir = tempfile::TempDir::new().unwrap();
    let db = open_test_db(&dir);

    let vc = make_clock(&[("node_a", 1)]);
    let id = db
        .sync_add_to_outbox(
            "users".into(),
            "row_1".into(),
            "INSERT".into(),
            "{\"name\":\"Alice\"}".into(),
            vc,
        )
        .unwrap();
    assert!(!id.is_empty());

    let delta = db.sync_get_delta(0, 100).unwrap();
    assert_eq!(delta.records.len(), 1);
    assert_eq!(delta.records[0].table_name, "users");
    assert_eq!(delta.records[0].row_id, "row_1");
    assert_eq!(delta.records[0].operation, "INSERT");
    assert!(delta.records[0].synced_at.is_none());
}

#[test]
fn mark_synced_updates_outbox() {
    let dir = tempfile::TempDir::new().unwrap();
    let db = open_test_db(&dir);

    let vc = make_clock(&[("a", 1)]);
    let id = db
        .sync_add_to_outbox(
            "users".into(),
            "r1".into(),
            "INSERT".into(),
            "{}".into(),
            vc,
        )
        .unwrap();

    let marked = db.sync_mark_synced(vec![id]).unwrap();
    assert_eq!(marked, 1);

    // After marking synced, get_delta should return nothing
    let delta = db.sync_get_delta(0, 100).unwrap();
    assert!(delta.records.is_empty());
}

#[test]
fn get_delta_respects_since_seq() {
    let dir = tempfile::TempDir::new().unwrap();
    let db = open_test_db(&dir);

    let vc = make_clock(&[("a", 1)]);
    db.sync_add_to_outbox(
        "t".into(),
        "r1".into(),
        "INSERT".into(),
        "{}".into(),
        vc.clone(),
    )
    .unwrap();

    // Sleep briefly to ensure different timestamps
    std::thread::sleep(std::time::Duration::from_millis(10));
    let mid_time = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap()
        .as_millis() as i64;

    std::thread::sleep(std::time::Duration::from_millis(10));
    db.sync_add_to_outbox("t".into(), "r2".into(), "UPDATE".into(), "{}".into(), vc)
        .unwrap();

    // Only records after mid_time
    let delta = db.sync_get_delta(mid_time, 100).unwrap();
    assert_eq!(delta.records.len(), 1);
    assert_eq!(delta.records[0].row_id, "r2");
}

#[test]
fn get_delta_respects_limit() {
    let dir = tempfile::TempDir::new().unwrap();
    let db = open_test_db(&dir);

    let vc = make_clock(&[("a", 1)]);
    for i in 0..10 {
        db.sync_add_to_outbox(
            "t".into(),
            format!("r{}", i),
            "INSERT".into(),
            "{}".into(),
            vc.clone(),
        )
        .unwrap();
    }

    let delta = db.sync_get_delta(0, 3).unwrap();
    assert_eq!(delta.records.len(), 3);
}

// ─── Apply Delta: No Conflict ────────────────────────────────────────

#[test]
fn apply_delta_no_local_record_applies() {
    let dir = tempfile::TempDir::new().unwrap();
    let db = open_test_db(&dir);

    let delta = SyncDelta {
        records: vec![SyncRecord {
            id: "remote_1".into(),
            table_name: "users".into(),
            row_id: "r1".into(),
            operation: "INSERT".into(),
            payload: "{\"name\":\"Bob\"}".into(),
            vector_clock: make_clock(&[("remote", 1)]).to_json(),
            created_at: 1000,
            synced_at: None,
            attempts: 0,
            tenant_id: "tenant_test".into(),
        }],
    };

    let result = db
        .sync_apply_delta(delta, ConflictResolution::LastWriteWins)
        .unwrap();
    assert_eq!(result.applied, 1);
    assert_eq!(result.conflicts, 0);
    assert_eq!(result.skipped, 0);
}

// ─── Apply Delta: Remote Ahead ───────────────────────────────────────

#[test]
fn apply_delta_remote_ahead_applies() {
    let dir = tempfile::TempDir::new().unwrap();
    let db = open_test_db(&dir);

    // Local has clock (a:1)
    let local_vc = make_clock(&[("a", 1)]);
    db.sync_add_to_outbox(
        "users".into(),
        "r1".into(),
        "INSERT".into(),
        "{\"v\":1}".into(),
        local_vc,
    )
    .unwrap();

    // Remote has clock (a:2) — ahead
    let delta = SyncDelta {
        records: vec![SyncRecord {
            id: "remote_1".into(),
            table_name: "users".into(),
            row_id: "r1".into(),
            operation: "UPDATE".into(),
            payload: "{\"v\":2}".into(),
            vector_clock: make_clock(&[("a", 2)]).to_json(),
            created_at: 2000,
            synced_at: None,
            attempts: 0,
            tenant_id: "tenant_test".into(),
        }],
    };

    let result = db
        .sync_apply_delta(delta, ConflictResolution::LastWriteWins)
        .unwrap();
    assert_eq!(result.applied, 1);
    assert_eq!(result.conflicts, 0);
}

// ─── Apply Delta: Concurrent → Conflict ──────────────────────────────

#[test]
fn apply_delta_concurrent_creates_conflict() {
    let dir = tempfile::TempDir::new().unwrap();
    let db = open_test_db(&dir);

    // Local has clock (a:2, b:0)
    let local_vc = make_clock(&[("a", 2)]);
    db.sync_add_to_outbox(
        "users".into(),
        "r1".into(),
        "UPDATE".into(),
        "{\"local\":true}".into(),
        local_vc,
    )
    .unwrap();

    // Remote has clock (a:0, b:2) — concurrent!
    let delta = SyncDelta {
        records: vec![SyncRecord {
            id: "remote_1".into(),
            table_name: "users".into(),
            row_id: "r1".into(),
            operation: "UPDATE".into(),
            payload: "{\"remote\":true}".into(),
            vector_clock: make_clock(&[("b", 2)]).to_json(),
            created_at: 2000,
            synced_at: None,
            attempts: 0,
            tenant_id: "tenant_test".into(),
        }],
    };

    let result = db
        .sync_apply_delta(delta, ConflictResolution::LocalWins)
        .unwrap();
    assert_eq!(result.conflicts, 1);
    assert_eq!(result.skipped, 1); // LocalWins → skip remote

    // Conflict should be recorded
    let conflicts = db.sync_get_conflicts().unwrap();
    assert_eq!(conflicts.len(), 1);
    assert_eq!(conflicts[0].row_id, "r1");
    assert!(!conflicts[0].resolved);
}

#[test]
fn apply_delta_concurrent_remote_wins() {
    let dir = tempfile::TempDir::new().unwrap();
    let db = open_test_db(&dir);

    let local_vc = make_clock(&[("a", 2)]);
    db.sync_add_to_outbox(
        "users".into(),
        "r1".into(),
        "UPDATE".into(),
        "{}".into(),
        local_vc,
    )
    .unwrap();

    let delta = SyncDelta {
        records: vec![SyncRecord {
            id: "r1".into(),
            table_name: "users".into(),
            row_id: "r1".into(),
            operation: "UPDATE".into(),
            payload: "{\"remote\":true}".into(),
            vector_clock: make_clock(&[("b", 2)]).to_json(),
            created_at: 2000,
            synced_at: None,
            attempts: 0,
            tenant_id: "tenant_test".into(),
        }],
    };

    let result = db
        .sync_apply_delta(delta, ConflictResolution::RemoteWins)
        .unwrap();
    assert_eq!(result.applied, 1); // remote applied
    assert_eq!(result.conflicts, 1); // conflict recorded
}

// ─── Conflict Management ─────────────────────────────────────────────

#[test]
fn resolve_conflict() {
    let dir = tempfile::TempDir::new().unwrap();
    let db = open_test_db(&dir);

    // Create a conflict
    let local_vc = make_clock(&[("a", 1)]);
    db.sync_add_to_outbox(
        "users".into(),
        "r1".into(),
        "UPDATE".into(),
        "{}".into(),
        local_vc,
    )
    .unwrap();
    let delta = SyncDelta {
        records: vec![SyncRecord {
            id: "r1".into(),
            table_name: "users".into(),
            row_id: "r1".into(),
            operation: "UPDATE".into(),
            payload: "{}".into(),
            vector_clock: make_clock(&[("b", 1)]).to_json(),
            created_at: 1000,
            synced_at: None,
            attempts: 0,
            tenant_id: "tenant_test".into(),
        }],
    };
    db.sync_apply_delta(delta, ConflictResolution::LocalWins)
        .unwrap();

    let conflicts = db.sync_get_conflicts().unwrap();
    assert_eq!(conflicts.len(), 1);

    db.set_actor("admin".into()).unwrap();
    db.sync_resolve_conflict(conflicts[0].id.clone(), "used_local".into())
        .unwrap();

    let remaining = db.sync_get_conflicts().unwrap();
    assert!(remaining.is_empty());
}

// ─── Retry + Dead Letter ─────────────────────────────────────────────

#[test]
fn increment_retry_with_backoff() {
    let dir = tempfile::TempDir::new().unwrap();
    let db = open_test_db(&dir);

    let vc = make_clock(&[("a", 1)]);
    let id = db
        .sync_add_to_outbox("t".into(), "r1".into(), "INSERT".into(), "{}".into(), vc)
        .unwrap();

    // Retry once — should stay in outbox
    let still_active = db
        .sync_increment_retry(id.clone(), 50, "timeout".into())
        .unwrap();
    assert!(still_active);

    // Record is still in outbox (attempts < max)
}

#[test]
fn dead_letter_after_max_attempts() {
    let dir = tempfile::TempDir::new().unwrap();
    let db = open_test_db(&dir);

    let vc = make_clock(&[("a", 1)]);
    let id = db
        .sync_add_to_outbox("t".into(), "r1".into(), "INSERT".into(), "{}".into(), vc)
        .unwrap();

    // Set max_attempts = 1, so first increment moves to dead-letter
    let still_active = db
        .sync_increment_retry(id.clone(), 1, "permanent failure".into())
        .unwrap();
    assert!(!still_active, "Should have moved to dead-letter");

    // Outbox should be empty
    let delta = db.sync_get_delta(0, 100).unwrap();
    assert!(delta.records.is_empty());

    // Dead-letter should have the record
    let dl_count: i64 = db
        .query_raw(
            "SELECT count(*) as c FROM _sync_dead_letter WHERE id = ?1".into(),
            vec![SqlValue::Text(id)],
        )
        .unwrap()[0]
        .get("c")
        .and_then(|v| match v {
            SqlValue::Integer(i) => Some(*i),
            _ => None,
        })
        .unwrap_or(0);
    assert_eq!(dl_count, 1);
}

// ─── Edge Cases ──────────────────────────────────────────────────────

#[test]
fn sync_ops_fail_after_close() {
    let dir = tempfile::TempDir::new().unwrap();
    let mut db = open_test_db(&dir);
    db.close().unwrap();

    let vc = make_clock(&[("a", 1)]);
    assert!(db
        .sync_add_to_outbox("t".into(), "r".into(), "I".into(), "{}".into(), vc)
        .is_err());
    assert!(db.sync_get_delta(0, 10).is_err());
    assert!(db.sync_mark_synced(vec![]).is_err());
    assert!(db
        .sync_apply_delta(SyncDelta { records: vec![] }, ConflictResolution::LocalWins)
        .is_err());
    assert!(db.sync_get_conflicts().is_err());
    assert!(db.sync_resolve_conflict("x".into(), "y".into()).is_err());
}

#[test]
fn empty_delta_apply_is_noop() {
    let dir = tempfile::TempDir::new().unwrap();
    let db = open_test_db(&dir);

    let result = db
        .sync_apply_delta(
            SyncDelta { records: vec![] },
            ConflictResolution::LastWriteWins,
        )
        .unwrap();
    assert_eq!(result.applied, 0);
    assert_eq!(result.conflicts, 0);
    assert_eq!(result.skipped, 0);
}

#[test]
fn mark_synced_nonexistent_id_returns_zero() {
    let dir = tempfile::TempDir::new().unwrap();
    let db = open_test_db(&dir);

    let count = db.sync_mark_synced(vec!["nonexistent".into()]).unwrap();
    assert_eq!(count, 0);
}

#[test]
fn resolve_nonexistent_conflict_errors() {
    let dir = tempfile::TempDir::new().unwrap();
    let db = open_test_db(&dir);

    let result = db.sync_resolve_conflict("nonexistent".into(), "x".into());
    assert!(result.is_err());
}

#[test]
fn vector_clock_serialized_in_outbox() {
    let dir = tempfile::TempDir::new().unwrap();
    let db = open_test_db(&dir);

    let vc = make_clock(&[("node_a", 5), ("node_b", 3)]);
    db.sync_add_to_outbox("t".into(), "r1".into(), "INSERT".into(), "{}".into(), vc)
        .unwrap();

    let delta = db.sync_get_delta(0, 100).unwrap();
    let stored_vc = &delta.records[0].vector_clock;
    assert!(stored_vc.contains("node_a"));
    assert!(stored_vc.contains("node_b"));
    assert!(stored_vc.contains('5'));
    assert!(stored_vc.contains('3'));
}
