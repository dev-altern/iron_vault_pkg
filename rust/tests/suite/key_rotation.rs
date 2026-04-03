use crate::common::*;
use iron_vault_core::api::types::*;
use std::collections::HashMap;

fn create_encrypted_table(db: &iron_vault_core::api::vault::IronVaultDb) {
    db.execute_raw(
        "CREATE TABLE secrets (\
            id TEXT PRIMARY KEY, email_enc TEXT, name_enc TEXT, \
            tenant_id TEXT NOT NULL, created_at INTEGER NOT NULL, \
            updated_at INTEGER NOT NULL, deleted_at INTEGER)"
            .into(),
        vec![],
    )
    .unwrap();
}

fn insert_encrypted(
    db: &iron_vault_core::api::vault::IronVaultDb,
    email: &str,
    name: &str,
) -> String {
    let enc_email = db.encrypt_field(email.into()).unwrap();
    let enc_name = db.encrypt_field(name.into()).unwrap();
    let mut data = HashMap::new();
    data.insert("email_enc".into(), SqlValue::Text(enc_email));
    data.insert("name_enc".into(), SqlValue::Text(enc_name));
    db.query_insert("secrets".into(), data).unwrap()
}

// ─── Basic Rotation ──────────────────────────────────────────────────

#[test]
fn rotate_re_encrypts_all_rows() {
    let dir = tempfile::TempDir::new().unwrap();
    let db = open_test_db(&dir);
    create_encrypted_table(&db);

    let id1 = insert_encrypted(&db, "alice@test.com", "Alice");
    let id2 = insert_encrypted(&db, "bob@test.com", "Bob");

    // Verify decryptable with current key
    let row = db
        .query_raw(
            "SELECT email_enc FROM secrets WHERE id = ?1 AND tenant_id = 'tenant_test'".into(),
            vec![SqlValue::Text(id1.clone())],
        )
        .unwrap();
    let enc = match &row[0]["email_enc"] {
        SqlValue::Text(s) => s.clone(),
        _ => panic!(),
    };
    assert_eq!(db.decrypt_field(enc).unwrap(), "alice@test.com");

    // Rotate to new key
    let new_key = vec![0xBBu8; 32];
    let mut tables = HashMap::new();
    tables.insert(
        "secrets".into(),
        vec!["email_enc".into(), "name_enc".into()],
    );
    let rotated = db.rotate_field_keys(new_key, tables).unwrap();
    assert_eq!(rotated, 4); // 2 rows × 2 columns

    // Verify decryptable with NEW key (internal key was updated)
    let row = db
        .query_raw(
            "SELECT email_enc, name_enc FROM secrets WHERE id = ?1 AND tenant_id = 'tenant_test'"
                .into(),
            vec![SqlValue::Text(id1.clone())],
        )
        .unwrap();
    let enc_email = match &row[0]["email_enc"] {
        SqlValue::Text(s) => s.clone(),
        _ => panic!(),
    };
    let enc_name = match &row[0]["name_enc"] {
        SqlValue::Text(s) => s.clone(),
        _ => panic!(),
    };
    assert_eq!(db.decrypt_field(enc_email).unwrap(), "alice@test.com");
    assert_eq!(db.decrypt_field(enc_name).unwrap(), "Alice");

    // Second row too
    let row = db
        .query_raw(
            "SELECT email_enc FROM secrets WHERE id = ?1 AND tenant_id = 'tenant_test'".into(),
            vec![SqlValue::Text(id2)],
        )
        .unwrap();
    let enc = match &row[0]["email_enc"] {
        SqlValue::Text(s) => s.clone(),
        _ => panic!(),
    };
    assert_eq!(db.decrypt_field(enc).unwrap(), "bob@test.com");
}

#[test]
fn old_key_cannot_decrypt_after_rotation() {
    let dir = tempfile::TempDir::new().unwrap();
    let path = dir.path().join("rotate.db").to_str().unwrap().to_string();
    let old_key = test_key();

    // Create and encrypt with old key
    let db = iron_vault_core::api::vault::IronVaultDb::open(
        path.clone(),
        old_key.clone(),
        "t".into(),
        VaultConfig::test_config(),
    )
    .unwrap();
    create_encrypted_table(&db);
    let id = insert_encrypted(&db, "secret@test.com", "Secret");

    // Rotate
    let new_key = vec![0xCCu8; 32];
    let mut tables = HashMap::new();
    tables.insert("secrets".into(), vec!["email_enc".into()]);
    db.rotate_field_keys(new_key.clone(), tables).unwrap();

    // Read the encrypted value with raw SQL
    let row = db
        .query_raw(
            format!("SELECT email_enc FROM secrets WHERE id = '{}'", id),
            vec![],
        )
        .unwrap();
    let ciphertext = match &row[0]["email_enc"] {
        SqlValue::Text(s) => s.clone(),
        _ => panic!(),
    };

    // Open a NEW db instance with the OLD key — should fail to decrypt
    drop(db);
    std::thread::sleep(std::time::Duration::from_millis(100));
    let old_db = iron_vault_core::api::vault::IronVaultDb::open(
        path,
        old_key,
        "t".into(),
        VaultConfig::test_config(),
    )
    .unwrap();
    let result = old_db.decrypt_field(ciphertext);
    assert!(
        result.is_err(),
        "Old key should not decrypt rotated ciphertext"
    );
}

