use iron_vault_core::api::types::*;
use iron_vault_core::api::vault::IronVaultDb;

fn test_key() -> Vec<u8> {
    vec![0xABu8; 32]
}

fn open_wal_db(dir: &tempfile::TempDir) -> IronVaultDb {
    let path = dir.path().join("test.db").to_str().unwrap().to_string();
    let mut config = VaultConfig::test_config();
    config.wal_mode = true;
    IronVaultDb::open(path, test_key(), "t".into(), config).unwrap()
}

#[test]
fn wal_mode_active() {
    let dir = tempfile::TempDir::new().unwrap();
    let db = open_wal_db(&dir);

    let rows = db.query_raw("PRAGMA journal_mode".into(), vec![]).unwrap();
    assert_eq!(rows.len(), 1);
    if let Some(SqlValue::Text(mode)) = rows[0].get("journal_mode") {
        assert_eq!(mode.to_lowercase(), "wal");
    } else {
        panic!("Expected text value for journal_mode");
    }
}

#[test]
fn checkpoint_passive() {
    let dir = tempfile::TempDir::new().unwrap();
    let db = open_wal_db(&dir);

    db.execute_raw(
        "CREATE TABLE ckpt_test (id INTEGER PRIMARY KEY)".into(),
        vec![],
    )
    .unwrap();

    let result = db.checkpoint(CheckpointMode::Passive).unwrap();
    assert!(result.wal_pages >= 0);
    assert!(result.checkpointed_pages >= 0);
}

#[test]
fn checkpoint_truncate_shrinks_wal() {
    let dir = tempfile::TempDir::new().unwrap();
    let path = dir.path().join("trunc.db").to_str().unwrap().to_string();

    let mut config = VaultConfig::test_config();
    config.wal_mode = true;
    let db = IronVaultDb::open(path.clone(), test_key(), "t".into(), config).unwrap();

    db.execute_raw(
        "CREATE TABLE trunc_test (id INTEGER PRIMARY KEY, data TEXT)".into(),
        vec![],
    )
    .unwrap();
    for i in 0..50 {
        db.execute_raw(
            "INSERT INTO trunc_test (id, data) VALUES (?1, ?2)".into(),
            vec![SqlValue::Integer(i), SqlValue::Text("data".repeat(100))],
        )
        .unwrap();
    }

    db.checkpoint(CheckpointMode::Truncate).unwrap();

    let wal_size = std::fs::metadata(format!("{}-wal", path))
        .map(|m| m.len())
        .unwrap_or(0);
    assert_eq!(wal_size, 0, "WAL should be truncated to 0 bytes");
}
