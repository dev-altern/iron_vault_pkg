use crate::common::*;
use iron_vault_core::api::types::*;
use std::collections::HashMap;

fn create_docs_with_embedding(db: &iron_vault_core::api::vault::IronVaultDb) {
    db.execute_raw(
        "CREATE TABLE docs (\
            id TEXT PRIMARY KEY, title TEXT, content TEXT, embedding BLOB, \
            tenant_id TEXT NOT NULL, created_at INTEGER NOT NULL, \
            updated_at INTEGER NOT NULL, deleted_at INTEGER)"
            .into(),
        vec![],
    )
    .unwrap();
}

fn insert_doc(db: &iron_vault_core::api::vault::IronVaultDb, title: &str) -> String {
    let mut data = HashMap::new();
    data.insert("title".into(), SqlValue::Text(title.into()));
    data.insert(
        "content".into(),
        SqlValue::Text(format!("Content for {}", title)),
    );
    db.query_insert("docs".into(), data).unwrap()
}

/// Simple fake embedding: normalize the char values into a fixed-dim vector.
fn fake_embedding(text: &str, dim: usize) -> Vec<f32> {
    let mut vec = vec![0.0f32; dim];
    for (i, ch) in text.chars().enumerate() {
        vec[i % dim] += (ch as u32 as f32) / 1000.0;
    }
    // Normalize to unit length
    let norm: f32 = vec.iter().map(|x| x * x).sum::<f32>().sqrt();
    if norm > 0.0 {
        for v in vec.iter_mut() {
            *v /= norm;
        }
    }
    vec
}

// ─── Vector Serialization ────────────────────────────────────────────

#[test]
fn serialize_and_deserialize_vector() {
    let original = vec![1.0f32, -2.5, 0.0, 3.12345];
    let bytes = iron_vault_core::api::vault::IronVaultDb::serialize_vector(original.clone());
    let restored = iron_vault_core::api::vault::IronVaultDb::deserialize_vector(bytes).unwrap();
    assert_eq!(original, restored);
}

#[test]
fn deserialize_bad_bytes_errors() {
    let result = iron_vault_core::api::vault::IronVaultDb::deserialize_vector(vec![0, 1, 2]);
    assert!(result.is_err());
}

// ─── Store / Retrieve Embeddings ─────────────────────────────────────

#[test]
fn store_and_retrieve_embedding() {
    let dir = tempfile::TempDir::new().unwrap();
    let db = open_test_db(&dir);
    create_docs_with_embedding(&db);

    let id = insert_doc(&db, "Test Doc");
    let embedding = vec![0.1f32, 0.2, 0.3, 0.4];
    db.store_embedding("docs".into(), id.clone(), embedding.clone())
        .unwrap();

    let retrieved = db.get_embedding("docs".into(), id).unwrap();
    assert_eq!(embedding, retrieved);
}

#[test]
fn store_embedding_nonexistent_row_errors() {
    let dir = tempfile::TempDir::new().unwrap();
    let db = open_test_db(&dir);
    create_docs_with_embedding(&db);

    let result = db.store_embedding("docs".into(), "nonexistent".into(), vec![1.0]);
    assert!(result.is_err());
}

#[test]
fn store_embedding_overwrite() {
    let dir = tempfile::TempDir::new().unwrap();
    let db = open_test_db(&dir);
    create_docs_with_embedding(&db);

    let id = insert_doc(&db, "Doc");
    db.store_embedding("docs".into(), id.clone(), vec![1.0, 2.0])
        .unwrap();
    db.store_embedding("docs".into(), id.clone(), vec![3.0, 4.0])
        .unwrap();

    let retrieved = db.get_embedding("docs".into(), id).unwrap();
    assert_eq!(retrieved, vec![3.0, 4.0]);
}

// ─── Semantic Search ─────────────────────────────────────────────────

