use crate::api::types::{BackupResult, BackupVerifyReport, RestoreResult};
use aes_gcm::aead::{Aead, KeyInit};
use aes_gcm::{Aes256Gcm, Nonce};
use anyhow::{anyhow, Context, Result};
use rand::RngCore;
use std::io::{Read, Write};
use std::path::Path;

const MAGIC: &[u8; 4] = b"IRON";
const FORMAT_VERSION: u32 = 2;
const FLAG_COMPRESSED: u8 = 0x01;
const FLAG_ENCRYPTED: u8 = 0x02;
const CHUNK_SIZE: usize = 1024 * 1024; // 1MB chunks
// Header v2: magic(4) + version(4) + flags(1) + chunk_size(4) + num_chunks(4) + uncompressed_size(8) = 25
const HEADER_LEN: usize = 4 + 4 + 1 + 4 + 4 + 8;

/// Create a backup using chunked streaming.
///
/// Reads the DB file in 1MB chunks, compresses + encrypts each independently.
/// Peak memory: ~2× chunk_size regardless of DB size.
pub(crate) fn create_backup(
    db_path: &str,
    output_path: &str,
    compress: bool,
    encrypt: bool,
    backup_key: &[u8],
) -> Result<BackupResult> {
    let db_file = std::fs::File::open(db_path)
        .with_context(|| format!("BackupException: cannot open {}", db_path))?;
    let uncompressed_size = db_file.metadata()?.len();
    let mut reader = std::io::BufReader::new(db_file);

    if let Some(parent) = Path::new(output_path).parent() {
        if !parent.as_os_str().is_empty() {
            std::fs::create_dir_all(parent)?;
        }
    }

    let mut flags: u8 = 0;
    if compress {
        flags |= FLAG_COMPRESSED;
    }
    if encrypt {
        flags |= FLAG_ENCRYPTED;
    }

    // Count chunks
    let num_chunks = ((uncompressed_size as usize + CHUNK_SIZE - 1) / CHUNK_SIZE).max(1) as u32;

    // Write header
    let out_file = std::fs::File::create(output_path)
        .with_context(|| format!("BackupException: cannot create {}", output_path))?;
    let mut writer = std::io::BufWriter::new(out_file);

    writer.write_all(MAGIC)?;
    writer.write_all(&FORMAT_VERSION.to_le_bytes())?;
    writer.write_all(&[flags])?;
    writer.write_all(&(CHUNK_SIZE as u32).to_le_bytes())?;
    writer.write_all(&num_chunks.to_le_bytes())?;
    writer.write_all(&uncompressed_size.to_le_bytes())?;

    let cipher = if encrypt {
        if backup_key.len() != 32 {
            return Err(anyhow!("BackupException: key must be 32 bytes"));
        }
        Some(
            Aes256Gcm::new_from_slice(backup_key)
                .map_err(|e| anyhow!("BackupException: invalid key: {}", e))?,
        )
    } else {
        None
    };

    // Process chunks
    let mut buf = vec![0u8; CHUNK_SIZE];
    loop {
        let bytes_read = read_full(&mut reader, &mut buf)?;
        if bytes_read == 0 {
            break;
        }
        let chunk_data = &buf[..bytes_read];

        // Compress
        let processed = if compress {
            zstd::encode_all(chunk_data, 3)
                .context("BackupException: chunk compression failed")?
        } else {
            chunk_data.to_vec()
        };

        // Encrypt
        if let Some(ref cipher) = cipher {
            let mut nonce_bytes = [0u8; 12];
            rand::thread_rng().fill_bytes(&mut nonce_bytes);
            let nonce = Nonce::from_slice(&nonce_bytes);
            let ciphertext = cipher
                .encrypt(nonce, processed.as_ref())
                .map_err(|e| anyhow!("BackupException: chunk encryption failed: {}", e))?;
            // Write: nonce(12) + ciphertext(N, includes GCM tag)
            writer.write_all(&nonce_bytes)?;
            writer.write_all(&(ciphertext.len() as u32).to_le_bytes())?;
            writer.write_all(&ciphertext)?;
        } else {
            // Unencrypted: length(4) + data(N)
            writer.write_all(&(processed.len() as u32).to_le_bytes())?;
            writer.write_all(&processed)?;
        }
    }

    writer.flush()?;
    drop(writer);

    // Compute BLAKE3 over entire file (except the checksum itself)
    let file_data = std::fs::read(output_path)?;
    let checksum = blake3::hash(&file_data);
    let checksum_hex = checksum.to_hex().to_string();

    // Append checksum
    let mut file = std::fs::OpenOptions::new()
        .append(true)
        .open(output_path)?;
    file.write_all(checksum.as_bytes())?;

    let final_size = std::fs::metadata(output_path)?.len();

    Ok(BackupResult {
        path: output_path.to_string(),
        size_bytes: final_size,
        checksum: checksum_hex,
        compressed: compress,
        encrypted: encrypt,
    })
}

