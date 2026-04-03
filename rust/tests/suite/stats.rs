use iron_vault_core::api::types::*;
use iron_vault_core::api::vault::IronVaultDb;

fn test_key() -> Vec<u8> {
    vec![0xABu8; 32]
}

fn open_test_db(dir: &tempfile::TempDir) -> IronVaultDb {
    let path = dir.path().join("test.db").to_str().unwrap().to_string();
    IronVaultDb::open(
        path,
        test_key(),
        "tenant_test".into(),
        VaultConfig::test_config(),
    )
    .expect("failed to open test database")
}

#[test]
fn stats_empty_db() {
    let dir = tempfile::TempDir::new().unwrap();
    let db = open_test_db(&dir);

    let stats = db.stats().unwrap();
    assert_eq!(stats.total_tables, 0);
    assert!(stats.page_size > 0);
}

#[test]
fn stats_after_create_table() {
    let dir = tempfile::TempDir::new().unwrap();
    let db = open_test_db(&dir);

    db.execute_raw(
        "CREATE TABLE stats_test (id INTEGER PRIMARY KEY, val TEXT)".into(),
        vec![],
    )
    .unwrap();

    let stats = db.stats().unwrap();
    assert_eq!(stats.total_tables, 1);
    assert!(stats.db_size_bytes > 0);
}

#[test]
fn stats_multiple_tables() {
    let dir = tempfile::TempDir::new().unwrap();
    let db = open_test_db(&dir);

    for i in 0..5 {
        db.execute_raw(
            format!("CREATE TABLE tbl_{} (id INTEGER PRIMARY KEY)", i),
            vec![],
        )
        .unwrap();
    }

    assert_eq!(db.stats().unwrap().total_tables, 5);
}

#[test]
fn integrity_check_clean() {
    let dir = tempfile::TempDir::new().unwrap();
    let db = open_test_db(&dir);

    let report = db.integrity_check().unwrap();
    assert!(report.is_clean);
    assert!(report.errors.is_empty());
}

#[test]
fn integrity_after_heavy_writes() {
    let dir = tempfile::TempDir::new().unwrap();
    let db = open_test_db(&dir);

    db.execute_raw(
        "CREATE TABLE integrity_test (id INTEGER PRIMARY KEY, data BLOB)".into(),
        vec![],
    )
    .unwrap();

    for i in 0..100 {
        db.execute_raw(
            "INSERT INTO integrity_test (id, data) VALUES (?1, ?2)".into(),
            vec![SqlValue::Integer(i), SqlValue::Blob(vec![i as u8; 256])],
        )
        .unwrap();
    }

    let report = db.integrity_check().unwrap();
    assert!(report.is_clean, "Integrity failed: {:?}", report.errors);
}

#[test]
fn vacuum_after_bulk_delete() {
    let dir = tempfile::TempDir::new().unwrap();
    let db = open_test_db(&dir);

    db.execute_raw(
        "CREATE TABLE vac_test (id INTEGER PRIMARY KEY, data TEXT)".into(),
        vec![],
    )
    .unwrap();

    for i in 0..100 {
        db.execute_raw(
            "INSERT INTO vac_test (id, data) VALUES (?1, ?2)".into(),
            vec![SqlValue::Integer(i), SqlValue::Text("x".repeat(1000))],
        )
        .unwrap();
    }
    db.execute_raw("DELETE FROM vac_test".into(), vec![])
        .unwrap();

    db.vacuum().unwrap();
}
