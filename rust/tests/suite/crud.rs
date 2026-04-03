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
fn execute_and_query_raw() {
    let dir = tempfile::TempDir::new().unwrap();
    let db = open_test_db(&dir);

    db.execute_raw(
        "CREATE TABLE test_users (id INTEGER PRIMARY KEY, name TEXT, score REAL)".into(),
        vec![],
    )
    .unwrap();

    let affected = db
        .execute_raw(
            "INSERT INTO test_users (id, name, score) VALUES (?1, ?2, ?3)".into(),
            vec![
                SqlValue::Integer(1),
                SqlValue::Text("Alice".into()),
                SqlValue::Real(95.5),
            ],
        )
        .unwrap();
    assert_eq!(affected, 1);

    db.execute_raw(
        "INSERT INTO test_users (id, name, score) VALUES (?1, ?2, ?3)".into(),
        vec![
            SqlValue::Integer(2),
            SqlValue::Text("Bob".into()),
            SqlValue::Real(87.0),
        ],
    )
    .unwrap();

    let rows = db
        .query_raw("SELECT id, name, score FROM test_users ORDER BY id".into(), vec![])
        .unwrap();
    assert_eq!(rows.len(), 2);

    if let Some(SqlValue::Text(name)) = rows[0].get("name") {
        assert_eq!(name, "Alice");
    } else {
        panic!("Expected text for name");
    }
    if let Some(SqlValue::Real(score)) = rows[0].get("score") {
        assert!((score - 95.5).abs() < f64::EPSILON);
    } else {
        panic!("Expected real for score");
    }

    let rows = db
        .query_raw(
            "SELECT name FROM test_users WHERE score > ?1".into(),
            vec![SqlValue::Real(90.0)],
        )
        .unwrap();
    assert_eq!(rows.len(), 1);
    if let Some(SqlValue::Text(name)) = rows[0].get("name") {
        assert_eq!(name, "Alice");
    }
}

#[test]
fn update_and_delete() {
    let dir = tempfile::TempDir::new().unwrap();
    let db = open_test_db(&dir);

    db.execute_raw(
        "CREATE TABLE crud (id INTEGER PRIMARY KEY, val TEXT)".into(),
        vec![],
    )
    .unwrap();
    db.execute_raw(
        "INSERT INTO crud (id, val) VALUES (?1, ?2)".into(),
        vec![SqlValue::Integer(1), SqlValue::Text("original".into())],
    )
    .unwrap();

    let affected = db
        .execute_raw(
            "UPDATE crud SET val = ?1 WHERE id = ?2".into(),
            vec![SqlValue::Text("updated".into()), SqlValue::Integer(1)],
        )
        .unwrap();
    assert_eq!(affected, 1);

    let rows = db
        .query_raw("SELECT val FROM crud WHERE id = 1".into(), vec![])
        .unwrap();
    assert!(matches!(rows[0].get("val"), Some(SqlValue::Text(s)) if s == "updated"));

    let affected = db
        .execute_raw(
            "DELETE FROM crud WHERE id = ?1".into(),
            vec![SqlValue::Integer(1)],
        )
        .unwrap();
    assert_eq!(affected, 1);

    let rows = db.query_raw("SELECT * FROM crud".into(), vec![]).unwrap();
    assert!(rows.is_empty());
}

#[test]
fn null_and_blob_values() {
    let dir = tempfile::TempDir::new().unwrap();
    let db = open_test_db(&dir);

    db.execute_raw(
        "CREATE TABLE blob_test (id INTEGER PRIMARY KEY, data BLOB, note TEXT)".into(),
        vec![],
    )
    .unwrap();

    let blob_data = vec![0xDE, 0xAD, 0xBE, 0xEF];
    db.execute_raw(
        "INSERT INTO blob_test (id, data, note) VALUES (?1, ?2, ?3)".into(),
        vec![
            SqlValue::Integer(1),
            SqlValue::Blob(blob_data.clone()),
            SqlValue::Null,
        ],
    )
    .unwrap();

    let rows = db
        .query_raw("SELECT data, note FROM blob_test WHERE id = 1".into(), vec![])
        .unwrap();
    assert_eq!(rows.len(), 1);

    if let Some(SqlValue::Blob(data)) = rows[0].get("data") {
        assert_eq!(data, &blob_data);
    } else {
        panic!("Expected blob");
    }

    match rows[0].get("note") {
        Some(SqlValue::Null) => {}
        other => panic!("Expected null, got {:?}", other),
    }
}

#[test]
fn empty_result_set() {
    let dir = tempfile::TempDir::new().unwrap();
    let db = open_test_db(&dir);

    db.execute_raw("CREATE TABLE empty_test (id INTEGER PRIMARY KEY)".into(), vec![])
        .unwrap();

    let rows = db.query_raw("SELECT * FROM empty_test".into(), vec![]).unwrap();
    assert!(rows.is_empty());
}

#[test]
fn large_batch_insert_and_query() {
    let dir = tempfile::TempDir::new().unwrap();
    let db = open_test_db(&dir);

    db.execute_raw("CREATE TABLE batch (id INTEGER PRIMARY KEY, val TEXT)".into(), vec![])
        .unwrap();

    db.execute_raw("BEGIN".into(), vec![]).unwrap();
    for i in 0..1000 {
        db.execute_raw(
            "INSERT INTO batch (id, val) VALUES (?1, ?2)".into(),
            vec![SqlValue::Integer(i), SqlValue::Text(format!("row_{}", i))],
        )
        .unwrap();
    }
    db.execute_raw("COMMIT".into(), vec![]).unwrap();

    let rows = db
        .query_raw("SELECT count(*) as cnt FROM batch".into(), vec![])
        .unwrap();
    assert!(matches!(rows[0].get("cnt"), Some(SqlValue::Integer(1000))));

    let rows = db
        .query_raw(
            "SELECT id FROM batch WHERE id >= ?1 AND id < ?2 ORDER BY id".into(),
            vec![SqlValue::Integer(990), SqlValue::Integer(1000)],
        )
        .unwrap();
    assert_eq!(rows.len(), 10);
}

#[test]
fn bad_sql_returns_error() {
    let dir = tempfile::TempDir::new().unwrap();
    let db = open_test_db(&dir);

    assert!(db.execute_raw("NOT VALID SQL".into(), vec![]).is_err());
    assert!(db.query_raw("SELECT FROM WHERE".into(), vec![]).is_err());
    assert!(db.query_raw("SELECT * FROM nonexistent_table".into(), vec![]).is_err());

    // Wrong number of params — must not crash
    let _result = db.execute_raw("SELECT ?1, ?2".into(), vec![SqlValue::Integer(1)]);
}

#[test]
fn foreign_keys_enforced() {
    let dir = tempfile::TempDir::new().unwrap();
    let db = open_test_db(&dir);

    db.execute_raw("CREATE TABLE parent (id INTEGER PRIMARY KEY)".into(), vec![])
        .unwrap();
    db.execute_raw(
        "CREATE TABLE child (id INTEGER PRIMARY KEY, parent_id INTEGER REFERENCES parent(id))"
            .into(),
        vec![],
    )
    .unwrap();

    let result = db.execute_raw(
        "INSERT INTO child (id, parent_id) VALUES (1, 999)".into(),
        vec![],
    );
    assert!(result.is_err(), "Foreign key violation should fail");
}