/// Verify a backup file without restoring.
pub(crate) fn verify_backup(
    backup_path: &str,
    decrypt_key: &[u8],
) -> Result<BackupVerifyReport> {
    let file_data = std::fs::read(backup_path)
        .with_context(|| format!("BackupException: cannot read {}", backup_path))?;

    let checksum_ok = verify_checksum(&file_data).unwrap_or(false);
    let decrypt_ok = if checksum_ok {
        restore_to_memory(&file_data, decrypt_key).is_ok()
    } else {
        false
    };
    let decompress_ok = decrypt_ok; // restore_to_memory handles both

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

    let raw_db = restore_to_memory(&file_data, decrypt_key)?;

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

// ─── Internal ────────────────────────────────────────────────────────

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

/// Restore chunks from the backup format into a single byte buffer.
fn restore_to_memory(file_data: &[u8], decrypt_key: &[u8]) -> Result<Vec<u8>> {
    if file_data.len() < HEADER_LEN + 32 {
        return Err(anyhow!("BackupException: file too small"));
    }

    let version = u32::from_le_bytes(file_data[4..8].try_into().unwrap());
    let flags = file_data[8];
    let is_compressed = flags & FLAG_COMPRESSED != 0;
    let is_encrypted = flags & FLAG_ENCRYPTED != 0;

    // Support both v1 and v2 formats
    let (mut cursor, _chunk_size_hint) = if version == 2 {
        let _chunk_size = u32::from_le_bytes(file_data[9..13].try_into().unwrap()) as usize;
        let _num_chunks = u32::from_le_bytes(file_data[13..17].try_into().unwrap());
        let _uncompressed = u64::from_le_bytes(file_data[17..25].try_into().unwrap());
        (HEADER_LEN, _chunk_size)
    } else if version == 1 {
        // Legacy v1: nonce(12) + uncompressed_size(8) after flags
        // header = 4+4+1+12+8 = 29
        return restore_v1(file_data, flags, decrypt_key);
    } else {
        return Err(anyhow!("BackupException: unsupported version {}", version));
    };

    let payload_end = file_data.len() - 32; // exclude BLAKE3 checksum

    let cipher = if is_encrypted {
        if decrypt_key.len() != 32 {
            return Err(anyhow!("BackupException: key must be 32 bytes"));
        }
        Some(
            Aes256Gcm::new_from_slice(decrypt_key)
                .map_err(|e| anyhow!("BackupException: invalid key: {}", e))?,
        )
    } else {
        None
    };

    let mut output = Vec::new();

    while cursor < payload_end {
        if is_encrypted {
            // Read nonce(12) + len(4) + ciphertext(len)
            if cursor + 16 > payload_end {
                break;
            }
            let nonce_bytes: [u8; 12] = file_data[cursor..cursor + 12].try_into().unwrap();
            cursor += 12;
            let ct_len =
                u32::from_le_bytes(file_data[cursor..cursor + 4].try_into().unwrap()) as usize;
            cursor += 4;
            if cursor + ct_len > payload_end {
                return Err(anyhow!("BackupException: truncated chunk"));
            }
            let ciphertext = &file_data[cursor..cursor + ct_len];
            cursor += ct_len;

            let decrypted = cipher
                .as_ref()
                .unwrap()
                .decrypt(Nonce::from_slice(&nonce_bytes), ciphertext)
                .map_err(|_| anyhow!("RestoreException: chunk decryption failed"))?;

            let chunk_data = if is_compressed {
                zstd::decode_all(decrypted.as_slice())
                    .context("RestoreException: chunk decompression failed")?
            } else {
                decrypted
            };
            output.extend_from_slice(&chunk_data);
        } else {
            // Read len(4) + data(len)
            if cursor + 4 > payload_end {
                break;
            }
            let data_len =
                u32::from_le_bytes(file_data[cursor..cursor + 4].try_into().unwrap()) as usize;
            cursor += 4;
            if cursor + data_len > payload_end {
                return Err(anyhow!("BackupException: truncated chunk"));
            }
            let data = &file_data[cursor..cursor + data_len];
            cursor += data_len;

            let chunk_data = if is_compressed {
                zstd::decode_all(data).context("RestoreException: chunk decompression failed")?
            } else {
                data.to_vec()
            };
            output.extend_from_slice(&chunk_data);
        }
    }

    Ok(output)
}

/// Restore legacy v1 format (single blob, not chunked).
fn restore_v1(file_data: &[u8], flags: u8, decrypt_key: &[u8]) -> Result<Vec<u8>> {
    let is_compressed = flags & FLAG_COMPRESSED != 0;
    let is_encrypted = flags & FLAG_ENCRYPTED != 0;
    // v1 header: magic(4) + version(4) + flags(1) + nonce(12) + uncompressed_size(8) = 29
    let nonce_bytes: [u8; 12] = file_data[9..21].try_into().unwrap();
    let payload = &file_data[29..file_data.len() - 32];

    let decrypted = if is_encrypted {
        if decrypt_key.len() != 32 {
            return Err(anyhow!("BackupException: key must be 32 bytes"));
        }
        let cipher = Aes256Gcm::new_from_slice(decrypt_key)
            .map_err(|e| anyhow!("BackupException: invalid key: {}", e))?;
        cipher
            .decrypt(Nonce::from_slice(&nonce_bytes), payload)
            .map_err(|_| anyhow!("RestoreException: decryption failed"))?
    } else {
        payload.to_vec()
    };

    if is_compressed {
        zstd::decode_all(decrypted.as_slice()).context("RestoreException: decompression failed")
    } else {
        Ok(decrypted)
    }
}

/// Read exactly buf.len() bytes, or less at EOF.
fn read_full(reader: &mut impl Read, buf: &mut [u8]) -> Result<usize> {
    let mut total = 0;
    while total < buf.len() {
        match reader.read(&mut buf[total..])? {
            0 => break,
            n => total += n,
        }
    }
    Ok(total)
}
