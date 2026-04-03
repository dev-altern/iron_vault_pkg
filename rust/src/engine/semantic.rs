use crate::api::types::{HybridHit, SemanticHit};
use anyhow::{anyhow, Result};
use r2d2::PooledConnection;
use r2d2_sqlite::SqliteConnectionManager;

// ─── Vector Serialization ────────────────────────────────────────────

/// Serialize a Vec<f32> to little-endian bytes for BLOB storage.
pub(crate) fn serialize_vector(vec: &[f32]) -> Vec<u8> {
    let mut bytes = Vec::with_capacity(vec.len() * 4);
    for &v in vec {
        bytes.extend_from_slice(&v.to_le_bytes());
    }
    bytes
}

/// Deserialize little-endian bytes back to Vec<f32>.
pub(crate) fn deserialize_vector(bytes: &[u8]) -> Result<Vec<f32>> {
    if !bytes.len().is_multiple_of(4) {
        return Err(anyhow!(
            "SemanticException: blob size {} is not a multiple of 4",
            bytes.len()
        ));
    }
    let mut vec = Vec::with_capacity(bytes.len() / 4);
    for chunk in bytes.chunks_exact(4) {
        vec.push(f32::from_le_bytes(chunk.try_into().unwrap()));
    }
    Ok(vec)
}

// ─── Cosine Similarity ──────────────────────────────────────────────

/// Compute cosine similarity between two vectors.
///
/// Returns a value in [-1, 1]. For normalized vectors, 1.0 = identical,
/// 0.0 = orthogonal, -1.0 = opposite.
pub(crate) fn cosine_similarity(a: &[f32], b: &[f32]) -> Result<f64> {
    if a.len() != b.len() {
        return Err(anyhow!(
            "SemanticException: vector dimension mismatch ({} vs {})",
            a.len(),
            b.len()
        ));
    }
    if a.is_empty() {
        return Err(anyhow!("SemanticException: vectors must not be empty"));
    }

    let mut dot = 0.0f64;
    let mut norm_a = 0.0f64;
    let mut norm_b = 0.0f64;

    for i in 0..a.len() {
        let ai = a[i] as f64;
        let bi = b[i] as f64;
        dot += ai * bi;
        norm_a += ai * ai;
        norm_b += bi * bi;
    }

    let denom = norm_a.sqrt() * norm_b.sqrt();
    if denom == 0.0 {
        return Ok(0.0); // zero vector
    }

    Ok(dot / denom)
}

// ─── Store / Retrieve Embeddings ─────────────────────────────────────

/// Store an embedding vector for a row.
///
/// The table must have an `embedding BLOB` column.
pub(crate) fn store_embedding(
    conn: &PooledConnection<SqliteConnectionManager>,
    table: &str,
    id: &str,
    embedding: &[f32],
    tenant_id: &str,
) -> Result<()> {
    crate::engine::validate::table_name(table)?;
    let blob = serialize_vector(embedding);
    let affected = conn.execute(
        &format!(
            "UPDATE {} SET embedding = ?1 WHERE id = ?2 AND tenant_id = ?3",
            table
        ),
        rusqlite::params![blob, id, tenant_id],
    )?;
    if affected == 0 {
        return Err(anyhow!(
            "SemanticException: row {} not found in {}",
            id,
            table
        ));
    }
    Ok(())
}

/// Retrieve an embedding vector for a row.
pub(crate) fn get_embedding(
    conn: &PooledConnection<SqliteConnectionManager>,
    table: &str,
    id: &str,
    tenant_id: &str,
) -> Result<Vec<f32>> {
    crate::engine::validate::table_name(table)?;
    let blob: Vec<u8> = conn.query_row(
        &format!(
            "SELECT embedding FROM {} WHERE id = ?1 AND tenant_id = ?2",
            table
        ),
        rusqlite::params![id, tenant_id],
        |row| row.get(0),
    )?;
    deserialize_vector(&blob)
}

// ─── Semantic Search ─────────────────────────────────────────────────

/// Search for rows with similar embeddings using brute-force cosine similarity.
///
/// Returns top-K results above the threshold, sorted by score descending.
pub(crate) fn search_semantic(
    conn: &PooledConnection<SqliteConnectionManager>,
    table: &str,
    query_embedding: &[f32],
    tenant_id: &str,
    top_k: u32,
    threshold: f64,
) -> Result<Vec<SemanticHit>> {
    crate::engine::validate::table_name(table)?;

    let mut stmt = conn.prepare(&format!(
        "SELECT id, embedding FROM {} WHERE tenant_id = ?1 AND deleted_at IS NULL AND embedding IS NOT NULL",
        table
    ))?;

    let rows = stmt.query_map(rusqlite::params![tenant_id], |row| {
        let id: String = row.get(0)?;
        let blob: Vec<u8> = row.get(1)?;
        Ok((id, blob))
    })?;

    let mut hits: Vec<SemanticHit> = Vec::new();
    for row in rows {
        let (id, blob) = row?;
        if let Ok(embedding) = deserialize_vector(&blob) {
            if let Ok(score) = cosine_similarity(query_embedding, &embedding) {
                if score >= threshold {
                    hits.push(SemanticHit { id, score });
                }
            }
        }
    }

    // Sort by score descending
    hits.sort_by(|a, b| {
        b.score
            .partial_cmp(&a.score)
            .unwrap_or(std::cmp::Ordering::Equal)
    });
    hits.truncate(top_k as usize);

    Ok(hits)
}

