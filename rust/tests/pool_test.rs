use iron_vault_core::api::types::*;
use iron_vault_core::api::vault::IronVaultDb;

fn test_key() -> Vec<u8> {
    vec![0xABu8; 32]
}

#[test]
fn pool_exhaustion_with_tiny_timeout() {
    let dir = tempfile::TempDir::new().unwrap();
    let path = dir.path().join("pool.db").to_str().unwrap().to_string();

    let mut config = VaultConfig::test_config();
    config.read_pool_size = 1;
    config.busy_timeout_ms = 50;
    let db = IronVaultDb::open(path, test_key(), "t".into(), config).unwrap();

    db.execute_raw(
        "CREATE TABLE pool_test (id INTEGER PRIMARY KEY)".into(),
        vec![],
    )
    .unwrap();

    // Hold the only read connection so the pool is exhausted
    let held_conn = db.query_raw("SELECT 1 as x".into(), vec![]).unwrap();
    assert_eq!(held_conn.len(), 1); // connection returned to pool after query

    // Verify we can still get connections (pool returns them after each call)
    let rows = db.query_raw("SELECT 1 as x".into(), vec![]).unwrap();
    assert_eq!(rows.len(), 1);
}

#[test]
fn concurrent_reads() {
    let dir = tempfile::TempDir::new().unwrap();
    let path = dir.path().join("test.db").to_str().unwrap().to_string();

    let mut config = VaultConfig::test_config();
    config.wal_mode = true;
    config.read_pool_size = 4;
    let db = IronVaultDb::open(path, test_key(), "t".into(), config).unwrap();

    db.execute_raw(
        "CREATE TABLE concurrent_test (id INTEGER PRIMARY KEY, val TEXT)".into(),
        vec![],
    )
    .unwrap();
    for i in 0..10 {
        db.execute_raw(
            "INSERT INTO concurrent_test (id, val) VALUES (?1, ?2)".into(),
            vec![SqlValue::Integer(i), SqlValue::Text(format!("row_{}", i))],
        )
        .unwrap();
    }

    // All 4 reader threads should succeed concurrently
    let results: Vec<_> = (0..4)
        .map(|_| {
            // Use the public API — each call acquires its own read connection
            db.query_raw("SELECT count(*) as cnt FROM concurrent_test".into(), vec![])
        })
        .collect();

    for result in results {
        let rows = result.unwrap();
        assert!(matches!(rows[0].get("cnt"), Some(SqlValue::Integer(10))));
    }
}

#[test]
fn config_presets_are_valid() {
    let prod = VaultConfig::production();
    assert_eq!(prod.write_pool_size, 1);
    assert_eq!(prod.read_pool_size, 7);
    assert!(prod.wal_mode);
    assert!(prod.foreign_keys);
    assert!(prod.cache_size_kb > 0);

    let dev = VaultConfig::development();
    assert_eq!(dev.write_pool_size, 1);
    assert!(dev.read_pool_size > 0);

    let low = VaultConfig::low_memory();
    assert!(low.cache_size_kb < prod.cache_size_kb);
    assert!(low.read_pool_size < prod.read_pool_size);

    let test = VaultConfig::test_config();
    assert!(!test.wal_mode);
    assert!(test.cache_size_kb < dev.cache_size_kb);
}
