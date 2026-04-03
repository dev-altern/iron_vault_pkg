use crate::common::*;
use iron_vault_core::api::types::*;

fn m(version: u32, name: &str, up: &str, down: Option<&str>) -> VaultMigration {
    VaultMigration {
        version,
        name: name.into(),
        up: up.into(),
        down: down.map(|s| s.into()),
    }
}

fn standard_migrations() -> Vec<VaultMigration> {
    vec![
        m(
            1,
            "create_items",
            "CREATE TABLE items (id TEXT PRIMARY KEY, name TEXT NOT NULL, \
             tenant_id TEXT NOT NULL, created_at INTEGER NOT NULL, \
             updated_at INTEGER NOT NULL, deleted_at INTEGER)",
            Some("DROP TABLE items"),
        ),
        m(
            2,
            "add_items_category",
            "ALTER TABLE items ADD COLUMN category TEXT DEFAULT 'general'",
            Some("ALTER TABLE items DROP COLUMN category"),
        ),
        m(
            3,
            "create_tags",
            "CREATE TABLE tags (id TEXT PRIMARY KEY, label TEXT NOT NULL, \
             tenant_id TEXT NOT NULL, created_at INTEGER NOT NULL, \
             updated_at INTEGER NOT NULL, deleted_at INTEGER)",
            Some("DROP TABLE tags"),
        ),
        m(
            4,
            "add_items_priority",
            "ALTER TABLE items ADD COLUMN priority INTEGER DEFAULT 0",
            Some("ALTER TABLE items DROP COLUMN priority"),
        ),
        m(
            5,
            "create_items_index",
            "CREATE INDEX idx_items_tenant ON items(tenant_id, created_at DESC)",
            Some("DROP INDEX idx_items_tenant"),
        ),
    ]
}

// ─── Forward Migration ───────────────────────────────────────────────

#[test]
fn apply_5_migrations_in_sequence() {
    let dir = tempfile::TempDir::new().unwrap();
    let db = open_test_db(&dir);

    let report = db.migrate(standard_migrations()).unwrap();
    assert_eq!(report.applied, vec![1, 2, 3, 4, 5]);
    assert!(report.skipped.is_empty());
    assert_eq!(report.current_version, 5);

    // Verify schemas exist by inserting data
    let mut data = std::collections::HashMap::new();
    data.insert("name".into(), SqlValue::Text("Widget".into()));
    data.insert("category".into(), SqlValue::Text("hardware".into()));
    data.insert("priority".into(), SqlValue::Integer(3));
    db.query_insert("items".into(), data).unwrap();

    let mut tag_data = std::collections::HashMap::new();
    tag_data.insert("label".into(), SqlValue::Text("urgent".into()));
    db.query_insert("tags".into(), tag_data).unwrap();
}

#[test]
fn idempotent_running_same_migrations_twice() {
    let dir = tempfile::TempDir::new().unwrap();
    let db = open_test_db(&dir);

    let r1 = db.migrate(standard_migrations()).unwrap();
    assert_eq!(r1.applied.len(), 5);

    let r2 = db.migrate(standard_migrations()).unwrap();
    assert!(r2.applied.is_empty(), "Second run should apply nothing");
    assert_eq!(r2.skipped, vec![1, 2, 3, 4, 5]);
    assert_eq!(r2.current_version, 5);
}

#[test]
fn incremental_migration_applies_only_new() {
    let dir = tempfile::TempDir::new().unwrap();
    let db = open_test_db(&dir);

    // Apply first 2
    let r1 = db.migrate(standard_migrations()[..2].to_vec()).unwrap();
    assert_eq!(r1.applied, vec![1, 2]);
    assert_eq!(r1.current_version, 2);

    // Apply all 5 — only 3,4,5 should be new
    let r2 = db.migrate(standard_migrations()).unwrap();
    assert_eq!(r2.applied, vec![3, 4, 5]);
    assert_eq!(r2.skipped, vec![1, 2]);
    assert_eq!(r2.current_version, 5);
}

#[test]
fn migrations_applied_in_version_order_regardless_of_input_order() {
    let dir = tempfile::TempDir::new().unwrap();
    let db = open_test_db(&dir);

    // Provide in reverse order
    let mut reversed = standard_migrations();
    reversed.reverse();
    let report = db.migrate(reversed).unwrap();
    assert_eq!(report.applied, vec![1, 2, 3, 4, 5]);
}

