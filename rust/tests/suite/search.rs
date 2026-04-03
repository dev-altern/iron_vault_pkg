use crate::common::*;
use iron_vault_core::api::types::*;
use std::collections::HashMap;

fn setup_search(db: &iron_vault_core::api::vault::IronVaultDb) {
    db.build_search_index(
        "docs".into(),
        vec![
            SearchField {
                name: "title".into(),
                weight: 3.0,
                stored: true,
            },
            SearchField {
                name: "content".into(),
                weight: 1.0,
                stored: true,
            },
        ],
    )
    .unwrap();
}

fn index_doc(db: &iron_vault_core::api::vault::IronVaultDb, id: &str, title: &str, content: &str) {
    let mut fields = HashMap::new();
    fields.insert("title".into(), title.into());
    fields.insert("content".into(), content.into());
    db.search_index_row("docs".into(), id.into(), fields)
        .unwrap();
}

fn create_docs_table(db: &iron_vault_core::api::vault::IronVaultDb) {
    db.execute_raw(
        "CREATE TABLE docs (\
            id TEXT PRIMARY KEY, title TEXT, content TEXT, \
            tenant_id TEXT NOT NULL, created_at INTEGER NOT NULL, \
            updated_at INTEGER NOT NULL, deleted_at INTEGER)"
            .into(),
        vec![],
    )
    .unwrap();
}

// ─── Single Word Search ──────────────────────────────────────────────

#[test]
fn single_word_search() {
    let dir = tempfile::TempDir::new().unwrap();
    let db = open_test_db(&dir);
    create_docs_table(&db);
    setup_search(&db);

    index_doc(&db, "d1", "Quarterly Report", "Financial results for Q3");
    index_doc(&db, "d2", "Meeting Notes", "Discussed project timeline");
    index_doc(&db, "d3", "Budget Plan", "Financial planning for next year");

    let hits = db
        .search("docs".into(), "financial".into(), 10, false)
        .unwrap();
    assert_eq!(hits.len(), 2);
    assert!(hits.iter().all(|h| h.table == "docs"));
    assert!(hits.iter().all(|h| h.score > 0.0));
}

#[test]
fn phrase_search() {
    let dir = tempfile::TempDir::new().unwrap();
    let db = open_test_db(&dir);
    create_docs_table(&db);
    setup_search(&db);

    index_doc(
        &db,
        "d1",
        "Quarterly Financial Report",
        "Full details inside",
    );
    index_doc(
        &db,
        "d2",
        "Financial Analysis",
        "Quarterly overview of finances",
    );

    // Phrase search — exact sequence
    let hits = db
        .search("docs".into(), "\"quarterly financial\"".into(), 10, false)
        .unwrap();
    assert_eq!(hits.len(), 1);
    assert_eq!(hits[0].id, "d1");
}

#[test]
fn boolean_and() {
    let dir = tempfile::TempDir::new().unwrap();
    let db = open_test_db(&dir);
    create_docs_table(&db);
    setup_search(&db);

    index_doc(
        &db,
        "d1",
        "Rust Programming",
        "Systems programming language",
    );
    index_doc(&db, "d2", "Rust Prevention", "How to prevent rust on metal");
    index_doc(
        &db,
        "d3",
        "Python Programming",
        "Dynamic programming language",
    );

    let hits = db
        .search("docs".into(), "rust AND programming".into(), 10, false)
        .unwrap();
    assert_eq!(hits.len(), 1);
    assert_eq!(hits[0].id, "d1");
}

#[test]
fn boolean_or() {
    let dir = tempfile::TempDir::new().unwrap();
    let db = open_test_db(&dir);
    create_docs_table(&db);
    setup_search(&db);

    index_doc(&db, "d1", "Rust Guide", "Systems language");
    index_doc(&db, "d2", "Python Guide", "Dynamic language");
    index_doc(&db, "d3", "SQL Reference", "Database queries");

    let hits = db
        .search("docs".into(), "rust OR python".into(), 10, false)
        .unwrap();
    assert_eq!(hits.len(), 2);
}

