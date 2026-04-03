use crate::common::*;
use iron_vault_core::api::types::*;

// ─── Actor Management ────────────────────────────────────────────────

#[test]
fn default_actor_is_system() {
    let dir = tempfile::TempDir::new().unwrap();
    let db = open_test_db(&dir);
    assert_eq!(db.get_actor().unwrap(), "system");
}

#[test]
fn set_and_get_actor() {
    let dir = tempfile::TempDir::new().unwrap();
    let db = open_test_db(&dir);
    db.set_actor("user_123".into()).unwrap();
    assert_eq!(db.get_actor().unwrap(), "user_123");
}

#[test]
fn clear_actor_resets_to_system() {
    let dir = tempfile::TempDir::new().unwrap();
    let db = open_test_db(&dir);
    db.set_actor("user_123".into()).unwrap();
    db.clear_actor().unwrap();
    assert_eq!(db.get_actor().unwrap(), "system");
}

#[test]
fn actor_ops_fail_after_close() {
    let dir = tempfile::TempDir::new().unwrap();
    let mut db = open_test_db(&dir);
    db.close().unwrap();
    assert!(db.set_actor("x".into()).is_err());
    assert!(db.clear_actor().is_err());
    assert!(db.get_actor().is_err());
}

// ─── Manual Audit Entries ────────────────────────────────────────────

#[test]
fn write_audit_creates_entry() {
    let dir = tempfile::TempDir::new().unwrap();
    let db = open_test_db(&dir);

    let audit_id = db
        .write_audit(
            "users".into(),
            "row_1".into(),
            "INSERT".into(),
            None,
            Some("{\"name\":\"Alice\"}".into()),
            None,
        )
        .unwrap();
    assert!(!audit_id.is_empty());

    let history = db.get_history("users".into(), "row_1".into(), 50).unwrap();
    assert_eq!(history.len(), 1);
    assert_eq!(history[0].operation, "INSERT");
    assert_eq!(history[0].actor_id, "system");
    assert_eq!(
        history[0].after_json.as_deref(),
        Some("{\"name\":\"Alice\"}")
    );
    assert!(history[0].before_json.is_none());
}

#[test]
fn write_audit_uses_current_actor() {
    let dir = tempfile::TempDir::new().unwrap();
    let db = open_test_db(&dir);
    db.set_actor("admin_42".into()).unwrap();

    db.write_audit(
        "users".into(),
        "r1".into(),
        "UPDATE".into(),
        None,
        None,
        None,
    )
    .unwrap();

    let history = db.get_history("users".into(), "r1".into(), 50).unwrap();
    assert_eq!(history[0].actor_id, "admin_42");
}

#[test]
fn write_audit_captures_before_and_after() {
    let dir = tempfile::TempDir::new().unwrap();
    let db = open_test_db(&dir);

    db.write_audit(
        "users".into(),
        "r1".into(),
        "UPDATE".into(),
        Some("{\"name\":\"Old\"}".into()),
        Some("{\"name\":\"New\"}".into()),
        Some("[\"name\"]".into()),
    )
    .unwrap();

    let h = db.get_history("users".into(), "r1".into(), 50).unwrap();
    assert_eq!(h[0].before_json.as_deref(), Some("{\"name\":\"Old\"}"));
    assert_eq!(h[0].after_json.as_deref(), Some("{\"name\":\"New\"}"));
    assert_eq!(h[0].changed_fields.as_deref(), Some("[\"name\"]"));
}

#[test]
fn multiple_entries_for_same_row() {
    let dir = tempfile::TempDir::new().unwrap();
    let db = open_test_db(&dir);

    db.write_audit(
        "users".into(),
        "r1".into(),
        "INSERT".into(),
        None,
        Some("{}".into()),
        None,
    )
    .unwrap();
    std::thread::sleep(std::time::Duration::from_millis(2));
    db.write_audit(
        "users".into(),
        "r1".into(),
        "UPDATE".into(),
        Some("{}".into()),
        Some("{\"v\":2}".into()),
        None,
    )
    .unwrap();
    std::thread::sleep(std::time::Duration::from_millis(2));
    db.write_audit(
        "users".into(),
        "r1".into(),
        "DELETE".into(),
        Some("{\"v\":2}".into()),
        None,
        None,
    )
    .unwrap();

    let history = db.get_history("users".into(), "r1".into(), 50).unwrap();
    assert_eq!(history.len(), 3);
    // Most recent first
    assert_eq!(history[0].operation, "DELETE");
    assert_eq!(history[1].operation, "UPDATE");
    assert_eq!(history[2].operation, "INSERT");
}

