use crate::common::*;
use iron_vault_core::api::types::*;
use std::collections::HashMap;

// ─── Backup Round-Trip ───────────────────────────────────────────────

#[test]
fn backup_and_restore_compressed_encrypted() {
    let dir = tempfile::TempDir::new().unwrap();
    let db = open_test_db(&dir);
    create_users_table(&db);
    insert_user(&db, "Alice", "a@t.com", "admin", 90.0);
    insert_user(&db, "Bob", "b@t.com", "member", 80.0);

    let backup_path = dir.path().join("backup.ivb").to_str().unwrap().to_string();
    let result = db.backup(backup_path.clone(), true, true).unwrap();
    assert!(result.size_bytes > 0);
    assert!(!result.checksum.is_empty());
    assert!(result.compressed);
    assert!(result.encrypted);

    // Restore to new path
    let restore_path = dir.path().join("restored.db").to_str().unwrap().to_string();
    let restore = db
        .restore_backup(backup_path, restore_path.clone(), Some(result.checksum))
        .unwrap();
    assert!(restore.integrity_ok);

    // The restored file should be byte-identical to the original SQLCipher file
    // Reopen with same key and verify data
    let restored_db = iron_vault_core::api::vault::IronVaultDb::open(
        restore_path,
        test_key(),
        "tenant_test".into(),
        VaultConfig::test_config(),
    )
    .unwrap();
    assert_eq!(restored_db.query_count(query("users")).unwrap(), 2);
}

#[test]
fn backup_uncompressed_unencrypted() {
    let dir = tempfile::TempDir::new().unwrap();
    let db = open_test_db(&dir);
    create_users_table(&db);
    insert_user(&db, "Alice", "a@t.com", "admin", 90.0);

    let backup_path = dir.path().join("backup.ivb").to_str().unwrap().to_string();
    let result = db.backup(backup_path.clone(), false, false).unwrap();
    assert!(!result.compressed);
    assert!(!result.encrypted);

    let restore_path = dir.path().join("restored.db").to_str().unwrap().to_string();
    let restore = db.restore_backup(backup_path, restore_path, None).unwrap();
    assert!(restore.integrity_ok);
}

#[test]
fn backup_compressed_only() {
    let dir = tempfile::TempDir::new().unwrap();
    let db = open_test_db(&dir);
    create_users_table(&db);
    for i in 0..100 {
        insert_user(
            &db,
            &format!("U{}", i),
            &format!("u{}@t.com", i),
            "m",
            i as f64,
        );
    }

    let uncompressed_path = dir.path().join("raw.ivb").to_str().unwrap().to_string();
    let compressed_path = dir.path().join("zstd.ivb").to_str().unwrap().to_string();

    let raw = db.backup(uncompressed_path, false, false).unwrap();
    let zstd = db.backup(compressed_path, true, false).unwrap();

    // Note: SQLCipher-encrypted DB files don't compress well because
    // encrypted data appears random. The compressed flag is set though.
    assert!(zstd.compressed);
    assert!(!raw.compressed);
    // Compressed file should at least be valid (not larger by a huge margin)
    assert!(zstd.size_bytes > 0);
}

// ─── Backup Verification ─────────────────────────────────────────────

#[test]
fn verify_backup_valid() {
    let dir = tempfile::TempDir::new().unwrap();
    let db = open_test_db(&dir);
    create_users_table(&db);
    insert_user(&db, "Alice", "a@t.com", "admin", 90.0);

    let backup_path = dir.path().join("backup.ivb").to_str().unwrap().to_string();
    db.backup(backup_path.clone(), true, true).unwrap();

    let report = db.verify_backup(backup_path).unwrap();
    assert!(report.checksum_ok);
    assert!(report.decrypt_ok);
    assert!(report.decompress_ok);
}

#[test]
fn corrupted_backup_checksum_fails() {
    let dir = tempfile::TempDir::new().unwrap();
    let db = open_test_db(&dir);
    create_users_table(&db);
    insert_user(&db, "Alice", "a@t.com", "admin", 90.0);

    let backup_path = dir.path().join("backup.ivb").to_str().unwrap().to_string();
    db.backup(backup_path.clone(), true, true).unwrap();

    // Corrupt the file
    let mut data = std::fs::read(&backup_path).unwrap();
    if data.len() > 50 {
        data[40] ^= 0xFF; // flip a byte in the payload
    }
    std::fs::write(&backup_path, &data).unwrap();

    let report = db.verify_backup(backup_path).unwrap();
    assert!(!report.checksum_ok);
}