// ─── Checksum Protection ─────────────────────────────────────────────

#[test]
fn checksum_mismatch_throws() {
    let dir = tempfile::TempDir::new().unwrap();
    let db = open_test_db(&dir);

    // Apply v1
    db.migrate(vec![m(
        1,
        "create_items",
        "CREATE TABLE items (id TEXT PRIMARY KEY)",
        None,
    )])
    .unwrap();

    // Try to apply with modified SQL — checksum should mismatch
    let result = db.migrate(vec![m(
        1,
        "create_items",
        "CREATE TABLE items (id TEXT PRIMARY KEY, MODIFIED)",
        None,
    )]);
    assert!(result.is_err());
    let msg = result.unwrap_err().to_string();
    assert!(
        msg.contains("MigrationChecksumException"),
        "Expected checksum error, got: {}",
        msg
    );
}

#[test]
fn checksum_mismatch_blocks_subsequent_migrations() {
    let dir = tempfile::TempDir::new().unwrap();
    let db = open_test_db(&dir);

    db.migrate(vec![m(1, "v1", "CREATE TABLE t1 (id TEXT PRIMARY KEY)", None)])
        .unwrap();

    // v1 tampered + v2 new — should fail on v1 before reaching v2
    let result = db.migrate(vec![
        m(1, "v1", "CREATE TABLE t1_MODIFIED (id TEXT PRIMARY KEY)", None),
        m(2, "v2", "CREATE TABLE t2 (id TEXT PRIMARY KEY)", None),
    ]);
    assert!(result.is_err());

    // v2 should NOT have been applied
    let records = db.get_migrations().unwrap();
    assert_eq!(records.len(), 1);
    assert_eq!(records[0].version, 1);
}

// ─── SQL Error Handling ──────────────────────────────────────────────

#[test]
fn sql_error_rolls_back_failing_migration() {
    let dir = tempfile::TempDir::new().unwrap();
    let db = open_test_db(&dir);

    let result = db.migrate(vec![m(
        1,
        "bad_sql",
        "THIS IS NOT VALID SQL",
        None,
    )]);
    assert!(result.is_err());
    let msg = result.unwrap_err().to_string();
    assert!(msg.contains("MigrationFailedException"), "got: {}", msg);

    // No migrations should be recorded
    let records = db.get_migrations().unwrap();
    assert!(records.is_empty());
}

#[test]
fn partial_failure_preserves_earlier_migrations() {
    let dir = tempfile::TempDir::new().unwrap();
    let db = open_test_db(&dir);

    let result = db.migrate(vec![
        m(1, "good", "CREATE TABLE good_table (id TEXT PRIMARY KEY)", None),
        m(2, "bad", "THIS IS INVALID SQL", None),
    ]);
    assert!(result.is_err());

    // v1 should be applied, v2 should not
    let records = db.get_migrations().unwrap();
    assert_eq!(records.len(), 1);
    assert_eq!(records[0].version, 1);

    // good_table should exist
    let rows = db
        .query_raw("SELECT count(*) as c FROM good_table".into(), vec![])
        .unwrap();
    assert!(matches!(rows[0].get("c"), Some(SqlValue::Integer(0))));
}

// ─── Rollback ────────────────────────────────────────────────────────

#[test]
fn rollback_reverses_schema_changes() {
    let dir = tempfile::TempDir::new().unwrap();
    let db = open_test_db(&dir);

    db.migrate(standard_migrations()).unwrap();

    // Rollback to v2 (removes v5, v4, v3)
    let report = db
        .rollback_to(2, standard_migrations())
        .unwrap();
    assert_eq!(report.applied, vec![5, 4, 3]); // rolled back in reverse
    assert_eq!(report.current_version, 2);

    // tags table should be gone (created in v3)
    let result = db.query_raw("SELECT * FROM tags".into(), vec![]);
    assert!(result.is_err(), "tags table should not exist after rollback");

    // items should exist but without priority (v4) or index (v5)
    let result = db.query_raw("SELECT id, name, category FROM items".into(), vec![]);
    assert!(result.is_ok(), "items table should still exist");
}

#[test]
fn rollback_to_zero_removes_all() {
    let dir = tempfile::TempDir::new().unwrap();
    let db = open_test_db(&dir);

    db.migrate(standard_migrations()).unwrap();

    let report = db.rollback_to(0, standard_migrations()).unwrap();
    assert_eq!(report.applied.len(), 5);
    assert_eq!(report.current_version, 0);

    let records = db.get_migrations().unwrap();
    assert!(records.is_empty());
}

