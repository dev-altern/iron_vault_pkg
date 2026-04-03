use aes_gcm::aead::{Aead, KeyInit};
use aes_gcm::{Aes256Gcm, Nonce};
use anyhow::{anyhow, Context, Result};
use argon2::Argon2;
use base64::Engine as _;
use hkdf::Hkdf;
use rand::RngCore;
use sha2::Sha256;
use zeroize::Zeroizing;

// ─── Argon2id Key Derivation ─────────────────────────────────────────

/// Derive a 32-byte encryption key from a password + salt using Argon2id.
///
/// This is CPU-intensive (by design) — call on a background thread.
/// Production params: memory_kb=65536, iterations=3, parallelism=4.
pub(crate) fn derive_key(
    password: &str,
    salt: &[u8],
    memory_kb: u32,
    iterations: u32,
    parallelism: u32,
) -> Result<Vec<u8>> {
    if salt.len() < 16 {
        return Err(anyhow!(
            "EncryptionException: salt must be at least 16 bytes, got {}",
            salt.len()
        ));
    }

    let params = argon2::Params::new(memory_kb, iterations, parallelism, Some(32))
        .map_err(|e| anyhow!("EncryptionException: invalid Argon2id params: {}", e))?;

    let argon2 = Argon2::new(argon2::Algorithm::Argon2id, argon2::Version::V0x13, params);

    let mut key = Zeroizing::new(vec![0u8; 32]);
    argon2
        .hash_password_into(password.as_bytes(), salt, &mut key)
        .map_err(|e| anyhow!("EncryptionException: Argon2id derivation failed: {}", e))?;

    Ok(key.to_vec())
}

/// Generate 32 cryptographically secure random bytes (for salt or keys).
pub(crate) fn generate_salt() -> Vec<u8> {
    let mut salt = vec![0u8; 32];
    rand::thread_rng().fill_bytes(&mut salt);
    salt
}

// ─── HKDF Key Isolation ──────────────────────────────────────────────

/// Derive a purpose-specific 32-byte key from a master key using HKDF-SHA256.
///
/// The `info` parameter scopes the derived key to a specific purpose
/// (e.g., "sqlcipher", "tenant:abc", "audit_hmac", "backup").
pub(crate) fn hkdf_derive(master_key: &[u8], info: &str) -> Result<Vec<u8>> {
    if master_key.len() != 32 {
        return Err(anyhow!(
            "EncryptionException: master key must be 32 bytes, got {}",
            master_key.len()
        ));
    }

    let hk = Hkdf::<Sha256>::new(None, master_key);
    let mut derived = Zeroizing::new(vec![0u8; 32]);
    hk.expand(info.as_bytes(), &mut derived)
        .map_err(|e| anyhow!("EncryptionException: HKDF expansion failed: {}", e))?;

    Ok(derived.to_vec())
}

/// Derive the tenant-scoped field encryption key.
pub(crate) fn tenant_field_key(master_key: &[u8], tenant_id: &str) -> Result<Vec<u8>> {
    hkdf_derive(master_key, &format!("tenant:{}", tenant_id))
}

/// Derive the audit HMAC signing key.
#[allow(dead_code)]
pub(crate) fn audit_hmac_key(master_key: &[u8]) -> Result<Vec<u8>> {
    hkdf_derive(master_key, "audit_hmac")
}

/// Derive the backup encryption key.
#[allow(dead_code)]
pub(crate) fn backup_key(master_key: &[u8]) -> Result<Vec<u8>> {
    hkdf_derive(master_key, "backup")
}

// ─── AES-256-GCM Field Encryption ───────────────────────────────────

/// Encrypt a plaintext string using AES-256-GCM with a random nonce.
///
/// Returns a JSON string: `{"ct":"<base64>","nonce":"<base64>","kid":"v1"}`
/// Each call produces different ciphertext due to the random nonce.
pub(crate) fn encrypt_field(plaintext: &str, key: &[u8]) -> Result<String> {
    if key.len() != 32 {
        return Err(anyhow!(
            "EncryptionException: field key must be 32 bytes, got {}",
            key.len()
        ));
    }

    let cipher = Aes256Gcm::new_from_slice(key)
        .map_err(|e| anyhow!("EncryptionException: invalid key: {}", e))?;

    let mut nonce_bytes = [0u8; 12];
    rand::thread_rng().fill_bytes(&mut nonce_bytes);
    let nonce = Nonce::from_slice(&nonce_bytes);

    let ciphertext = cipher
        .encrypt(nonce, plaintext.as_bytes())
        .map_err(|e| anyhow!("EncryptionException: encryption failed: {}", e))?;

    let b64 = base64::engine::general_purpose::STANDARD;
    let json = format!(
        "{{\"ct\":\"{}\",\"nonce\":\"{}\",\"kid\":\"v1\"}}",
        b64.encode(&ciphertext),
        b64.encode(nonce_bytes),
    );
    Ok(json)
}