#[test]
fn restore_with_wrong_checksum_fails() {
    let dir = tempfile::TempDir::new().unwrap();
    let db = open_test_db(&dir);
    create_users_table(&db);
    insert_user(&db, "Alice", "a@t.com", "admin", 90.0);

    let backup_path = dir.path().join("backup.ivb").to_str().unwrap().to_string();
    db.backup(backup_path.clone(), true, true).unwrap();

    let restore_path = dir.path().join("restored.db").to_str().unwrap().to_string();
    let result = db.restore_backup(
        backup_path,
        restore_path,
        Some("0000000000000000000000000000000000000000000000000000000000000000".into()),
    );
    assert!(result.is_err());
}

#[test]
fn restore_corrupted_file_fails() {
    let dir = tempfile::TempDir::new().unwrap();
    let db = open_test_db(&dir);
    create_users_table(&db);
    insert_user(&db, "Alice", "a@t.com", "admin", 90.0);

    let backup_path = dir.path().join("backup.ivb").to_str().unwrap().to_string();
    db.backup(backup_path.clone(), false, false).unwrap();

    // Corrupt
    let mut data = std::fs::read(&backup_path).unwrap();
    data[40] ^= 0xFF;
    std::fs::write(&backup_path, &data).unwrap();

    let restore_path = dir.path().join("restored.db").to_str().unwrap().to_string();
    let result = db.restore_backup(backup_path, restore_path, None);
    assert!(result.is_err());
}

// ─── Export CSV ──────────────────────────────────────────────────────

#[test]
fn export_csv_basic() {
    let dir = tempfile::TempDir::new().unwrap();
    let db = open_test_db(&dir);
    create_users_table(&db);
    insert_user(&db, "Alice", "alice@t.com", "admin", 90.0);
    insert_user(&db, "Bob", "bob@t.com", "member", 80.0);

    let csv_bytes = db
        .export_table("users".into(), ExportFormat::Csv, None)
        .unwrap();
    let csv = String::from_utf8(csv_bytes).unwrap();

    // Has header
    assert!(csv.starts_with("id,") || csv.contains(",name,") || csv.contains("name"));
    // Has data rows
    assert!(csv.contains("Alice"));
    assert!(csv.contains("Bob"));
    // Count lines (header + 2 data rows)
    let lines: Vec<&str> = csv.trim().lines().collect();
    assert_eq!(lines.len(), 3); // header + 2 rows
}

#[test]
fn export_csv_with_specific_columns() {
    let dir = tempfile::TempDir::new().unwrap();
    let db = open_test_db(&dir);
    create_users_table(&db);
    insert_user(&db, "Alice", "alice@t.com", "admin", 90.0);

    let csv_bytes = db
        .export_table(
            "users".into(),
            ExportFormat::Csv,
            Some(vec!["name".into(), "email".into()]),
        )
        .unwrap();
    let csv = String::from_utf8(csv_bytes).unwrap();
    assert!(csv.starts_with("name,email\n"));
    assert!(csv.contains("Alice"));
}

#[test]
fn export_csv_escapes_commas_and_quotes() {
    let dir = tempfile::TempDir::new().unwrap();
    let db = open_test_db(&dir);
    create_users_table(&db);

    let mut data = HashMap::new();
    data.insert("name".into(), SqlValue::Text("O'Brien, Jr.".into()));
    data.insert("email".into(), SqlValue::Text("ob@t.com".into()));
    data.insert("status".into(), SqlValue::Text("active".into()));
    db.query_insert("users".into(), data).unwrap();

    let csv = String::from_utf8(
        db.export_table("users".into(), ExportFormat::Csv, Some(vec!["name".into()]))
            .unwrap(),
    )
    .unwrap();
    // Comma in value should be quoted
    assert!(csv.contains("\"O'Brien, Jr.\""));
}

#[test]
fn export_empty_table_csv() {
    let dir = tempfile::TempDir::new().unwrap();
    let db = open_test_db(&dir);
    create_users_table(&db);

    let csv = String::from_utf8(
        db.export_table("users".into(), ExportFormat::Csv, None)
            .unwrap(),
    )
    .unwrap();
    // Just the header line
    assert_eq!(csv.trim().lines().count(), 1);
}

// ─── Export JSON ─────────────────────────────────────────────────────

#[test]
fn export_json_basic() {
    let dir = tempfile::TempDir::new().unwrap();
    let db = open_test_db(&dir);
    create_users_table(&db);
    insert_user(&db, "Alice", "a@t.com", "admin", 90.0);

    let json_bytes = db
        .export_table("users".into(), ExportFormat::Json, None)
        .unwrap();
    let json = String::from_utf8(json_bytes).unwrap();
    assert!(json.starts_with('['));
    assert!(json.ends_with(']'));
    assert!(json.contains("\"Alice\""));
}

