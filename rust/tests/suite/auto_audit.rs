use crate::common::*;
use iron_vault_core::api::types::*;
use std::collections::HashMap;

// ─── Insert Auto-Audit ───────────────────────────────────────────────

#[test]
fn insert_auto_creates_audit_entry() {
    let dir = tempfile::TempDir::new().unwrap();
    let db = open_test_db(&dir);
    create_users_table(&db);

    let id = insert_user(&db, "Alice", "a@t.com", "admin", 90.0);

    let history = db.get_history("users".into(), id.clone(), 50).unwrap();
    assert_eq!(history.len(), 1);
    assert_eq!(history[0].operation, "INSERT");
    assert!(
        history[0].before_json.is_none(),
        "INSERT should have no before"
    );
    assert!(history[0].after_json.is_some(), "INSERT should have after");
    let after = history[0].after_json.as_ref().unwrap();
    assert!(
        after.contains("Alice"),
        "after_json should contain the inserted data"
    );
}

#[test]
fn insert_audit_uses_current_actor() {
    let dir = tempfile::TempDir::new().unwrap();
    let db = open_test_db(&dir);
    create_users_table(&db);
    db.set_actor("user_42".into()).unwrap();

    let id = insert_user(&db, "Alice", "a@t.com", "admin", 90.0);

    let history = db.get_history("users".into(), id, 50).unwrap();
    assert_eq!(history[0].actor_id, "user_42");
}

// ─── Update Auto-Audit ───────────────────────────────────────────────

#[test]
fn update_auto_creates_audit_with_before_and_after() {
    let dir = tempfile::TempDir::new().unwrap();
    let db = open_test_db(&dir);
    create_users_table(&db);

    let id = insert_user(&db, "Alice", "a@t.com", "admin", 90.0);

    let mut data = HashMap::new();
    data.insert("name".into(), SqlValue::Text("Alicia".into()));
    db.query_update("users".into(), id.clone(), data).unwrap();

    let history = db.get_history("users".into(), id, 50).unwrap();
    // Most recent first: UPDATE, then INSERT
    assert_eq!(history.len(), 2);
    assert_eq!(history[0].operation, "UPDATE");
    assert!(
        history[0].before_json.is_some(),
        "UPDATE should have before"
    );
    assert!(history[0].after_json.is_some(), "UPDATE should have after");

    let before = history[0].before_json.as_ref().unwrap();
    let after = history[0].after_json.as_ref().unwrap();
    assert!(before.contains("Alice"), "before should have old name");
    assert!(after.contains("Alicia"), "after should have new name");
}

#[test]
fn update_auto_captures_changed_fields() {
    let dir = tempfile::TempDir::new().unwrap();
    let db = open_test_db(&dir);
    create_users_table(&db);

    let id = insert_user(&db, "Alice", "a@t.com", "admin", 90.0);
    std::thread::sleep(std::time::Duration::from_millis(2));

    let mut data = HashMap::new();
    data.insert("name".into(), SqlValue::Text("Bob".into()));
    db.query_update("users".into(), id.clone(), data).unwrap();

    let history = db.get_history("users".into(), id, 50).unwrap();
    let update_entry = &history[0];
    assert!(update_entry.changed_fields.is_some());
    let changed = update_entry.changed_fields.as_ref().unwrap();
    assert!(
        changed.contains("name"),
        "changed_fields should include 'name'"
    );
}

// ─── Delete Auto-Audit ───────────────────────────────────────────────

#[test]
fn soft_delete_auto_creates_audit_with_before() {
    let dir = tempfile::TempDir::new().unwrap();
    let db = open_test_db(&dir);
    create_users_table(&db);

    let id = insert_user(&db, "Alice", "a@t.com", "admin", 90.0);
    std::thread::sleep(std::time::Duration::from_millis(2));
    db.query_delete("users".into(), id.clone()).unwrap();

    let history = db.get_history("users".into(), id, 50).unwrap();
    // DELETE + INSERT
    let delete_entry = history.iter().find(|e| e.operation == "DELETE").unwrap();
    assert!(delete_entry.before_json.is_some());
    assert!(delete_entry.after_json.is_none());
    assert!(delete_entry.before_json.as_ref().unwrap().contains("Alice"));
}

#[test]
fn hard_delete_auto_creates_audit_with_before() {
    let dir = tempfile::TempDir::new().unwrap();
    let db = open_test_db(&dir);
    create_users_table(&db);

    let id = insert_user(&db, "Alice", "a@t.com", "admin", 90.0);
    std::thread::sleep(std::time::Duration::from_millis(2));
    db.query_hard_delete("users".into(), id.clone()).unwrap();

    let history = db.get_history("users".into(), id, 50).unwrap();
    let hd_entry = history
        .iter()
        .find(|e| e.operation == "HARD_DELETE")
        .unwrap();
    assert!(hd_entry.before_json.is_some());
    assert!(hd_entry.after_json.is_none());
}

// ─── Full Lifecycle ──────────────────────────────────────────────────

#[test]
fn full_lifecycle_produces_complete_audit_trail() {
    let dir = tempfile::TempDir::new().unwrap();
    let db = open_test_db(&dir);
    create_users_table(&db);

    db.set_actor("admin".into()).unwrap();
    let id = insert_user(&db, "Alice", "a@t.com", "admin", 90.0);
    std::thread::sleep(std::time::Duration::from_millis(2));

    let mut data = HashMap::new();
    data.insert("name".into(), SqlValue::Text("Alicia".into()));
    db.query_update("users".into(), id.clone(), data).unwrap();
    std::thread::sleep(std::time::Duration::from_millis(2));

    db.query_delete("users".into(), id.clone()).unwrap();

    let history = db.get_history("users".into(), id, 50).unwrap();
    assert_eq!(history.len(), 3);

    // Most recent first
    assert_eq!(history[0].operation, "DELETE");
    assert_eq!(history[1].operation, "UPDATE");
    assert_eq!(history[2].operation, "INSERT");

    // All signed with HMAC
    for entry in &history {
        assert_eq!(entry.checksum.len(), 64);
        assert_eq!(entry.actor_id, "admin");
    }

    // Integrity should pass
    let report = db.verify_audit_integrity(None, None).unwrap();
    assert!(report.is_clean);
    assert_eq!(report.total_checked, 3);
}

// ─── Update with 0 Rows Produces No Audit ────────────────────────────

#[test]
fn update_zero_rows_no_audit_entry() {
    let dir = tempfile::TempDir::new().unwrap();
    let db = open_test_db(&dir);
    create_users_table(&db);

    let mut data = HashMap::new();
    data.insert("name".into(), SqlValue::Text("Ghost".into()));
    db.query_update("users".into(), "nonexistent".into(), data)
        .unwrap();

    let history = db
        .get_history("users".into(), "nonexistent".into(), 50)
        .unwrap();
    assert!(
        history.is_empty(),
        "Zero-row update should produce no audit"
    );
}

// ─── Integrity After Auto-Audit ──────────────────────────────────────

#[test]
fn integrity_passes_after_auto_audited_writes() {
    let dir = tempfile::TempDir::new().unwrap();
    let db = open_test_db(&dir);
    create_users_table(&db);

    for i in 0..10 {
        insert_user(
            &db,
            &format!("User{}", i),
            &format!("u{}@t.com", i),
            "member",
            i as f64,
        );
    }

    let report = db.verify_audit_integrity(None, None).unwrap();
    assert!(report.is_clean);
    assert_eq!(report.total_checked, 10);
}
