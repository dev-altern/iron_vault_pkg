use crate::common::*;
use iron_vault_core::api::crypto;
use iron_vault_core::api::types::*;
use std::collections::HashMap;

// ─── Argon2id Key Derivation (Standalone) ────────────────────────────

#[test]
fn derive_key_produces_32_bytes() {
    let salt = crypto::generate_salt();
    let key = crypto::derive_key("password".into(), salt, 1024, 1, 1).unwrap();
    assert_eq!(key.len(), 32);
}

#[test]
fn derive_key_deterministic() {
    let salt = vec![0x42u8; 32];
    let k1 = crypto::derive_key("test".into(), salt.clone(), 1024, 1, 1).unwrap();
    let k2 = crypto::derive_key("test".into(), salt, 1024, 1, 1).unwrap();
    assert_eq!(k1, k2);
}

#[test]
fn derive_key_different_passwords() {
    let salt = vec![0x42u8; 32];
    let k1 = crypto::derive_key("pw1".into(), salt.clone(), 1024, 1, 1).unwrap();
    let k2 = crypto::derive_key("pw2".into(), salt, 1024, 1, 1).unwrap();
    assert_ne!(k1, k2);
}

#[test]
fn derive_key_different_salts() {
    let k1 = crypto::derive_key("same".into(), vec![1u8; 32], 1024, 1, 1).unwrap();
    let k2 = crypto::derive_key("same".into(), vec![2u8; 32], 1024, 1, 1).unwrap();
    assert_ne!(k1, k2);
}

#[test]
fn derive_key_short_salt_rejected() {
    let result = crypto::derive_key("pw".into(), vec![0u8; 8], 1024, 1, 1);
    assert!(result.is_err());
}

#[test]
fn generate_salt_is_32_bytes_and_unique() {
    let s1 = crypto::generate_salt();
    let s2 = crypto::generate_salt();
    assert_eq!(s1.len(), 32);
    assert_eq!(s2.len(), 32);
    assert_ne!(s1, s2);
}

// ─── Static Field Encryption (Standalone) ────────────────────────────

#[test]
fn static_encrypt_decrypt_round_trip() {
    let key = vec![0x42u8; 32];
    let enc = crypto::encrypt_field_static("hello".into(), key.clone()).unwrap();
    let dec = crypto::decrypt_field_static(enc, key).unwrap();
    assert_eq!(dec, "hello");
}

#[test]
fn static_same_plaintext_different_ciphertext() {
    let key = vec![0x42u8; 32];
    let e1 = crypto::encrypt_field_static("test".into(), key.clone()).unwrap();
    let e2 = crypto::encrypt_field_static("test".into(), key).unwrap();
    assert_ne!(e1, e2, "Random nonce must produce different output");
}

#[test]
fn static_wrong_key_fails() {
    let e = crypto::encrypt_field_static("secret".into(), vec![1u8; 32]).unwrap();
    let result = crypto::decrypt_field_static(e, vec![2u8; 32]);
    assert!(result.is_err());
}

// ─── Instance Field Encryption (via IronVaultDb) ─────────────────────

#[test]
fn instance_encrypt_decrypt_round_trip() {
    let dir = tempfile::TempDir::new().unwrap();
    let db = open_test_db(&dir);

    let encrypted = db.encrypt_field("tom@altab.be".into()).unwrap();
    let decrypted = db.decrypt_field(encrypted).unwrap();
    assert_eq!(decrypted, "tom@altab.be");
}

#[test]
fn instance_different_ciphertext_each_call() {
    let dir = tempfile::TempDir::new().unwrap();
    let db = open_test_db(&dir);

    let e1 = db.encrypt_field("same".into()).unwrap();
    let e2 = db.encrypt_field("same".into()).unwrap();
    assert_ne!(e1, e2);
}

#[test]
fn instance_encrypt_unicode() {
    let dir = tempfile::TempDir::new().unwrap();
    let db = open_test_db(&dir);

    for text in &["café", "日本語", "🔐🗝️", "مرحبا"] {
        let enc = db.encrypt_field(text.to_string()).unwrap();
        let dec = db.decrypt_field(enc).unwrap();
        assert_eq!(&dec, text);
    }
}

#[test]
fn instance_encrypt_empty_string() {
    let dir = tempfile::TempDir::new().unwrap();
    let db = open_test_db(&dir);

    let enc = db.encrypt_field("".into()).unwrap();
    let dec = db.decrypt_field(enc).unwrap();
    assert_eq!(dec, "");
}

