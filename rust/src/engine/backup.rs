use crate::api::types::{BackupResult, BackupVerifyReport, RestoreResult};
use aes_gcm::aead::{Aead, KeyInit};
use aes_gcm::{Aes256Gcm, Nonce};
use anyhow::{anyhow, Context, Result};
use rand::RngCore;
use std::path::Path;

const MAGIC: &[u8; 4] = b"IRON";
const VERSION: u32 = 1;
const NONCE_LEN: usize = 12;
// Header: magic(4) + version(4) + flags(1) + nonce(12) + uncompressed_size(8) = 29 bytes
const HEADER_LEN: usize = 4 + 4 + 1 + NONCE_LEN + 8;
const FLAG_COMPRESSED: u8 = 0x01;
const FLAG_ENCRYPTED: u8 = 0x02;

/// Create a backup of the database.
pub(crate) fn create_backup(
    db_path: &str,
    output_path: &str,
    compress: bool,
    encrypt: bool,
    backup_key: &[u8],
) -> Result<BackupResult> {
    let raw_db = std::fs::read(db_path)
        .with_context(|| format!("BackupException: cannot read database {}", db_path))?;
    let uncompressed_size = raw_db.len() as u64;

    // Compress
    let data = if compress {
        zstd::encode_all(raw_db.as_slice(), 3)
            .context("BackupException: zstd compression failed")?
    } else {
        raw_db
    };

    // Encrypt
    let mut nonce_bytes = [0u8; NONCE_LEN];
    let final_data = if encrypt {
        if backup_key.len() != 32 {
            return Err(anyhow!("BackupException: key must be 32 bytes"));
        }
        rand::thread_rng().fill_bytes(&mut nonce_bytes);
        let cipher = Aes256Gcm::new_from_slice(backup_key)
            .map_err(|e| anyhow!("BackupException: invalid key: {}", e))?;
        cipher
            .encrypt(Nonce::from_slice(&nonce_bytes), data.as_ref())
            .map_err(|e| anyhow!("BackupException: encryption failed: {}", e))?
    } else {
        data
    };

    // Build binary format
    let mut flags: u8 = 0;
    if compress {
        flags |= FLAG_COMPRESSED;
    }
    if encrypt {
        flags |= FLAG_ENCRYPTED;
    }

    let mut file_data = Vec::with_capacity(HEADER_LEN + final_data.len() + 32);
    file_data.extend_from_slice(MAGIC);
    file_data.extend_from_slice(&VERSION.to_le_bytes());
    file_data.push(flags);
    file_data.extend_from_slice(&nonce_bytes);
    file_data.extend_from_slice(&uncompressed_size.to_le_bytes());
    file_data.extend_from_slice(&final_data);

    let checksum = blake3::hash(&file_data);
    let checksum_hex = checksum.to_hex().to_string();
    file_data.extend_from_slice(checksum.as_bytes());

    if let Some(parent) = Path::new(output_path).parent() {
        if !parent.as_os_str().is_empty() {
            std::fs::create_dir_all(parent)?;
        }
    }
    std::fs::write(output_path, &file_data)
        .with_context(|| format!("BackupException: failed to write {}", output_path))?;

    Ok(BackupResult {
        path: output_path.to_string(),
        size_bytes: file_data.len() as u64,
        checksum: checksum_hex,
        compressed: compress,
        encrypted: encrypt,
    })
}

/// Verify a backup file without restoring.
pub(crate) fn verify_backup(backup_path: &str, decrypt_key: &[u8]) -> Result<BackupVerifyReport> {
    let file_data = std::fs::read(backup_path)
        .with_context(|| format!("BackupException: cannot read {}", backup_path))?;
    let checksum_ok = verify_checksum(&file_data).unwrap_or(false);
    let decrypt_ok = if checksum_ok {
        parse_and_decrypt(&file_data, decrypt_key).is_ok()
    } else {
        false
    };
    let decompress_ok = if decrypt_ok {
        let (flags, data) = parse_and_decrypt(&file_data, decrypt_key).unwrap();
        if flags & FLAG_COMPRESSED != 0 {
            zstd::decode_all(data.as_slice()).is_ok()
        } else {
            true
        }
    } else {
        false
    };
    Ok(BackupVerifyReport {
        checksum_ok,
        decrypt_ok,
        decompress_ok,
    })
}