#[test]
fn rollback_to_current_version_is_noop() {
    let dir = tempfile::TempDir::new().unwrap();
    let db = open_test_db(&dir);

    db.migrate(standard_migrations()).unwrap();

    let report = db.rollback_to(5, standard_migrations()).unwrap();
    assert!(report.applied.is_empty());
    assert_eq!(report.current_version, 5);
}

#[test]
fn rollback_to_higher_version_is_noop() {
    let dir = tempfile::TempDir::new().unwrap();
    let db = open_test_db(&dir);

    db.migrate(standard_migrations()).unwrap();

    let report = db.rollback_to(99, standard_migrations()).unwrap();
    assert!(report.applied.is_empty());
    assert_eq!(report.current_version, 5);
}

#[test]
fn rollback_without_down_sql_errors() {
    let dir = tempfile::TempDir::new().unwrap();
    let db = open_test_db(&dir);

    db.migrate(vec![m(
        1,
        "irreversible",
        "CREATE TABLE perm (id TEXT PRIMARY KEY)",
        None, // no down SQL
    )])
    .unwrap();

    let result = db.rollback_to(0, vec![m(
        1,
        "irreversible",
        "CREATE TABLE perm (id TEXT PRIMARY KEY)",
        None,
    )]);
    assert!(result.is_err());
    let msg = result.unwrap_err().to_string();
    assert!(
        msg.contains("MigrationNoRollbackException"),
        "Expected no-rollback error, got: {}",
        msg
    );
}

#[test]
fn rollback_missing_migration_definition_errors() {
    let dir = tempfile::TempDir::new().unwrap();
    let db = open_test_db(&dir);

    db.migrate(standard_migrations()).unwrap();

    // Try to rollback but only provide definitions for v1-v3 (missing v4, v5)
    let result = db.rollback_to(0, standard_migrations()[..3].to_vec());
    assert!(result.is_err(), "Should fail — missing migration definitions for v4/v5");
}

// ─── Duration Tracking ───────────────────────────────────────────────

#[test]
fn duration_tracked_per_migration() {
    let dir = tempfile::TempDir::new().unwrap();
    let db = open_test_db(&dir);

    db.migrate(standard_migrations()).unwrap();

    let records = db.get_migrations().unwrap();
    assert_eq!(records.len(), 5);
    for record in &records {
        assert!(record.duration_ms >= 0, "Duration should be non-negative");
        assert!(record.applied_at > 0, "applied_at should be set");
    }
}

// ─── Migration Records ──────────────────────────────────────────────

#[test]
fn get_migrations_returns_all_records() {
    let dir = tempfile::TempDir::new().unwrap();
    let db = open_test_db(&dir);

    db.migrate(standard_migrations()).unwrap();

    let records = db.get_migrations().unwrap();
    assert_eq!(records.len(), 5);
    assert_eq!(records[0].version, 1);
    assert_eq!(records[0].name, "create_items");
    assert_eq!(records[4].version, 5);
    assert!(!records[0].checksum.is_empty());
}

#[test]
fn get_migrations_empty_before_any_applied() {
    let dir = tempfile::TempDir::new().unwrap();
    let db = open_test_db(&dir);

    let records = db.get_migrations().unwrap();
    assert!(records.is_empty());
}

// ─── Stats Integration ──────────────────────────────────────────────

#[test]
fn stats_reports_migration_version() {
    let dir = tempfile::TempDir::new().unwrap();
    let db = open_test_db(&dir);

    assert_eq!(db.stats().unwrap().migration_version, 0);

    db.migrate(standard_migrations()[..3].to_vec()).unwrap();
    assert_eq!(db.stats().unwrap().migration_version, 3);

    db.migrate(standard_migrations()).unwrap();
    assert_eq!(db.stats().unwrap().migration_version, 5);
}

#[test]
fn stats_total_tables_excludes_internal_tables() {
    let dir = tempfile::TempDir::new().unwrap();
    let db = open_test_db(&dir);

    // Before migrations — _migrations may or may not exist
    db.migrate(vec![m(
        1,
        "create_users",
        "CREATE TABLE users (id TEXT PRIMARY KEY, tenant_id TEXT NOT NULL, \
         created_at INTEGER NOT NULL, updated_at INTEGER NOT NULL, deleted_at INTEGER)",
        None,
    )])
    .unwrap();

    let stats = db.stats().unwrap();
    // Should count 'users' but NOT '_migrations'
    assert_eq!(stats.total_tables, 1, "Internal _migrations table should be excluded");
}