// ─── Edge Cases ──────────────────────────────────────────────────────

#[test]
fn rotate_empty_table_returns_zero() {
    let dir = tempfile::TempDir::new().unwrap();
    let db = open_test_db(&dir);
    create_encrypted_table(&db);

    let new_key = vec![0xBBu8; 32];
    let mut tables = HashMap::new();
    tables.insert("secrets".into(), vec!["email_enc".into()]);
    let rotated = db.rotate_field_keys(new_key, tables).unwrap();
    assert_eq!(rotated, 0);
}

#[test]
fn rotate_with_null_columns_skipped() {
    let dir = tempfile::TempDir::new().unwrap();
    let db = open_test_db(&dir);
    create_encrypted_table(&db);

    // Insert with only email_enc, name_enc is NULL
    let enc = db.encrypt_field("test@t.com".into()).unwrap();
    let mut data = HashMap::new();
    data.insert("email_enc".into(), SqlValue::Text(enc));
    // name_enc not set → NULL
    db.query_insert("secrets".into(), data).unwrap();

    let new_key = vec![0xBBu8; 32];
    let mut tables = HashMap::new();
    tables.insert(
        "secrets".into(),
        vec!["email_enc".into(), "name_enc".into()],
    );
    let rotated = db.rotate_field_keys(new_key, tables).unwrap();
    assert_eq!(rotated, 1); // only email_enc rotated, name_enc was NULL
}

#[test]
fn rotate_wrong_key_length_errors() {
    let dir = tempfile::TempDir::new().unwrap();
    let db = open_test_db(&dir);

    let result = db.rotate_field_keys(vec![0u8; 16], HashMap::new());
    assert!(result.is_err());
}

#[test]
fn rotate_after_close_errors() {
    let dir = tempfile::TempDir::new().unwrap();
    let mut db = open_test_db(&dir);
    db.close().unwrap();

    let result = db.rotate_field_keys(vec![0xBBu8; 32], HashMap::new());
    assert!(result.is_err());
}

#[test]
fn encrypt_decrypt_works_after_rotation() {
    let dir = tempfile::TempDir::new().unwrap();
    let db = open_test_db(&dir);
    create_encrypted_table(&db);

    insert_encrypted(&db, "before@t.com", "Before");

    // Rotate
    let new_key = vec![0xDDu8; 32];
    let mut tables = HashMap::new();
    tables.insert(
        "secrets".into(),
        vec!["email_enc".into(), "name_enc".into()],
    );
    db.rotate_field_keys(new_key, tables).unwrap();

    // New encryptions should use the new key
    let enc = db.encrypt_field("after@t.com".into()).unwrap();
    let dec = db.decrypt_field(enc).unwrap();
    assert_eq!(dec, "after@t.com");
}

#[test]
fn rotate_multiple_tables() {
    let dir = tempfile::TempDir::new().unwrap();
    let db = open_test_db(&dir);
    create_encrypted_table(&db);
    db.execute_raw(
        "CREATE TABLE notes (\
            id TEXT PRIMARY KEY, body_enc TEXT, \
            tenant_id TEXT NOT NULL, created_at INTEGER NOT NULL, \
            updated_at INTEGER NOT NULL, deleted_at INTEGER)"
            .into(),
        vec![],
    )
    .unwrap();

    insert_encrypted(&db, "a@t.com", "Alice");
    let enc_note = db.encrypt_field("secret note".into()).unwrap();
    let mut note_data = HashMap::new();
    note_data.insert("body_enc".into(), SqlValue::Text(enc_note));
    db.query_insert("notes".into(), note_data).unwrap();

    let new_key = vec![0xEEu8; 32];
    let mut tables = HashMap::new();
    tables.insert(
        "secrets".into(),
        vec!["email_enc".into(), "name_enc".into()],
    );
    tables.insert("notes".into(), vec!["body_enc".into()]);
    let rotated = db.rotate_field_keys(new_key, tables).unwrap();
    assert_eq!(rotated, 3); // 2 from secrets + 1 from notes
}