#[test]
fn semantic_search_finds_similar() {
    let dir = tempfile::TempDir::new().unwrap();
    let db = open_test_db(&dir);
    create_docs_with_embedding(&db);

    let id1 = insert_doc(&db, "rust programming");
    let id2 = insert_doc(&db, "rust language guide");
    let id3 = insert_doc(&db, "cooking recipes");

    // Use explicit vectors: id1 and id2 are similar (close in vector space),
    // id3 is very different (orthogonal direction)
    db.store_embedding("docs".into(), id1.clone(), vec![0.9, 0.1, 0.0, 0.0])
        .unwrap();
    db.store_embedding("docs".into(), id2.clone(), vec![0.8, 0.2, 0.0, 0.0])
        .unwrap();
    db.store_embedding("docs".into(), id3.clone(), vec![0.0, 0.0, 0.9, 0.1])
        .unwrap();

    let query_vec = vec![1.0, 0.0, 0.0, 0.0]; // aligned with rust docs
    let hits = db
        .search_semantic("docs".into(), query_vec, 10, 0.0)
        .unwrap();

    assert_eq!(hits.len(), 3);
    // Rust-related docs (id1, id2) should score higher than cooking (id3)
    let rust_scores: Vec<f64> = hits
        .iter()
        .filter(|h| h.id == id1 || h.id == id2)
        .map(|h| h.score)
        .collect();
    let cooking_score = hits
        .iter()
        .find(|h| h.id == id3)
        .map(|h| h.score)
        .unwrap_or(0.0);
    assert!(
        rust_scores.iter().all(|&s| s > cooking_score),
        "Rust docs ({:?}) should score higher than cooking ({:.4})",
        rust_scores,
        cooking_score
    );
}

#[test]
fn semantic_search_respects_threshold() {
    let dir = tempfile::TempDir::new().unwrap();
    let db = open_test_db(&dir);
    create_docs_with_embedding(&db);

    let id1 = insert_doc(&db, "match");
    db.store_embedding("docs".into(), id1, vec![1.0, 0.0])
        .unwrap();

    // Query orthogonal vector — cosine = 0.0
    let hits = db
        .search_semantic("docs".into(), vec![0.0, 1.0], 10, 0.5)
        .unwrap();
    assert!(
        hits.is_empty(),
        "Orthogonal vector should be below threshold 0.5"
    );

    // Same direction — cosine = 1.0
    let hits = db
        .search_semantic("docs".into(), vec![1.0, 0.0], 10, 0.5)
        .unwrap();
    assert_eq!(hits.len(), 1);
}

#[test]
fn semantic_search_respects_top_k() {
    let dir = tempfile::TempDir::new().unwrap();
    let db = open_test_db(&dir);
    create_docs_with_embedding(&db);

    for i in 0..20 {
        let id = insert_doc(&db, &format!("doc {}", i));
        db.store_embedding(
            "docs".into(),
            id,
            fake_embedding(&format!("content {}", i), 8),
        )
        .unwrap();
    }

    let hits = db
        .search_semantic("docs".into(), fake_embedding("content", 8), 5, 0.0)
        .unwrap();
    assert_eq!(hits.len(), 5);
}

#[test]
fn semantic_search_excludes_soft_deleted() {
    let dir = tempfile::TempDir::new().unwrap();
    let db = open_test_db(&dir);
    create_docs_with_embedding(&db);

    let id1 = insert_doc(&db, "visible");
    let id2 = insert_doc(&db, "deleted");
    db.store_embedding("docs".into(), id1, vec![1.0, 0.0])
        .unwrap();
    db.store_embedding("docs".into(), id2.clone(), vec![1.0, 0.0])
        .unwrap();

    db.query_delete("docs".into(), id2).unwrap();

    let hits = db
        .search_semantic("docs".into(), vec![1.0, 0.0], 10, 0.0)
        .unwrap();
    assert_eq!(hits.len(), 1);
}

#[test]
fn semantic_search_empty_table() {
    let dir = tempfile::TempDir::new().unwrap();
    let db = open_test_db(&dir);
    create_docs_with_embedding(&db);

    let hits = db
        .search_semantic("docs".into(), vec![1.0, 0.0], 10, 0.0)
        .unwrap();
    assert!(hits.is_empty());
}