#[test]
fn instance_encrypt_after_close_fails() {
    let dir = tempfile::TempDir::new().unwrap();
    let mut db = open_test_db(&dir);
    db.close().unwrap();

    assert!(db.encrypt_field("test".into()).is_err());
    assert!(db.decrypt_field("{}".into()).is_err());
}

// ─── Tenant-Scoped Encryption ────────────────────────────────────────

#[test]
fn different_tenants_produce_different_ciphertext_keys() {
    let dir = tempfile::TempDir::new().unwrap();
    let path = dir.path().join("shared.db").to_str().unwrap().to_string();

    let db_a = iron_vault_core::api::vault::IronVaultDb::open(
        path.clone(), test_key(), "tenant_a".into(), VaultConfig::test_config(),
    ).unwrap();
    let db_b = iron_vault_core::api::vault::IronVaultDb::open(
        path, test_key(), "tenant_b".into(), VaultConfig::test_config(),
    ).unwrap();

    let enc_a = db_a.encrypt_field("secret".into()).unwrap();
    let enc_b = db_b.encrypt_field("secret".into()).unwrap();

    // Different tenants → different keys → tenant A can't decrypt tenant B's data
    let result = db_a.decrypt_field(enc_b);
    assert!(result.is_err(), "Tenant A should not decrypt tenant B's ciphertext");

    let result = db_b.decrypt_field(enc_a);
    assert!(result.is_err(), "Tenant B should not decrypt tenant A's ciphertext");
}

// ─── HKDF Key Isolation ──────────────────────────────────────────────

#[test]
fn derive_purpose_key_different_purposes() {
    let dir = tempfile::TempDir::new().unwrap();
    let db = open_test_db(&dir);

    let k1 = db.derive_purpose_key("sqlcipher".into()).unwrap();
    let k2 = db.derive_purpose_key("audit_hmac".into()).unwrap();
    let k3 = db.derive_purpose_key("backup".into()).unwrap();

    assert_eq!(k1.len(), 32);
    assert_ne!(k1, k2);
    assert_ne!(k1, k3);
    assert_ne!(k2, k3);
}

#[test]
fn derive_purpose_key_deterministic() {
    let dir = tempfile::TempDir::new().unwrap();
    let db = open_test_db(&dir);

    let k1 = db.derive_purpose_key("test_purpose".into()).unwrap();
    let k2 = db.derive_purpose_key("test_purpose".into()).unwrap();
    assert_eq!(k1, k2);
}

// ─── End-to-End: Encrypt, Store, Retrieve, Decrypt ──────────────────

#[test]
fn encrypt_store_retrieve_decrypt_cycle() {
    let dir = tempfile::TempDir::new().unwrap();
    let db = open_test_db(&dir);
    create_users_table(&db);

    let email = "alice@example.com";
    let encrypted_email = db.encrypt_field(email.into()).unwrap();

    // Store encrypted value in DB
    let mut data = HashMap::new();
    data.insert("name".into(), SqlValue::Text("Alice".into()));
    data.insert("email".into(), SqlValue::Text(encrypted_email.clone()));
    data.insert("status".into(), SqlValue::Text("active".into()));
    let id = db.query_insert("users".into(), data).unwrap();

    // Retrieve and decrypt
    let mut spec = query("users");
    spec.conditions.push(Condition::Eq {
        column: "id".into(),
        value: SqlValue::Text(id),
    });
    let row = db.query_first(spec).unwrap().unwrap();
    let stored = match row.get("email") {
        Some(SqlValue::Text(s)) => s.clone(),
        _ => panic!("Expected text email"),
    };

    let decrypted = db.decrypt_field(stored).unwrap();
    assert_eq!(decrypted, email);
}

#[test]
fn tampered_ciphertext_fails_on_decrypt() {
    let dir = tempfile::TempDir::new().unwrap();
    let db = open_test_db(&dir);

    let enc = db.encrypt_field("secret".into()).unwrap();
    let tampered = enc.replace("ct\":\"", "ct\":\"AAAA");
    let result = db.decrypt_field(tampered);
    assert!(result.is_err());
}

#[test]
fn invalid_json_fails_on_decrypt() {
    let dir = tempfile::TempDir::new().unwrap();
    let db = open_test_db(&dir);

    assert!(db.decrypt_field("not json at all".into()).is_err());
    assert!(db.decrypt_field("{}".into()).is_err());
}
