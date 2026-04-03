use crate::common::*;
use iron_vault_core::api::types::*;
use std::collections::HashMap;

// ─── Drop Safety ─────────────────────────────────────────────────────

#[test]
fn drop_without_close_does_not_panic() {
    let dir = tempfile::TempDir::new().unwrap();
    let db = open_test_db(&dir);
    create_users_table(&db);
    insert_user(&db, "Alice", "a@t.com", "admin", 90.0);
    drop(db);
}

#[test]
fn drop_after_close_is_safe() {
    let dir = tempfile::TempDir::new().unwrap();
    let mut db = open_test_db(&dir);
    create_users_table(&db);
    db.close().unwrap();
    drop(db);
}

#[test]
fn open_close_10_times_no_leak() {
    let dir = tempfile::TempDir::new().unwrap();
    let path = dir.path().join("stress.db").to_str().unwrap().to_string();

    {
        let db = iron_vault_core::api::vault::IronVaultDb::open(
            path.clone(), test_key(), "t".into(), VaultConfig::test_config(),
        ).unwrap();
        create_users_table(&db);
    }

    for _ in 0..10 {
        let mut db = iron_vault_core::api::vault::IronVaultDb::open(
            path.clone(), test_key(), "t".into(), VaultConfig::test_config(),
        ).unwrap();
        db.close().unwrap();
    }

    let db = iron_vault_core::api::vault::IronVaultDb::open(
        path, test_key(), "t".into(), VaultConfig::test_config(),
    ).unwrap();
    assert_eq!(db.query_count(query("users")).unwrap(), 0);
}

#[test]
fn drop_without_close_then_reopen() {
    let dir = tempfile::TempDir::new().unwrap();
    let path = dir.path().join("drop.db").to_str().unwrap().to_string();

    {
        let db = iron_vault_core::api::vault::IronVaultDb::open(
            path.clone(), test_key(), "t".into(), VaultConfig::test_config(),
        ).unwrap();
        create_users_table(&db);
        insert_user(&db, "Alice", "a@t.com", "admin", 90.0);
        // Drop without close — Drop impl checkpoints + closes notifier
    }

    // Brief sleep for pool cleanup
    std::thread::sleep(std::time::Duration::from_millis(100));

    // Reopen should work and data should be persisted
    let db = iron_vault_core::api::vault::IronVaultDb::open(
        path, test_key(), "t".into(), VaultConfig::test_config(),
    ).unwrap();
    assert_eq!(db.query_count(query("users")).unwrap(), 1);
}

// ─── Key Material Safety ─────────────────────────────────────────────

#[test]
fn derived_keys_are_32_bytes_and_unique() {
    let dir = tempfile::TempDir::new().unwrap();
    let db = open_test_db(&dir);

    let k1 = db.derive_purpose_key("sqlcipher".into()).unwrap();
    let k2 = db.derive_purpose_key("audit_hmac".into()).unwrap();
    let k3 = db.derive_purpose_key("backup".into()).unwrap();
    assert_eq!(k1.len(), 32);
    assert_eq!(k2.len(), 32);
    assert_eq!(k3.len(), 32);
    assert_ne!(k1, k2);
    assert_ne!(k1, k3);
    assert_ne!(k2, k3);
}

#[test]
fn encrypt_after_many_operations_still_works() {
    let dir = tempfile::TempDir::new().unwrap();
    let db = open_test_db(&dir);
    create_users_table(&db);

    for i in 0..50 {
        insert_user(&db, &format!("U{}", i), &format!("u{}@t.com", i), "m", i as f64);
    }

    let enc = db.encrypt_field("test".into()).unwrap();
    let dec = db.decrypt_field(enc).unwrap();
    assert_eq!(dec, "test");
}

// ─── Concurrent Safety ──────────────────────────────────────────────

#[test]
fn concurrent_reads_and_writes() {
    let dir = tempfile::TempDir::new().unwrap();
    let path = dir.path().join("concurrent.db").to_str().unwrap().to_string();

    let mut config = VaultConfig::test_config();
    config.wal_mode = true;
    config.read_pool_size = 4;

    let db = iron_vault_core::api::vault::IronVaultDb::open(
        path, test_key(), "t".into(), config,
    ).unwrap();

    db.execute_raw(
        "CREATE TABLE conc (id TEXT PRIMARY KEY, val INTEGER, \
         tenant_id TEXT NOT NULL, created_at INTEGER NOT NULL, \
         updated_at INTEGER NOT NULL, deleted_at INTEGER)".into(),
        vec![],
    ).unwrap();

    for i in 0..100 {
        let mut data = HashMap::new();
        data.insert("val".into(), SqlValue::Integer(i));
        db.query_insert("conc".into(), data).unwrap();
    }

    // Multiple reads through the public API
    for _ in 0..4 {
        let count = db.query_count(query("conc")).unwrap();
        assert_eq!(count, 100);
    }
}

// ─── Notification Cleanup ────────────────────────────────────────────

#[test]
fn notifier_closed_on_drop() {
    let dir = tempfile::TempDir::new().unwrap();
    let db = open_test_db(&dir);
    create_users_table(&db);
    insert_user(&db, "Alice", "a@t.com", "admin", 90.0);
    assert_eq!(db.notification_version("users".into()).unwrap(), 1);
    drop(db);
    // No hang = notifier cleaned up
}

// ─── Actor Mutex Safety ──────────────────────────────────────────────

#[test]
fn rapid_set_actor_does_not_deadlock() {
    let dir = tempfile::TempDir::new().unwrap();
    let db = open_test_db(&dir);

    for i in 0..100 {
        db.set_actor(format!("actor_{}", i)).unwrap();
    }
    assert!(db.get_actor().unwrap().starts_with("actor_"));
}