#[test]
fn semantic_search_rows_without_embedding_skipped() {
    let dir = tempfile::TempDir::new().unwrap();
    let db = open_test_db(&dir);
    create_docs_with_embedding(&db);

    let id1 = insert_doc(&db, "with embedding");
    let _id2 = insert_doc(&db, "no embedding");
    db.store_embedding("docs".into(), id1, vec![1.0, 0.0])
        .unwrap();

    let hits = db
        .search_semantic("docs".into(), vec![1.0, 0.0], 10, 0.0)
        .unwrap();
    assert_eq!(hits.len(), 1);
}

#[test]
fn semantic_search_results_sorted_by_score() {
    let dir = tempfile::TempDir::new().unwrap();
    let db = open_test_db(&dir);
    create_docs_with_embedding(&db);

    let id1 = insert_doc(&db, "exact");
    let id2 = insert_doc(&db, "close");
    let id3 = insert_doc(&db, "far");

    db.store_embedding("docs".into(), id1, vec![1.0, 0.0])
        .unwrap();
    db.store_embedding("docs".into(), id2, vec![0.9, 0.1])
        .unwrap();
    db.store_embedding("docs".into(), id3, vec![0.0, 1.0])
        .unwrap();

    let hits = db
        .search_semantic("docs".into(), vec![1.0, 0.0], 10, 0.0)
        .unwrap();
    assert!(hits.len() >= 2);
    for i in 1..hits.len() {
        assert!(
            hits[i - 1].score >= hits[i].score,
            "Results should be sorted by score desc"
        );
    }
}

// ─── Tenant Isolation ────────────────────────────────────────────────

#[test]
fn semantic_search_tenant_isolated() {
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

    create_docs_with_embedding(&db_a);

    let id_a = {
        let mut d = HashMap::new();
        d.insert("title".into(), SqlValue::Text("A's doc".into()));
        db_a.query_insert("docs".into(), d).unwrap()
    };
    let id_b = {
        let mut d = HashMap::new();
        d.insert("title".into(), SqlValue::Text("B's doc".into()));
        db_b.query_insert("docs".into(), d).unwrap()
    };

    db_a.store_embedding("docs".into(), id_a, vec![1.0, 0.0])
        .unwrap();
    db_b.store_embedding("docs".into(), id_b, vec![1.0, 0.0])
        .unwrap();

    let hits_a = db_a
        .search_semantic("docs".into(), vec![1.0, 0.0], 10, 0.0)
        .unwrap();
    let hits_b = db_b
        .search_semantic("docs".into(), vec![1.0, 0.0], 10, 0.0)
        .unwrap();

    assert_eq!(hits_a.len(), 1);
    assert_eq!(hits_b.len(), 1);
}

// ─── Edge Cases ──────────────────────────────────────────────────────

#[test]
fn semantic_ops_fail_after_close() {
    let dir = tempfile::TempDir::new().unwrap();
    let mut db = open_test_db(&dir);
    db.close().unwrap();

    assert!(db
        .store_embedding("t".into(), "id".into(), vec![1.0])
        .is_err());
    assert!(db.get_embedding("t".into(), "id".into()).is_err());
    assert!(db.search_semantic("t".into(), vec![1.0], 10, 0.0).is_err());
    assert!(db
        .search_hybrid("t".into(), "q".into(), vec![1.0], 0.5, 0.5, 10)
        .is_err());
}

#[test]
fn large_embedding_storage() {
    let dir = tempfile::TempDir::new().unwrap();
    let db = open_test_db(&dir);
    create_docs_with_embedding(&db);

    let id = insert_doc(&db, "large");
    let large_vec: Vec<f32> = (0..1536).map(|i| (i as f32) * 0.001).collect(); // GPT-3 size
    db.store_embedding("docs".into(), id.clone(), large_vec.clone())
        .unwrap();

    let retrieved = db.get_embedding("docs".into(), id).unwrap();
    assert_eq!(retrieved.len(), 1536);
    assert_eq!(retrieved, large_vec);
}
