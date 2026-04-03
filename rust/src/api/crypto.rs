use crate::engine::crypto;
use anyhow::Result;

/// Derive a 32-byte encryption key from a password and salt using Argon2id.
///
/// This is CPU-intensive by design — call from a background isolate in Dart.
///
/// Recommended production parameters:
/// - `memory_kb`: 65536 (64MB)
/// - `iterations`: 3
/// - `parallelism`: 4
///
/// The salt must be at least 16 bytes (32 recommended). Generate with `generate_salt()`.
pub fn derive_key(
    password: String,
    salt: Vec<u8>,
    memory_kb: u32,
    iterations: u32,
    parallelism: u32,
) -> Result<Vec<u8>> {
    crypto::derive_key(&password, &salt, memory_kb, iterations, parallelism)
}

/// Generate 32 cryptographically secure random bytes.
///
/// Use for encryption salt (store in platform secure storage)
/// or as a random key.
#[flutter_rust_bridge::frb(sync)]
pub fn generate_salt() -> Vec<u8> {
    crypto::generate_salt()
}

/// Encrypt a plaintext string using AES-256-GCM with a provided key.
///
/// Standalone version — does not require an open database.
/// Key must be exactly 32 bytes. Each call produces different ciphertext.
/// Returns JSON: `{"ct":"<base64>","nonce":"<base64>","kid":"v1"}`.
pub fn encrypt_field_static(plaintext: String, key: Vec<u8>) -> Result<String> {
    crypto::encrypt_field(&plaintext, &key)
}

/// Decrypt a ciphertext JSON string using AES-256-GCM with a provided key.
///
/// Standalone version — does not require an open database.
/// Key must be exactly 32 bytes.
pub fn decrypt_field_static(ciphertext_json: String, key: Vec<u8>) -> Result<String> {
    crypto::decrypt_field(&ciphertext_json, &key)
}
