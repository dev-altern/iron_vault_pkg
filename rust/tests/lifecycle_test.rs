use iron_vault_core::api::types::*;
use iron_vault_core::api::vault::IronVaultDb;

fn test_key() -> Vec<u8> {
    vec![0xABu8; 32]
}

fn open_test_db(dir: &tempfile::TempDir) -> IronVaultDb {
    let path = dir.path().join("test.db").to_str().unwrap().to_string();
    IronVaultDb::open(path, test_key(), "tenant_test".into(), VaultConfig::test_config())
        .expect("failed to open test database")
}

#[test]
fn open_close_roundtrip() {
    let dir = tempfile::TempDir::new().unwrap();
    let mut db = open_test_db(&dir);

    let stats = db.stats().unwrap();
    assert!(stats.page_size > 0);

    db.execute_raw("CREATE TABLE roundtrip (id INTEGER PRIMARY KEY)".into(), vec![])
        .unwrap();
    let stats = db.stats().unwrap();
    assert!(stats.db_size_bytes > 0);
    assert_eq!(stats.total_tables, 1);

    db.close().unwrap();

    let err = db.stats().unwrap_err();
    assert!(err.to_string().contains("closed"));
}

#[test]
fn double_close_is_safe() {
    let dir = tempfile::TempDir::new().unwrap();
    let mut db = open_test_db(&dir);
    db.close().unwrap();
    db.close().unwrap();
}

#[test]
fn all_operations_fail_after_close() {
    let dir = tempfile::TempDir::new().unwrap();
    let mut db = open_test_db(&dir);
    db.execute_raw("CREATE TABLE close_test (id INTEGER PRIMARY KEY)".into(), vec![])
        .unwrap();
    db.close().unwrap();

    let ops: Vec<(&str, Result<(), anyhow::Error>)> = vec![
        ("execute_raw", db.execute_raw("SELECT 1".into(), vec![]).map(|_| ())),
        ("query_raw", db.query_raw("SELECT 1".into(), vec![]).map(|_| ())),
        ("checkpoint", db.checkpoint(CheckpointMode::Passive).map(|_| ())),
        ("stats", db.stats().map(|_| ())),
        ("integrity_check", db.integrity_check().map(|_| ())),
        ("vacuum", db.vacuum()),
    ];
    for (name, result) in ops {
        assert!(result.is_err(), "{} should fail after close", name);
        assert!(
            result.unwrap_err().to_string().contains("closed"),
            "{} should mention 'closed'",
            name
        );
    }
}

#[test]
fn get_path_and_tenant_id() {
    let dir = tempfile::TempDir::new().unwrap();
    let path = dir.path().join("path_test.db").to_str().unwrap().to_string();
    let db =
        IronVaultDb::open(path.clone(), test_key(), "my_tenant".into(), VaultConfig::test_config())
            .unwrap();
    assert_eq!(db.get_path().unwrap(), path);
    assert_eq!(db.get_tenant_id().unwrap(), "my_tenant");
}

#[test]
fn parent_directory_created() {
    let dir = tempfile::TempDir::new().unwrap();
    let path = dir
        .path()
        .join("nested")
        .join("deep")
        .join("test.db")
        .to_str()
        .unwrap()
        .to_string();

    let db = IronVaultDb::open(path.clone(), test_key(), "t".into(), VaultConfig::test_config())
        .unwrap();
    assert_eq!(db.get_path().unwrap(), path);
}