/// Hybrid search combining FTS and semantic scores.
///
/// 1. FTS search returns top candidates with BM25 scores
/// 2. Compute cosine similarity for those candidates
/// 3. Combine: score = fts_weight * normalized_bm25 + semantic_weight * cosine
/// 4. Re-rank and return top limit
pub(crate) fn search_hybrid(
    conn: &PooledConnection<SqliteConnectionManager>,
    table: &str,
    query_embedding: &[f32],
    fts_hits: &[crate::api::types::SearchHit],
    fts_weight: f64,
    semantic_weight: f64,
    tenant_id: &str,
    limit: u32,
) -> Result<Vec<HybridHit>> {
    if fts_hits.is_empty() {
        return Ok(Vec::new());
    }

    // Normalize FTS scores to [0, 1]
    let max_fts = fts_hits.iter().map(|h| h.score).fold(0.0f64, f64::max);
    let max_fts = if max_fts == 0.0 { 1.0 } else { max_fts };

    let mut hybrid_hits = Vec::new();
    for fts_hit in fts_hits {
        let fts_normalized = fts_hit.score / max_fts;

        // Get embedding for this row
        let semantic_score = match get_embedding(conn, table, &fts_hit.id, tenant_id) {
            Ok(embedding) => cosine_similarity(query_embedding, &embedding).unwrap_or(0.0),
            Err(_) => 0.0, // no embedding stored
        };

        let combined = fts_weight * fts_normalized + semantic_weight * semantic_score;

        hybrid_hits.push(HybridHit {
            id: fts_hit.id.clone(),
            table: table.to_string(),
            score: combined,
            fts_score: fts_normalized,
            semantic_score,
            snippet: fts_hit.snippet.clone(),
        });
    }

    hybrid_hits.sort_by(|a, b| {
        b.score
            .partial_cmp(&a.score)
            .unwrap_or(std::cmp::Ordering::Equal)
    });
    hybrid_hits.truncate(limit as usize);

    Ok(hybrid_hits)
}

// ─── Unit Tests ──────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn serialize_deserialize_round_trip() {
        let vec = vec![1.0f32, -2.5, 0.0, 3.12345, f32::MAX, f32::MIN];
        let bytes = serialize_vector(&vec);
        assert_eq!(bytes.len(), vec.len() * 4);
        let restored = deserialize_vector(&bytes).unwrap();
        assert_eq!(vec, restored);
    }

    #[test]
    fn serialize_empty_vector() {
        let vec: Vec<f32> = vec![];
        let bytes = serialize_vector(&vec);
        assert!(bytes.is_empty());
        let restored = deserialize_vector(&bytes).unwrap();
        assert!(restored.is_empty());
    }

    #[test]
    fn deserialize_bad_length_errors() {
        assert!(deserialize_vector(&[0u8; 3]).is_err()); // not multiple of 4
        assert!(deserialize_vector(&[0u8; 5]).is_err());
    }

    #[test]
    fn cosine_identical_vectors() {
        let v = vec![1.0f32, 2.0, 3.0];
        let sim = cosine_similarity(&v, &v).unwrap();
        assert!((sim - 1.0).abs() < 1e-6);
    }

    #[test]
    fn cosine_orthogonal_vectors() {
        let a = vec![1.0f32, 0.0];
        let b = vec![0.0f32, 1.0];
        let sim = cosine_similarity(&a, &b).unwrap();
        assert!(sim.abs() < 1e-6);
    }

    #[test]
    fn cosine_opposite_vectors() {
        let a = vec![1.0f32, 0.0];
        let b = vec![-1.0f32, 0.0];
        let sim = cosine_similarity(&a, &b).unwrap();
        assert!((sim - (-1.0)).abs() < 1e-6);
    }

    #[test]
    fn cosine_similar_vectors_high_score() {
        let a = vec![1.0f32, 2.0, 3.0];
        let b = vec![1.1f32, 2.1, 3.1]; // slightly different
        let sim = cosine_similarity(&a, &b).unwrap();
        assert!(sim > 0.99);
    }

    #[test]
    fn cosine_different_dimensions_error() {
        assert!(cosine_similarity(&[1.0, 2.0], &[1.0]).is_err());
    }

    #[test]
    fn cosine_empty_vectors_error() {
        assert!(cosine_similarity(&[], &[]).is_err());
    }

    #[test]
    fn cosine_zero_vector() {
        let a = vec![0.0f32, 0.0, 0.0];
        let b = vec![1.0f32, 2.0, 3.0];
        let sim = cosine_similarity(&a, &b).unwrap();
        assert_eq!(sim, 0.0);
    }

    #[test]
    fn cosine_large_vectors() {
        let dim = 384; // typical embedding dimension
        let a: Vec<f32> = (0..dim).map(|i| (i as f32) * 0.01).collect();
        let b: Vec<f32> = (0..dim).map(|i| (i as f32) * 0.01 + 0.001).collect();
        let sim = cosine_similarity(&a, &b).unwrap();
        assert!(sim > 0.99);
    }

    #[test]
    fn cosine_normalized_vectors() {
        // Pre-normalized (unit length)
        let a = vec![0.6f32, 0.8];
        let b = vec![0.8f32, 0.6];
        let sim = cosine_similarity(&a, &b).unwrap();
        // dot = 0.48 + 0.48 = 0.96
        assert!((sim - 0.96).abs() < 1e-4);
    }
}