/// Restore a backup to a target database path.
pub(crate) fn restore_backup(
    backup_path: &str,
    target_path: &str,
    decrypt_key: &[u8],
    expected_checksum: Option<&str>,
) -> Result<RestoreResult> {
    let file_data = std::fs::read(backup_path)
        .with_context(|| format!("RestoreException: cannot read {}", backup_path))?;
    if !verify_checksum(&file_data)? {
        return Err(anyhow!("RestoreException: BLAKE3 checksum mismatch"));
    }
    if let Some(expected) = expected_checksum {
        let actual = blake3::hash(&file_data[..file_data.len() - 32]);
        if actual.to_hex().to_string() != expected {
            return Err(anyhow!("RestoreException: expected checksum mismatch"));
        }
    }

    let (flags, payload) = parse_and_decrypt(&file_data, decrypt_key)?;
    let raw_db = if flags & FLAG_COMPRESSED != 0 {
        zstd::decode_all(payload.as_slice()).context("RestoreException: decompression failed")?
    } else {
        payload
    };

    if let Some(parent) = Path::new(target_path).parent() {
        if !parent.as_os_str().is_empty() {
            std::fs::create_dir_all(parent)?;
        }
    }
    std::fs::write(target_path, &raw_db).context("RestoreException: failed to write")?;
    let _ = std::fs::remove_file(format!("{}-wal", target_path));
    let _ = std::fs::remove_file(format!("{}-shm", target_path));

    Ok(RestoreResult {
        pages_restored: (raw_db.len() / 4096) as u64,
        integrity_ok: true,
    })
}

fn verify_checksum(file_data: &[u8]) -> Result<bool> {
    if file_data.len() < HEADER_LEN + 32 {
        return Err(anyhow!("BackupException: file too small"));
    }
    if &file_data[..4] != MAGIC {
        return Err(anyhow!("BackupException: invalid magic bytes"));
    }
    let stored = &file_data[file_data.len() - 32..];
    let computed = blake3::hash(&file_data[..file_data.len() - 32]);
    Ok(computed.as_bytes() == stored)
}

/// Returns (flags, decrypted_payload).
fn parse_and_decrypt(file_data: &[u8], decrypt_key: &[u8]) -> Result<(u8, Vec<u8>)> {
    if file_data.len() < HEADER_LEN + 32 {
        return Err(anyhow!("BackupException: file too small"));
    }
    let version = u32::from_le_bytes(file_data[4..8].try_into().unwrap());
    if version != VERSION {
        return Err(anyhow!("BackupException: unsupported version {}", version));
    }
    let flags = file_data[8];
    let nonce_bytes: [u8; NONCE_LEN] = file_data[9..21].try_into().unwrap();
    let _uncompressed_size = u64::from_le_bytes(file_data[21..29].try_into().unwrap());
    let payload = &file_data[HEADER_LEN..file_data.len() - 32];

    let decrypted = if flags & FLAG_ENCRYPTED != 0 {
        if decrypt_key.len() != 32 {
            return Err(anyhow!("BackupException: key must be 32 bytes"));
        }
        let cipher = Aes256Gcm::new_from_slice(decrypt_key)
            .map_err(|e| anyhow!("BackupException: invalid key: {}", e))?;
        cipher
            .decrypt(Nonce::from_slice(&nonce_bytes), payload)
            .map_err(|_| anyhow!("RestoreException: decryption failed (wrong key)"))?
    } else {
        payload.to_vec()
    };

    Ok((flags, decrypted))
}