// ─── Query APIs ──────────────────────────────────────────────────────

#[test]
fn get_actor_history_filters_by_actor() {
    let dir = tempfile::TempDir::new().unwrap();
    let db = open_test_db(&dir);

    db.set_actor("alice".into()).unwrap();
    db.write_audit(
        "users".into(),
        "r1".into(),
        "INSERT".into(),
        None,
        None,
        None,
    )
    .unwrap();
    db.write_audit(
        "orders".into(),
        "r2".into(),
        "INSERT".into(),
        None,
        None,
        None,
    )
    .unwrap();

    db.set_actor("bob".into()).unwrap();
    db.write_audit(
        "users".into(),
        "r3".into(),
        "INSERT".into(),
        None,
        None,
        None,
    )
    .unwrap();

    let alice_history = db
        .get_actor_history("alice".into(), None, None, 50)
        .unwrap();
    assert_eq!(alice_history.len(), 2);

    let bob_history = db.get_actor_history("bob".into(), None, None, 50).unwrap();
    assert_eq!(bob_history.len(), 1);
}

#[test]
fn get_table_history_filters_by_table() {
    let dir = tempfile::TempDir::new().unwrap();
    let db = open_test_db(&dir);

    db.write_audit(
        "users".into(),
        "r1".into(),
        "INSERT".into(),
        None,
        None,
        None,
    )
    .unwrap();
    db.write_audit(
        "users".into(),
        "r2".into(),
        "INSERT".into(),
        None,
        None,
        None,
    )
    .unwrap();
    db.write_audit(
        "orders".into(),
        "r3".into(),
        "INSERT".into(),
        None,
        None,
        None,
    )
    .unwrap();

    let users_h = db
        .get_table_history("users".into(), None, None, 50)
        .unwrap();
    assert_eq!(users_h.len(), 2);

    let orders_h = db
        .get_table_history("orders".into(), None, None, 50)
        .unwrap();
    assert_eq!(orders_h.len(), 1);
}

#[test]
fn get_history_respects_limit() {
    let dir = tempfile::TempDir::new().unwrap();
    let db = open_test_db(&dir);

    for i in 0..10 {
        db.write_audit(
            "users".into(),
            "r1".into(),
            "UPDATE".into(),
            None,
            Some(format!("{{{}}}", i)),
            None,
        )
        .unwrap();
    }

    let h = db.get_history("users".into(), "r1".into(), 3).unwrap();
    assert_eq!(h.len(), 3);
}

#[test]
fn get_history_empty_when_no_entries() {
    let dir = tempfile::TempDir::new().unwrap();
    let db = open_test_db(&dir);

    let h = db
        .get_history("users".into(), "nonexistent".into(), 50)
        .unwrap();
    assert!(h.is_empty());
}

// ─── HMAC Integrity Verification ─────────────────────────────────────

#[test]
fn verify_integrity_clean_log() {
    let dir = tempfile::TempDir::new().unwrap();
    let db = open_test_db(&dir);

    db.write_audit(
        "users".into(),
        "r1".into(),
        "INSERT".into(),
        None,
        Some("{}".into()),
        None,
    )
    .unwrap();
    db.write_audit(
        "users".into(),
        "r2".into(),
        "INSERT".into(),
        None,
        Some("{}".into()),
        None,
    )
    .unwrap();

    let report = db.verify_audit_integrity(None, None).unwrap();
    assert!(report.is_clean);
    assert_eq!(report.total_checked, 2);
    assert!(report.tampered_ids.is_empty());
}