// ─── Edge Cases ──────────────────────────────────────────────────────

#[test]
fn version_zero_rejected() {
    let dir = tempfile::TempDir::new().unwrap();
    let db = open_test_db(&dir);

    let result = db.migrate(vec![m(0, "bad", "SELECT 1", None)]);
    assert!(result.is_err());
    assert!(result.unwrap_err().to_string().contains("version must be > 0"));
}

#[test]
fn empty_migrations_list_is_noop() {
    let dir = tempfile::TempDir::new().unwrap();
    let db = open_test_db(&dir);

    let report = db.migrate(vec![]).unwrap();
    assert!(report.applied.is_empty());
    assert!(report.skipped.is_empty());
    assert_eq!(report.current_version, 0);
}

#[test]
fn multi_statement_migration() {
    let dir = tempfile::TempDir::new().unwrap();
    let db = open_test_db(&dir);

    let report = db
        .migrate(vec![m(
            1,
            "multi_stmt",
            "CREATE TABLE a (id TEXT PRIMARY KEY);\
             CREATE TABLE b (id TEXT PRIMARY KEY);\
             CREATE INDEX idx_a ON a(id);",
            Some("DROP INDEX idx_a; DROP TABLE b; DROP TABLE a;"),
        )])
        .unwrap();
    assert_eq!(report.applied, vec![1]);

    // Both tables should exist
    db.query_raw("SELECT * FROM a".into(), vec![]).unwrap();
    db.query_raw("SELECT * FROM b".into(), vec![]).unwrap();

    // Rollback should remove both
    db.rollback_to(0, vec![m(
        1,
        "multi_stmt",
        "CREATE TABLE a (id TEXT PRIMARY KEY);\
         CREATE TABLE b (id TEXT PRIMARY KEY);\
         CREATE INDEX idx_a ON a(id);",
        Some("DROP INDEX idx_a; DROP TABLE b; DROP TABLE a;"),
    )])
    .unwrap();

    assert!(db.query_raw("SELECT * FROM a".into(), vec![]).is_err());
    assert!(db.query_raw("SELECT * FROM b".into(), vec![]).is_err());
}

#[test]
fn migrate_then_rollback_then_re_migrate() {
    let dir = tempfile::TempDir::new().unwrap();
    let db = open_test_db(&dir);

    db.migrate(standard_migrations()).unwrap();
    db.rollback_to(2, standard_migrations()).unwrap();

    // Re-apply 3,4,5
    let report = db.migrate(standard_migrations()).unwrap();
    assert_eq!(report.applied, vec![3, 4, 5]);
    assert_eq!(report.current_version, 5);
}

#[test]
fn migrate_after_close_fails() {
    let dir = tempfile::TempDir::new().unwrap();
    let mut db = open_test_db(&dir);
    db.close().unwrap();

    assert!(db.migrate(standard_migrations()).is_err());
    assert!(db.rollback_to(0, standard_migrations()).is_err());
    assert!(db.get_migrations().is_err());
}

#[test]
fn duplicate_version_numbers_handled() {
    let dir = tempfile::TempDir::new().unwrap();
    let db = open_test_db(&dir);

    // Two migrations with same version — first one wins (sorted, first applied, second checksum-checked)
    let result = db.migrate(vec![
        m(1, "first", "CREATE TABLE dup1 (id TEXT PRIMARY KEY)", None),
        m(1, "second", "CREATE TABLE dup2 (id TEXT PRIMARY KEY)", None),
    ]);
    // The second one has a different checksum — should fail
    assert!(result.is_err());
}

#[test]
fn whitespace_only_difference_changes_checksum() {
    // Whitespace INSIDE the SQL (not leading/trailing) should change the checksum
    let dir = tempfile::TempDir::new().unwrap();
    let db = open_test_db(&dir);

    db.migrate(vec![m(1, "v1", "CREATE TABLE t (id TEXT PRIMARY KEY)", None)])
        .unwrap();

    // Extra space inside — different checksum
    let result = db.migrate(vec![m(
        1,
        "v1",
        "CREATE  TABLE  t  (id  TEXT  PRIMARY  KEY)",
        None,
    )]);
    assert!(result.is_err(), "Internal whitespace change should change checksum");
}