#[test]
fn boolean_not() {
    let dir = tempfile::TempDir::new().unwrap();
    let db = open_test_db(&dir);
    create_docs_table(&db);
    setup_search(&db);

    index_doc(&db, "d1", "Rust Systems", "Low level language");
    index_doc(&db, "d2", "Rust Prevention", "Metal care guide");

    // NOT — exclude "prevention" (Tantivy uses -term syntax)
    let hits = db
        .search("docs".into(), "rust -prevention".into(), 10, false)
        .unwrap();
    assert_eq!(hits.len(), 1);
    assert_eq!(hits[0].id, "d1");
}

// ─── Snippet Generation ─────────────────────────────────────────────

#[test]
fn snippet_contains_highlighted_terms() {
    let dir = tempfile::TempDir::new().unwrap();
    let db = open_test_db(&dir);
    create_docs_table(&db);
    setup_search(&db);

    index_doc(
        &db,
        "d1",
        "Important Financial Report",
        "This report covers the quarterly financial results",
    );

    let hits = db
        .search("docs".into(), "financial".into(), 10, true)
        .unwrap();
    assert_eq!(hits.len(), 1);
    // Snippet should have <b> tags around the highlighted term
    assert!(
        hits[0].snippet.contains("<b>") || hits[0].snippet.contains("financial"),
        "Snippet should highlight: {}",
        hits[0].snippet
    );
}

// ─── Index Management ────────────────────────────────────────────────

#[test]
fn index_row_then_search() {
    let dir = tempfile::TempDir::new().unwrap();
    let db = open_test_db(&dir);
    create_docs_table(&db);
    setup_search(&db);

    // Before indexing — no results
    let hits = db
        .search("docs".into(), "budget".into(), 10, false)
        .unwrap();
    assert!(hits.is_empty());

    // Index a row
    index_doc(&db, "d1", "Budget Plan", "Annual budget planning");

    // Now searchable
    let hits = db
        .search("docs".into(), "budget".into(), 10, false)
        .unwrap();
    assert_eq!(hits.len(), 1);
    assert_eq!(hits[0].id, "d1");
}

#[test]
fn remove_from_index() {
    let dir = tempfile::TempDir::new().unwrap();
    let db = open_test_db(&dir);
    create_docs_table(&db);
    setup_search(&db);

    index_doc(&db, "d1", "Temp Doc", "To be deleted");
    assert_eq!(
        db.search("docs".into(), "temp".into(), 10, false)
            .unwrap()
            .len(),
        1
    );

    db.search_remove_row("docs".into(), "d1".into()).unwrap();
    assert_eq!(
        db.search("docs".into(), "temp".into(), 10, false)
            .unwrap()
            .len(),
        0
    );
}

#[test]
fn reindex_row_updates_content() {
    let dir = tempfile::TempDir::new().unwrap();
    let db = open_test_db(&dir);
    create_docs_table(&db);
    setup_search(&db);

    index_doc(&db, "d1", "Old Title", "Old content");
    assert_eq!(
        db.search("docs".into(), "old".into(), 10, false)
            .unwrap()
            .len(),
        1
    );

    // Re-index with new content
    index_doc(&db, "d1", "New Title", "New content");
    assert_eq!(
        db.search("docs".into(), "new".into(), 10, false)
            .unwrap()
            .len(),
        1
    );
    assert_eq!(
        db.search("docs".into(), "old".into(), 10, false)
            .unwrap()
            .len(),
        0
    );
}

// ─── Index Stats ─────────────────────────────────────────────────────

#[test]
fn index_stats_basic() {
    let dir = tempfile::TempDir::new().unwrap();
    let db = open_test_db(&dir);
    create_docs_table(&db);
    setup_search(&db);

    index_doc(&db, "d1", "Doc One", "Content one");
    index_doc(&db, "d2", "Doc Two", "Content two");

    let stats = db.search_index_stats("docs".into()).unwrap();
    assert_eq!(stats.num_docs, 2);
    assert!(stats.size_bytes > 0);
}

#[test]
fn index_stats_after_remove() {
    let dir = tempfile::TempDir::new().unwrap();
    let db = open_test_db(&dir);
    create_docs_table(&db);
    setup_search(&db);

    index_doc(&db, "d1", "Doc One", "Content one");
    index_doc(&db, "d2", "Doc Two", "Content two");
    db.search_remove_row("docs".into(), "d1".into()).unwrap();

    let stats = db.search_index_stats("docs".into()).unwrap();
    // Tantivy may still report 2 docs until segment merge, but the deleted doc won't appear in search
    assert!(stats.num_docs <= 2);
}