/// Decrypt a ciphertext string that was produced by `encrypt_field`.
///
/// Expects JSON: `{"ct":"<base64>","nonce":"<base64>","kid":"..."}`
/// Returns the original plaintext.
pub(crate) fn decrypt_field(ciphertext_json: &str, key: &[u8]) -> Result<String> {
    if key.len() != 32 {
        return Err(anyhow!(
            "EncryptionException: field key must be 32 bytes, got {}",
            key.len()
        ));
    }

    // Parse the JSON manually (avoid serde dependency for this simple format)
    let ct_b64 = extract_json_field(ciphertext_json, "ct")
        .context("EncryptionException: missing 'ct' field in ciphertext JSON")?;
    let nonce_b64 = extract_json_field(ciphertext_json, "nonce")
        .context("EncryptionException: missing 'nonce' field in ciphertext JSON")?;

    let b64 = base64::engine::general_purpose::STANDARD;
    let ct_bytes = b64
        .decode(&ct_b64)
        .context("EncryptionException: invalid base64 in ct")?;
    let nonce_bytes = b64
        .decode(&nonce_b64)
        .context("EncryptionException: invalid base64 in nonce")?;

    if nonce_bytes.len() != 12 {
        return Err(anyhow!(
            "EncryptionException: nonce must be 12 bytes, got {}",
            nonce_bytes.len()
        ));
    }

    let cipher = Aes256Gcm::new_from_slice(key)
        .map_err(|e| anyhow!("EncryptionException: invalid key: {}", e))?;
    let nonce = Nonce::from_slice(&nonce_bytes);

    let plaintext_bytes = cipher.decrypt(nonce, ct_bytes.as_ref()).map_err(|_| {
        anyhow!("EncryptionException: decryption failed (wrong key or tampered data)")
    })?;

    String::from_utf8(plaintext_bytes)
        .context("EncryptionException: decrypted data is not valid UTF-8")
}

/// Extract a string value from simple JSON like `{"key":"value","key2":"value2"}`.
fn extract_json_field(json: &str, field: &str) -> Option<String> {
    let pattern = format!("\"{}\":\"", field);
    let start = json.find(&pattern)? + pattern.len();
    let end = json[start..].find('"')? + start;
    Some(json[start..end].to_string())
}