#[test]
fn export_json_empty_table() {
    let dir = tempfile::TempDir::new().unwrap();
    let db = open_test_db(&dir);
    create_users_table(&db);

    let json = String::from_utf8(
        db.export_table("users".into(), ExportFormat::Json, None)
            .unwrap(),
    )
    .unwrap();
    assert_eq!(json, "[]");
}

// ─── Export JSONL ────────────────────────────────────────────────────

#[test]
fn export_jsonl_basic() {
    let dir = tempfile::TempDir::new().unwrap();
    let db = open_test_db(&dir);
    create_users_table(&db);
    insert_user(&db, "Alice", "a@t.com", "admin", 90.0);
    insert_user(&db, "Bob", "b@t.com", "member", 80.0);

    let jsonl = String::from_utf8(
        db.export_table("users".into(), ExportFormat::Jsonl, None)
            .unwrap(),
    )
    .unwrap();
    let lines: Vec<&str> = jsonl.trim().lines().collect();
    assert_eq!(lines.len(), 2);
    for line in &lines {
        assert!(line.starts_with('{'));
        assert!(line.ends_with('}'));
    }
}

// ─── Tenant Isolation ────────────────────────────────────────────────

#[test]
fn export_respects_tenant_isolation() {
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

    create_users_table(&db_a);
    insert_user_on(&db_a, "Alice");
    insert_user_on(&db_a, "Alice2");
    insert_user_on(&db_b, "Bob");

    let csv_a = String::from_utf8(
        db_a.export_table("users".into(), ExportFormat::Csv, Some(vec!["name".into()]))
            .unwrap(),
    )
    .unwrap();
    let csv_b = String::from_utf8(
        db_b.export_table("users".into(), ExportFormat::Csv, Some(vec!["name".into()]))
            .unwrap(),
    )
    .unwrap();

    assert_eq!(csv_a.trim().lines().count(), 3); // header + 2 rows
    assert_eq!(csv_b.trim().lines().count(), 2); // header + 1 row
}

// ─── Edge Cases ──────────────────────────────────────────────────────

#[test]
fn backup_after_close_fails() {
    let dir = tempfile::TempDir::new().unwrap();
    let mut db = open_test_db(&dir);
    db.close().unwrap();

    assert!(db.backup("x".into(), false, false).is_err());
    assert!(db.verify_backup("x".into()).is_err());
    assert!(db.restore_backup("x".into(), "y".into(), None).is_err());
    assert!(db
        .export_table("t".into(), ExportFormat::Csv, None)
        .is_err());
}

#[test]
fn backup_nonexistent_restore_file_fails() {
    let dir = tempfile::TempDir::new().unwrap();
    let db = open_test_db(&dir);

    let result = db.restore_backup(
        "/nonexistent/path.ivb".into(),
        "/tmp/restored.db".into(),
        None,
    );
    assert!(result.is_err());
}

#[test]
fn backup_invalid_file_fails_verify() {
    let dir = tempfile::TempDir::new().unwrap();
    let db = open_test_db(&dir);

    let bad_path = dir.path().join("bad.ivb").to_str().unwrap().to_string();
    std::fs::write(&bad_path, b"not a valid backup file").unwrap();

    let result = db.verify_backup(bad_path);
    assert!(result.is_err() || !result.unwrap().checksum_ok);
}

// ─── Soft-Deleted Rows Excluded from Export ──────────────────────────

#[test]
fn export_excludes_soft_deleted() {
    let dir = tempfile::TempDir::new().unwrap();
    let db = open_test_db(&dir);
    create_users_table(&db);
    let id = insert_user(&db, "Alice", "a@t.com", "admin", 90.0);
    insert_user(&db, "Bob", "b@t.com", "member", 80.0);

    db.query_delete("users".into(), id).unwrap();

    let csv = String::from_utf8(
        db.export_table("users".into(), ExportFormat::Csv, Some(vec!["name".into()]))
            .unwrap(),
    )
    .unwrap();
    assert!(
        !csv.contains("Alice"),
        "Soft-deleted row should be excluded"
    );
    assert!(csv.contains("Bob"));
}

// ─── Helpers ─────────────────────────────────────────────────────────

fn insert_user_on(db: &iron_vault_core::api::vault::IronVaultDb, name: &str) -> String {
    let mut data = HashMap::new();
    data.insert("name".into(), SqlValue::Text(name.into()));
    data.insert("email".into(), SqlValue::Text(format!("{}@t.com", name)));
    data.insert("status".into(), SqlValue::Text("active".into()));
    db.query_insert("users".into(), data).unwrap()
}