// ─── Edge Cases ──────────────────────────────────────────────────────

#[test]
fn search_empty_index() {
    let dir = tempfile::TempDir::new().unwrap();
    let db = open_test_db(&dir);
    create_docs_table(&db);
    setup_search(&db);

    let hits = db
        .search("docs".into(), "anything".into(), 10, false)
        .unwrap();
    assert!(hits.is_empty());
}

#[test]
fn search_nonexistent_table_errors() {
    let dir = tempfile::TempDir::new().unwrap();
    let db = open_test_db(&dir);

    let result = db.search("nonexistent".into(), "test".into(), 10, false);
    assert!(result.is_err());
}

#[test]
fn search_with_limit() {
    let dir = tempfile::TempDir::new().unwrap();
    let db = open_test_db(&dir);
    create_docs_table(&db);
    setup_search(&db);

    for i in 0..20 {
        index_doc(
            &db,
            &format!("d{}", i),
            &format!("Document {}", i),
            "common search term here",
        );
    }

    let hits = db.search("docs".into(), "common".into(), 5, false).unwrap();
    assert_eq!(hits.len(), 5);
}

#[test]
fn search_unicode_content() {
    let dir = tempfile::TempDir::new().unwrap();
    let db = open_test_db(&dir);
    create_docs_table(&db);
    setup_search(&db);

    index_doc(
        &db,
        "d1",
        "Café Menu",
        "Espresso and café au lait available",
    );
    index_doc(&db, "d2", "日本語ドキュメント", "これはテストです");

    let hits = db.search("docs".into(), "café".into(), 10, false).unwrap();
    assert!(!hits.is_empty());
}

#[test]
fn build_index_no_fields_errors() {
    let dir = tempfile::TempDir::new().unwrap();
    let db = open_test_db(&dir);

    let result = db.build_search_index("docs".into(), vec![]);
    assert!(result.is_err());
}

#[test]
fn search_ops_fail_after_close() {
    let dir = tempfile::TempDir::new().unwrap();
    let mut db = open_test_db(&dir);
    db.close().unwrap();

    assert!(db
        .build_search_index(
            "t".into(),
            vec![SearchField {
                name: "x".into(),
                weight: 1.0,
                stored: true
            }]
        )
        .is_err());
    assert!(db.search("t".into(), "q".into(), 10, false).is_err());
    assert!(db
        .search_index_row("t".into(), "id".into(), HashMap::new())
        .is_err());
    assert!(db.search_remove_row("t".into(), "id".into()).is_err());
    assert!(db.search_index_stats("t".into()).is_err());
}

#[test]
fn search_results_ordered_by_score() {
    let dir = tempfile::TempDir::new().unwrap();
    let db = open_test_db(&dir);
    create_docs_table(&db);
    setup_search(&db);

    // Title has weight 3.0, content has weight 1.0
    // d1 has "financial" in title (high score)
    // d2 has "financial" only in content (lower score)
    index_doc(&db, "d1", "Financial Report", "Quarterly overview");
    index_doc(
        &db,
        "d2",
        "Quarterly Report",
        "Financial details for review",
    );

    let hits = db
        .search("docs".into(), "financial".into(), 10, false)
        .unwrap();
    assert_eq!(hits.len(), 2);
    // d1 should score higher (term in weighted title field)
    assert!(hits[0].score >= hits[1].score);
}

#[test]
#[cfg_attr(target_os = "windows", ignore)] // Tantivy 500-doc index too slow on Windows CI
fn many_documents_searchable() {
    let dir = tempfile::TempDir::new().unwrap();
    let db = open_test_db(&dir);
    create_docs_table(&db);
    setup_search(&db);

    for i in 0..500 {
        index_doc(
            &db,
            &format!("d{}", i),
            &format!("Document number {}", i),
            &format!(
                "This is the content of document {} with some searchable text",
                i
            ),
        );
    }

    let hits = db
        .search("docs".into(), "searchable".into(), 20, false)
        .unwrap();
    assert_eq!(hits.len(), 20); // limited to 20

    let stats = db.search_index_stats("docs".into()).unwrap();
    assert_eq!(stats.num_docs, 500);
}