#[test]
fn verify_integrity_detects_tampered_actor() {
    let dir = tempfile::TempDir::new().unwrap();
    let db = open_test_db(&dir);

    db.write_audit(
        "users".into(),
        "r1".into(),
        "INSERT".into(),
        None,
        None,
        None,
    )
    .unwrap();
    let history = db.get_history("users".into(), "r1".into(), 1).unwrap();
    let entry_id = &history[0].id;

    // Tamper: modify actor_id directly in SQL
    db.execute_raw(
        format!(
            "UPDATE _audit_log SET actor_id = 'hacker' WHERE id = '{}'",
            entry_id
        ),
        vec![],
    )
    .unwrap();

    let report = db.verify_audit_integrity(None, None).unwrap();
    assert!(!report.is_clean);
    assert_eq!(report.tampered_ids.len(), 1);
    assert_eq!(report.tampered_ids[0], *entry_id);
}

#[test]
fn verify_integrity_detects_tampered_data() {
    let dir = tempfile::TempDir::new().unwrap();
    let db = open_test_db(&dir);

    db.write_audit(
        "users".into(),
        "r1".into(),
        "INSERT".into(),
        None,
        Some("{\"name\":\"Alice\"}".into()),
        None,
    )
    .unwrap();
    let history = db.get_history("users".into(), "r1".into(), 1).unwrap();
    let entry_id = &history[0].id;

    // Tamper: modify after_json
    db.execute_raw(
        format!(
            "UPDATE _audit_log SET after_json = '{{\"name\":\"EVIL\"}}' WHERE id = '{}'",
            entry_id
        ),
        vec![],
    )
    .unwrap();

    let report = db.verify_audit_integrity(None, None).unwrap();
    assert!(!report.is_clean);
    assert_eq!(report.tampered_ids[0], *entry_id);
}

#[test]
fn verify_integrity_detects_deleted_and_reinserted() {
    let dir = tempfile::TempDir::new().unwrap();
    let db = open_test_db(&dir);

    db.write_audit(
        "users".into(),
        "r1".into(),
        "INSERT".into(),
        None,
        None,
        None,
    )
    .unwrap();
    let history = db.get_history("users".into(), "r1".into(), 1).unwrap();
    let entry_id = &history[0].id;

    // Delete and reinsert with wrong checksum
    db.execute_raw(
        format!("DELETE FROM _audit_log WHERE id = '{}'", entry_id),
        vec![],
    )
    .unwrap();
    db.execute_raw(
        format!(
            "INSERT INTO _audit_log (id, table_name, row_id, operation, actor_id, tenant_id, timestamp, checksum) \
             VALUES ('{}', 'users', 'r1', 'INSERT', 'system', 'tenant_test', 0, 'fake_checksum')",
            entry_id
        ),
        vec![],
    )
    .unwrap();

    let report = db.verify_audit_integrity(None, None).unwrap();
    assert!(!report.is_clean);
}

// ─── Tenant Isolation ────────────────────────────────────────────────

#[test]
fn audit_entries_scoped_to_tenant() {
    let dir = tempfile::TempDir::new().unwrap();
    let path = dir.path().join("shared.db").to_str().unwrap().to_string();
    let db_a = iron_vault_core::api::vault::IronVaultDb::open(
        path.clone(),
        test_key(),
        "tenant_a".into(),
        VaultConfig::test_config(),
    )
    .unwrap();
    let db_b = iron_vault_core::api::vault::IronVaultDb::open(
        path,
        test_key(),
        "tenant_b".into(),
        VaultConfig::test_config(),
    )
    .unwrap();

    db_a.write_audit(
        "users".into(),
        "r1".into(),
        "INSERT".into(),
        None,
        None,
        None,
    )
    .unwrap();
    db_a.write_audit(
        "users".into(),
        "r2".into(),
        "INSERT".into(),
        None,
        None,
        None,
    )
    .unwrap();
    db_b.write_audit(
        "users".into(),
        "r3".into(),
        "INSERT".into(),
        None,
        None,
        None,
    )
    .unwrap();

    // Each tenant only sees their own entries
    assert_eq!(
        db_a.get_history("users".into(), "r1".into(), 50)
            .unwrap()
            .len(),
        1
    );
    assert_eq!(
        db_a.get_history("users".into(), "r3".into(), 50)
            .unwrap()
            .len(),
        0
    ); // r3 is tenant_b
    assert_eq!(
        db_b.get_history("users".into(), "r3".into(), 50)
            .unwrap()
            .len(),
        1
    );

    // Integrity check scoped to tenant
    let report_a = db_a.verify_audit_integrity(None, None).unwrap();
    assert_eq!(report_a.total_checked, 2);
    let report_b = db_b.verify_audit_integrity(None, None).unwrap();
    assert_eq!(report_b.total_checked, 1);
}

