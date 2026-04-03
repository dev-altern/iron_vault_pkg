use iron_vault_core::api::types::*;
use iron_vault_core::api::vault::IronVaultDb;

#[test]
fn wrong_key_fails() {
    let dir = tempfile::TempDir::new().unwrap();
    let path = dir.path().join("test.db").to_str().unwrap().to_string();

    {
        let db = IronVaultDb::open(
            path.clone(),
            vec![0x01u8; 32],
            "t".into(),
            VaultConfig::test_config(),
        )
        .unwrap();
        db.execute_raw(
            "CREATE TABLE key_test (id INTEGER PRIMARY KEY, val TEXT)".into(),
            vec![],
        )
        .unwrap();
        db.execute_raw(
            "INSERT INTO key_test (id, val) VALUES (1, 'secret')".into(),
            vec![],
        )
        .unwrap();
    }

    let result = IronVaultDb::open(path, vec![0x02u8; 32], "t".into(), VaultConfig::test_config());
    assert!(result.is_err());
    let msg = result.unwrap_err().to_string();
    assert!(msg.contains("VaultOpenException"), "got: {}", msg);
}

#[test]
fn invalid_key_length() {
    let dir = tempfile::TempDir::new().unwrap();
    let path = dir.path().join("test.db").to_str().unwrap().to_string();

    let result =
        IronVaultDb::open(path.clone(), vec![0u8; 16], "t".into(), VaultConfig::test_config());
    assert!(result.is_err());
    assert!(result.unwrap_err().to_string().contains("32 bytes"));

    let result = IronVaultDb::open(path, vec![0u8; 64], "t".into(), VaultConfig::test_config());
    assert!(result.is_err());
}

#[test]
fn reopen_with_correct_key_persists_data() {
    let dir = tempfile::TempDir::new().unwrap();
    let path = dir.path().join("persist.db").to_str().unwrap().to_string();
    let key = vec![0x42u8; 32];

    {
        let db = IronVaultDb::open(
            path.clone(),
            key.clone(),
            "t".into(),
            VaultConfig::test_config(),
        )
        .unwrap();
        db.execute_raw(
            "CREATE TABLE persist (id INTEGER PRIMARY KEY, msg TEXT)".into(),
            vec![],
        )
        .unwrap();
        db.execute_raw("INSERT INTO persist (id, msg) VALUES (1, 'hello')".into(), vec![])
            .unwrap();
        db.execute_raw("INSERT INTO persist (id, msg) VALUES (2, 'world')".into(), vec![])
            .unwrap();
    }

    {
        let db = IronVaultDb::open(path, key, "t".into(), VaultConfig::test_config()).unwrap();
        let rows = db
            .query_raw("SELECT msg FROM persist ORDER BY id".into(), vec![])
            .unwrap();
        assert_eq!(rows.len(), 2);
        assert!(matches!(rows[0].get("msg"), Some(SqlValue::Text(s)) if s == "hello"));
        assert!(matches!(rows[1].get("msg"), Some(SqlValue::Text(s)) if s == "world"));
    }
}