// ─── Unit Tests ──────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;

    // ── Argon2id ─────────────────────────────────────────────────

    #[test]
    fn derive_key_produces_32_bytes() {
        let salt = generate_salt();
        let key = derive_key("password", &salt, 1024, 1, 1).unwrap();
        assert_eq!(key.len(), 32);
    }

    #[test]
    fn derive_key_deterministic_for_same_inputs() {
        let salt = vec![0x42u8; 32];
        let k1 = derive_key("test", &salt, 1024, 1, 1).unwrap();
        let k2 = derive_key("test", &salt, 1024, 1, 1).unwrap();
        assert_eq!(k1, k2);
    }

    #[test]
    fn derive_key_different_passwords_different_keys() {
        let salt = vec![0x42u8; 32];
        let k1 = derive_key("password1", &salt, 1024, 1, 1).unwrap();
        let k2 = derive_key("password2", &salt, 1024, 1, 1).unwrap();
        assert_ne!(k1, k2);
    }

    #[test]
    fn derive_key_different_salts_different_keys() {
        let k1 = derive_key("same", &[1u8; 32], 1024, 1, 1).unwrap();
        let k2 = derive_key("same", &[2u8; 32], 1024, 1, 1).unwrap();
        assert_ne!(k1, k2);
    }

    #[test]
    fn derive_key_short_salt_rejected() {
        let result = derive_key("pw", &[0u8; 8], 1024, 1, 1);
        assert!(result.is_err());
    }

    #[test]
    fn generate_salt_is_32_bytes() {
        let salt = generate_salt();
        assert_eq!(salt.len(), 32);
    }

    #[test]
    fn generate_salt_not_all_zeros() {
        let salt = generate_salt();
        assert!(salt.iter().any(|&b| b != 0), "Salt should not be all zeros");
    }

    #[test]
    fn generate_salt_unique_each_call() {
        let s1 = generate_salt();
        let s2 = generate_salt();
        assert_ne!(s1, s2);
    }

    // ── HKDF ─────────────────────────────────────────────────────

    #[test]
    fn hkdf_produces_32_bytes() {
        let master = vec![0xABu8; 32];
        let derived = hkdf_derive(&master, "test").unwrap();
        assert_eq!(derived.len(), 32);
    }

    #[test]
    fn hkdf_different_info_different_keys() {
        let master = vec![0xABu8; 32];
        let k1 = hkdf_derive(&master, "sqlcipher").unwrap();
        let k2 = hkdf_derive(&master, "tenant:abc").unwrap();
        let k3 = hkdf_derive(&master, "audit_hmac").unwrap();
        let k4 = hkdf_derive(&master, "backup").unwrap();
        assert_ne!(k1, k2);
        assert_ne!(k1, k3);
        assert_ne!(k1, k4);
        assert_ne!(k2, k3);
        assert_ne!(k2, k4);
        assert_ne!(k3, k4);
    }

    #[test]
    fn hkdf_deterministic() {
        let master = vec![0xABu8; 32];
        let k1 = hkdf_derive(&master, "test").unwrap();
        let k2 = hkdf_derive(&master, "test").unwrap();
        assert_eq!(k1, k2);
    }

    #[test]
    fn hkdf_wrong_key_length_rejected() {
        assert!(hkdf_derive(&[0u8; 16], "test").is_err());
        assert!(hkdf_derive(&[0u8; 64], "test").is_err());
    }

    #[test]
    fn tenant_keys_differ_per_tenant() {
        let master = vec![0xABu8; 32];
        let k1 = tenant_field_key(&master, "tenant_a").unwrap();
        let k2 = tenant_field_key(&master, "tenant_b").unwrap();
        assert_ne!(k1, k2);
    }

    // ── AES-256-GCM ──────────────────────────────────────────────

    #[test]
    fn encrypt_decrypt_round_trip() {
        let key = vec![0x42u8; 32];
        let plaintext = "hello@example.com";
        let encrypted = encrypt_field(plaintext, &key).unwrap();
        let decrypted = decrypt_field(&encrypted, &key).unwrap();
        assert_eq!(decrypted, plaintext);
    }

    #[test]
    fn same_plaintext_different_ciphertext() {
        let key = vec![0x42u8; 32];
        let enc1 = encrypt_field("test", &key).unwrap();
        let enc2 = encrypt_field("test", &key).unwrap();
        assert_ne!(
            enc1, enc2,
            "Random nonce should produce different ciphertext"
        );
    }

    #[test]
    fn wrong_key_fails_decrypt() {
        let key1 = vec![0x01u8; 32];
        let key2 = vec![0x02u8; 32];
        let encrypted = encrypt_field("secret", &key1).unwrap();
        let result = decrypt_field(&encrypted, &key2);
        assert!(result.is_err());
        assert!(result
            .unwrap_err()
            .to_string()
            .contains("EncryptionException"));
    }

    #[test]
    fn encrypt_empty_string() {
        let key = vec![0x42u8; 32];
        let encrypted = encrypt_field("", &key).unwrap();
        let decrypted = decrypt_field(&encrypted, &key).unwrap();
        assert_eq!(decrypted, "");
    }

    #[test]
    fn encrypt_unicode() {
        let key = vec![0x42u8; 32];
        let texts = ["café", "日本語", "🔐🗝️", "مرحبا", "line1\nline2\ttab"];
        for text in &texts {
            let encrypted = encrypt_field(text, &key).unwrap();
            let decrypted = decrypt_field(&encrypted, &key).unwrap();
            assert_eq!(&decrypted, text, "Unicode round-trip failed for {:?}", text);
        }
    }

    #[test]
    fn encrypt_large_data() {
        let key = vec![0x42u8; 32];
        let large = "x".repeat(1_000_000); // 1MB
        let encrypted = encrypt_field(&large, &key).unwrap();
        let decrypted = decrypt_field(&encrypted, &key).unwrap();
        assert_eq!(decrypted, large);
    }

    #[test]
    fn encrypted_output_is_valid_json() {
        let key = vec![0x42u8; 32];
        let encrypted = encrypt_field("test", &key).unwrap();
        assert!(encrypted.starts_with('{'));
        assert!(encrypted.ends_with('}'));
        assert!(encrypted.contains("\"ct\":\""));
        assert!(encrypted.contains("\"nonce\":\""));
        assert!(encrypted.contains("\"kid\":\"v1\""));
    }

    #[test]
    fn tampered_ciphertext_fails() {
        let key = vec![0x42u8; 32];
        let encrypted = encrypt_field("test", &key).unwrap();
        // Flip a character in the ciphertext
        let tampered = encrypted.replacen("ct\":\"", "ct\":\"A", 1);
        let result = decrypt_field(&tampered, &key);
        // Should fail (either base64 decode or GCM auth tag mismatch)
        assert!(result.is_err());
    }

    #[test]
    fn invalid_json_fails_decrypt() {
        let key = vec![0x42u8; 32];
        assert!(decrypt_field("not json", &key).is_err());
        assert!(decrypt_field("{}", &key).is_err());
        assert!(decrypt_field("{\"ct\":\"abc\"}", &key).is_err()); // missing nonce
    }

    #[test]
    fn wrong_key_length_rejected() {
        assert!(encrypt_field("test", &[0u8; 16]).is_err());
        assert!(decrypt_field("{}", &[0u8; 16]).is_err());
    }
}