// ─── Edge Cases ──────────────────────────────────────────────────────

#[test]
fn audit_with_unicode_data() {
    let dir = tempfile::TempDir::new().unwrap();
    let db = open_test_db(&dir);

    db.write_audit(
        "users".into(),
        "r1".into(),
        "INSERT".into(),
        None,
        Some("{\"name\":\"café ☕ 日本語\"}".into()),
        None,
    )
    .unwrap();

    let h = db.get_history("users".into(), "r1".into(), 1).unwrap();
    assert_eq!(
        h[0].after_json.as_deref(),
        Some("{\"name\":\"café ☕ 日本語\"}")
    );

    // Integrity still valid
    let report = db.verify_audit_integrity(None, None).unwrap();
    assert!(report.is_clean);
}

#[test]
fn audit_with_large_json() {
    let dir = tempfile::TempDir::new().unwrap();
    let db = open_test_db(&dir);

    let large_json = format!("{{\"data\":\"{}\"}}", "x".repeat(100_000));
    db.write_audit(
        "users".into(),
        "r1".into(),
        "INSERT".into(),
        None,
        Some(large_json.clone()),
        None,
    )
    .unwrap();

    let h = db.get_history("users".into(), "r1".into(), 1).unwrap();
    assert_eq!(h[0].after_json.as_deref(), Some(large_json.as_str()));

    let report = db.verify_audit_integrity(None, None).unwrap();
    assert!(report.is_clean);
}

#[test]
fn audit_entry_has_valid_timestamp() {
    let dir = tempfile::TempDir::new().unwrap();
    let db = open_test_db(&dir);

    let before = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap()
        .as_millis() as i64;
    db.write_audit(
        "users".into(),
        "r1".into(),
        "INSERT".into(),
        None,
        None,
        None,
    )
    .unwrap();
    let after = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap()
        .as_millis() as i64;

    let h = db.get_history("users".into(), "r1".into(), 1).unwrap();
    assert!(h[0].timestamp >= before && h[0].timestamp <= after);
}

#[test]
fn audit_checksum_is_64_hex_chars() {
    let dir = tempfile::TempDir::new().unwrap();
    let db = open_test_db(&dir);

    db.write_audit("t".into(), "r".into(), "INSERT".into(), None, None, None)
        .unwrap();
    let h = db.get_history("t".into(), "r".into(), 1).unwrap();
    assert_eq!(h[0].checksum.len(), 64);
    assert!(h[0].checksum.chars().all(|c| c.is_ascii_hexdigit()));
}

#[test]
fn audit_ops_fail_after_close() {
    let dir = tempfile::TempDir::new().unwrap();
    let mut db = open_test_db(&dir);
    db.close().unwrap();

    assert!(db
        .write_audit("t".into(), "r".into(), "INSERT".into(), None, None, None)
        .is_err());
    assert!(db.get_history("t".into(), "r".into(), 50).is_err());
    assert!(db.get_actor_history("a".into(), None, None, 50).is_err());
    assert!(db.get_table_history("t".into(), None, None, 50).is_err());
    assert!(db.verify_audit_integrity(None, None).is_err());
}

#[test]
fn verify_integrity_on_empty_log() {
    let dir = tempfile::TempDir::new().unwrap();
    let db = open_test_db(&dir);

    let report = db.verify_audit_integrity(None, None).unwrap();
    assert!(report.is_clean);
    assert_eq!(report.total_checked, 0);
    assert!(report.tampered_ids.is_empty());
}
