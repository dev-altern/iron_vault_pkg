use crate::common::*;
use iron_vault_core::api::types::*;
use std::collections::HashMap;

fn create_docs_table(db: &iron_vault_core::api::vault::IronVaultDb) {
    db.execute_raw(
        "CREATE TABLE docs (\
            id TEXT PRIMARY KEY, title TEXT NOT NULL, content TEXT, \
            tenant_id TEXT NOT NULL, created_at INTEGER NOT NULL, \
            updated_at INTEGER NOT NULL, deleted_at INTEGER)"
            .into(),
        vec![],
    )
    .unwrap();
}

fn setup_auto_index(db: &iron_vault_core::api::vault::IronVaultDb) {
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

fn insert_doc(db: &iron_vault_core::api::vault::IronVaultDb, title: &str, content: &str) -> String {
    let mut data = HashMap::new();
    data.insert("title".into(), SqlValue::Text(title.into()));
    data.insert("content".into(), SqlValue::Text(content.into()));
    db.query_insert("docs".into(), data).unwrap()
}

// ─── Insert Auto-Indexes ─────────────────────────────────────────────

#[test]
fn insert_auto_indexes_row() {
    let dir = tempfile::TempDir::new().unwrap();
    let db = open_test_db(&dir);
    create_docs_table(&db);
    setup_auto_index(&db);

    insert_doc(&db, "Rust Programming", "Systems language guide");

    // Should be immediately searchable without manual index_row call
    let hits = db
        .search("docs".into(), "rust".into(), 10, false)
        .unwrap();
    assert_eq!(hits.len(), 1);
}

#[test]
fn insert_multiple_all_auto_indexed() {
    let dir = tempfile::TempDir::new().unwrap();
    let db = open_test_db(&dir);
    create_docs_table(&db);
    setup_auto_index(&db);

    insert_doc(&db, "Rust Guide", "Systems programming");
    insert_doc(&db, "Python Guide", "Dynamic language");
    insert_doc(&db, "SQL Reference", "Database queries");

    let hits = db
        .search("docs".into(), "guide".into(), 10, false)
        .unwrap();
    assert_eq!(hits.len(), 2);
}

// ─── Update Auto-Re-Indexes ─────────────────────────────────────────

#[test]
fn update_auto_reindexes_row() {
    let dir = tempfile::TempDir::new().unwrap();
    let db = open_test_db(&dir);
    create_docs_table(&db);
    setup_auto_index(&db);

    let id = insert_doc(&db, "Unique Alpha", "Some content");
    assert_eq!(
        db.search("docs".into(), "alpha".into(), 10, false)
            .unwrap()
            .len(),
        1
    );

    // Update both title and content
    let mut data = HashMap::new();
    data.insert("title".into(), SqlValue::Text("Unique Beta".into()));
    data.insert("content".into(), SqlValue::Text("Updated content".into()));
    db.query_update("docs".into(), id, data).unwrap();

    // "alpha" should be gone (was only in title), "beta" should be found
    let alpha_hits = db
        .search("docs".into(), "alpha".into(), 10, false)
        .unwrap();
    assert_eq!(alpha_hits.len(), 0, "Old title term should be removed");

    let beta_hits = db
        .search("docs".into(), "beta".into(), 10, false)
        .unwrap();
    assert_eq!(beta_hits.len(), 1, "New title term should be found");
}

// ─── Delete Auto-Removes from Index ──────────────────────────────────

#[test]
fn soft_delete_auto_removes_from_index() {
    let dir = tempfile::TempDir::new().unwrap();
    let db = open_test_db(&dir);
    create_docs_table(&db);
    setup_auto_index(&db);

    let id = insert_doc(&db, "Deletable Doc", "Will be removed");
    assert_eq!(
        db.search("docs".into(), "deletable".into(), 10, false)
            .unwrap()
            .len(),
        1
    );

    db.query_delete("docs".into(), id).unwrap();

    assert_eq!(
        db.search("docs".into(), "deletable".into(), 10, false)
            .unwrap()
            .len(),
        0
    );
}

#[test]
fn hard_delete_auto_removes_from_index() {
    let dir = tempfile::TempDir::new().unwrap();
    let db = open_test_db(&dir);
    create_docs_table(&db);
    setup_auto_index(&db);

    let id = insert_doc(&db, "Permanent Delete", "Gone forever");
    assert_eq!(
        db.search("docs".into(), "permanent".into(), 10, false)
            .unwrap()
            .len(),
        1
    );

    db.query_hard_delete("docs".into(), id).unwrap();

    assert_eq!(
        db.search("docs".into(), "permanent".into(), 10, false)
            .unwrap()
            .len(),
        0
    );
}

// ─── Non-Indexed Table Unaffected ────────────────────────────────────

#[test]
fn writes_to_non_indexed_table_work_normally() {
    let dir = tempfile::TempDir::new().unwrap();
    let db = open_test_db(&dir);
    create_users_table(&db);
    // No search index for users — auto-index should be a noop

    let id = insert_user(&db, "Alice", "a@t.com", "admin", 90.0);
    let mut data = HashMap::new();
    data.insert("name".into(), SqlValue::Text("Bob".into()));
    db.query_update("users".into(), id.clone(), data).unwrap();
    db.query_delete("users".into(), id).unwrap();
    // No errors — auto-index silently skipped
}

// ─── Index Stats After Auto-Indexing ─────────────────────────────────

#[test]
fn index_stats_reflect_auto_indexed_docs() {
    let dir = tempfile::TempDir::new().unwrap();
    let db = open_test_db(&dir);
    create_docs_table(&db);
    setup_auto_index(&db);

    for i in 0..10 {
        insert_doc(&db, &format!("Doc {}", i), &format!("Content {}", i));
    }

    let stats = db.search_index_stats("docs".into()).unwrap();
    assert_eq!(stats.num_docs, 10);
}
