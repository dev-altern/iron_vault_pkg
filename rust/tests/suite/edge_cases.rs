use crate::common::*;
use iron_vault_core::api::types::*;
use std::collections::HashMap;

// ─── P2: Data Type Boundaries ────────────────────────────────────────

#[test]
fn integer_boundary_values() {
    let dir = tempfile::TempDir::new().unwrap();
    let db = open_test_db(&dir);
    create_users_table(&db);

    let values = [i64::MIN, i64::MAX, 0i64, -1, 1];
    for (i, &val) in values.iter().enumerate() {
        let mut data = HashMap::new();
        data.insert("name".into(), SqlValue::Text(format!("u{}", i)));
        data.insert("email".into(), SqlValue::Text(format!("u{}@t.com", i)));
        data.insert("status".into(), SqlValue::Text("active".into()));
        data.insert("score".into(), SqlValue::Integer(val));
        // score is REAL in our table, but let's use created_at which is INTEGER
        data.insert("created_at".into(), SqlValue::Integer(val));
        db.query_insert("users".into(), data).unwrap();
    }

    // Query back and verify round-trip
    for (i, &val) in values.iter().enumerate() {
        let mut spec = query("users");
        spec.conditions.push(Condition::Eq {
            column: "name".into(),
            value: SqlValue::Text(format!("u{}", i)),
        });
        let row = db.query_first(spec).unwrap().unwrap();
        if let Some(SqlValue::Integer(v)) = row.get("created_at") {
            assert_eq!(*v, val, "i64 round-trip failed for {}", val);
        } else {
            panic!("Expected integer for created_at");
        }
    }

    // Condition matching on boundary values
    let mut spec = query("users");
    spec.conditions.push(Condition::Eq {
        column: "created_at".into(),
        value: SqlValue::Integer(i64::MAX),
    });
    assert_eq!(db.query_count(spec).unwrap(), 1);
}

#[test]
fn empty_string_round_trips_not_null() {
    let dir = tempfile::TempDir::new().unwrap();
    let db = open_test_db(&dir);
    create_users_table(&db);

    let mut data = HashMap::new();
    data.insert("name".into(), SqlValue::Text("".into())); // empty string
    data.insert("email".into(), SqlValue::Text("e@t.com".into()));
    data.insert("status".into(), SqlValue::Text("active".into()));
    let id = db.query_insert("users".into(), data).unwrap();

    let mut spec = query("users");
    spec.conditions.push(Condition::Eq {
        column: "id".into(),
        value: SqlValue::Text(id),
    });
    let row = db.query_first(spec).unwrap().unwrap();
    match row.get("name") {
        Some(SqlValue::Text(s)) => assert_eq!(s, "", "Empty string must not become NULL"),
        other => panic!("Expected Text(\"\"), got {:?}", other),
    }
}

#[test]
fn unicode_round_trip() {
    let dir = tempfile::TempDir::new().unwrap();
    let db = open_test_db(&dir);
    create_users_table(&db);

    let cases = [
        ("\u{1F600}", "emoji"),              // 😀
        ("\u{4E16}\u{754C}", "chinese"),     // 世界
        ("caf\u{00E9}", "accented"),         // café (precomposed)
        ("cafe\u{0301}", "combining"),       // cafe + combining accent
        ("\u{200D}", "zwj"),                 // zero-width joiner
        ("\u{202E}RLO", "rtl_override"),     // right-to-left override
        ("line1\nline2\ttab", "whitespace"), // newlines and tabs
    ];

    for (val, label) in &cases {
        let mut data = HashMap::new();
        data.insert("name".into(), SqlValue::Text(val.to_string()));
        data.insert("email".into(), SqlValue::Text(format!("{}@t.com", label)));
        data.insert("status".into(), SqlValue::Text("active".into()));
        let id = db.query_insert("users".into(), data).unwrap();

        let mut spec = query("users");
        spec.conditions.push(Condition::Eq {
            column: "id".into(),
            value: SqlValue::Text(id),
        });
        let row = db.query_first(spec).unwrap().unwrap();
        if let Some(SqlValue::Text(s)) = row.get("name") {
            assert_eq!(s, val, "Unicode round-trip failed for {}", label);
        } else {
            panic!("Expected text for {}", label);
        }
    }
}

#[test]
fn empty_blob_round_trips() {
    let dir = tempfile::TempDir::new().unwrap();
    let db = open_test_db(&dir);
    db.execute_raw(
        "CREATE TABLE blob_edge (id TEXT PRIMARY KEY, data BLOB, tenant_id TEXT NOT NULL, \
         created_at INTEGER NOT NULL, updated_at INTEGER NOT NULL, deleted_at INTEGER)"
            .into(),
        vec![],
    )
    .unwrap();

    let mut data = HashMap::new();
    data.insert("data".into(), SqlValue::Blob(vec![]));
    let id = db.query_insert("blob_edge".into(), data).unwrap();

    let rows = db
        .query_raw(
            "SELECT data FROM blob_edge WHERE id = ?1".into(),
            vec![SqlValue::Text(id)],
        )
        .unwrap();
    match rows[0].get("data") {
        Some(SqlValue::Blob(b)) => assert!(b.is_empty(), "Empty blob must round-trip"),
        other => panic!("Expected empty blob, got {:?}", other),
    }
}

#[test]
fn null_in_not_null_column_errors() {
    let dir = tempfile::TempDir::new().unwrap();
    let db = open_test_db(&dir);
    create_users_table(&db);

    let mut data = HashMap::new();
    data.insert("name".into(), SqlValue::Null); // name is NOT NULL
    data.insert("email".into(), SqlValue::Text("e@t.com".into()));
    data.insert("status".into(), SqlValue::Text("active".into()));
    let result = db.query_insert("users".into(), data);
    assert!(result.is_err(), "NULL in NOT NULL column must error");
}

// ─── P3: Pagination Edge Cases ───────────────────────────────────────

#[test]
fn paginate_beyond_last_page() {
    let dir = tempfile::TempDir::new().unwrap();
    let db = open_test_db(&dir);
    create_users_table(&db);
    for i in 0..5 {
        insert_user(
            &db,
            &format!("U{}", i),
            &format!("u{}@t.com", i),
            "m",
            i as f64,
        );
    }

    let page = db.query_paginate(query("users"), 100, 10).unwrap();
    assert!(page.items.is_empty());
    assert_eq!(page.total, 5);
    assert_eq!(page.total_pages, 1);
    assert_eq!(page.page, 100);
}

#[test]
fn paginate_page_size_one() {
    let dir = tempfile::TempDir::new().unwrap();
    let db = open_test_db(&dir);
    create_users_table(&db);
    for i in 0..5 {
        insert_user(
            &db,
            &format!("U{}", i),
            &format!("u{}@t.com", i),
            "m",
            i as f64,
        );
    }

    let page = db.query_paginate(query("users"), 0, 1).unwrap();
    assert_eq!(page.total_pages, 5);
    assert_eq!(page.items.len(), 1);

    // Every page should have exactly 1 item
    for p in 0..5 {
        let page = db.query_paginate(query("users"), p, 1).unwrap();
        assert_eq!(page.items.len(), 1, "Page {} should have 1 item", p);
    }
}

#[test]
fn paginate_single_item() {
    let dir = tempfile::TempDir::new().unwrap();
    let db = open_test_db(&dir);
    create_users_table(&db);
    insert_user(&db, "Solo", "s@t.com", "m", 1.0);

    let p0 = db.query_paginate(query("users"), 0, 10).unwrap();
    assert_eq!(p0.total, 1);
    assert_eq!(p0.total_pages, 1);
    assert_eq!(p0.items.len(), 1);

    let p1 = db.query_paginate(query("users"), 1, 10).unwrap();
    assert!(p1.items.is_empty());
}

#[test]
fn paginate_empty_table() {
    let dir = tempfile::TempDir::new().unwrap();
    let db = open_test_db(&dir);
    create_users_table(&db);

    let page = db.query_paginate(query("users"), 0, 10).unwrap();
    assert_eq!(page.total, 0);
    assert_eq!(page.total_pages, 0);
    assert!(page.items.is_empty());
}

// ─── P4: Aggregate Edge Cases ────────────────────────────────────────

#[test]
fn aggregate_on_empty_table() {
    let dir = tempfile::TempDir::new().unwrap();
    let db = open_test_db(&dir);
    create_users_table(&db);

    let result = db
        .query_aggregate(
            query("users"),
            vec![
                AggExpr::Count {
                    column: "*".into(),
                    alias: "cnt".into(),
                },
                AggExpr::Sum {
                    column: "score".into(),
                    alias: "total".into(),
                },
                AggExpr::Avg {
                    column: "score".into(),
                    alias: "avg_score".into(),
                },
                AggExpr::Min {
                    column: "score".into(),
                    alias: "min_score".into(),
                },
                AggExpr::Max {
                    column: "score".into(),
                    alias: "max_score".into(),
                },
            ],
        )
        .unwrap();

    assert!(matches!(result.get("cnt"), Some(SqlValue::Integer(0))));
    // SUM/AVG/MIN/MAX on empty set return NULL in SQLite
    assert!(matches!(result.get("total"), Some(SqlValue::Null)));
    assert!(matches!(result.get("avg_score"), Some(SqlValue::Null)));
    assert!(matches!(result.get("min_score"), Some(SqlValue::Null)));
    assert!(matches!(result.get("max_score"), Some(SqlValue::Null)));
}

// ─── P5: Batch Edge Cases ────────────────────────────────────────────

#[test]
fn batch_insert_empty_succeeds() {
    let dir = tempfile::TempDir::new().unwrap();
    let db = open_test_db(&dir);
    create_users_table(&db);

    let ids = db.query_insert_batch("users".into(), vec![]).unwrap();
    assert!(ids.is_empty());
    assert_eq!(db.query_count(query("users")).unwrap(), 0);
}

#[test]
fn batch_update_empty_succeeds() {
    let dir = tempfile::TempDir::new().unwrap();
    let db = open_test_db(&dir);
    create_users_table(&db);

    let affected = db.query_update_batch("users".into(), vec![]).unwrap();
    assert_eq!(affected, 0);
}

#[test]
fn batch_delete_empty_succeeds() {
    let dir = tempfile::TempDir::new().unwrap();
    let db = open_test_db(&dir);
    create_users_table(&db);

    let affected = db.query_delete_batch("users".into(), vec![]).unwrap();
    assert_eq!(affected, 0);
}

#[test]
fn batch_delete_mix_of_active_and_already_deleted() {
    let dir = tempfile::TempDir::new().unwrap();
    let db = open_test_db(&dir);
    create_users_table(&db);
    let id1 = insert_user(&db, "A", "a@t.com", "m", 1.0);
    let id2 = insert_user(&db, "B", "b@t.com", "m", 2.0);
    let id3 = insert_user(&db, "C", "c@t.com", "m", 3.0);

    // Soft-delete id1 first
    db.query_delete("users".into(), id1.clone()).unwrap();

    // Batch delete all three — only id2 and id3 should be affected
    let affected = db
        .query_delete_batch("users".into(), vec![id1, id2, id3])
        .unwrap();
    assert_eq!(affected, 2, "Already-deleted row should not count");
}

// ─── P5: NotIn empty returns all ─────────────────────────────────────

#[test]
fn not_in_empty_returns_all_rows() {
    let dir = tempfile::TempDir::new().unwrap();
    let db = open_test_db(&dir);
    create_users_table(&db);
    insert_user(&db, "A", "a@t.com", "admin", 1.0);
    insert_user(&db, "B", "b@t.com", "member", 2.0);
    insert_user(&db, "C", "c@t.com", "viewer", 3.0);

    let mut spec = query("users");
    spec.conditions.push(Condition::NotIn {
        column: "role".into(),
        values: vec![], // empty NOT IN → always true
    });
    assert_eq!(db.query_get(spec).unwrap().len(), 3);
}

// ─── P9: OrderBy::Raw ────────────────────────────────────────────────

#[test]
fn order_by_raw_expression() {
    let dir = tempfile::TempDir::new().unwrap();
    let db = open_test_db(&dir);
    create_users_table(&db);
    insert_user(&db, "Alice", "a@t.com", "m", 1.0);
    insert_user(&db, "Bo", "b@t.com", "m", 2.0);
    insert_user(&db, "Charlotte", "c@t.com", "m", 3.0);

    let mut spec = query("users");
    spec.order_by.push(OrderBy::Raw {
        expression: "length(name) ASC".into(),
    });
    let rows = db.query_get(spec).unwrap();
    let names: Vec<&str> = rows
        .iter()
        .filter_map(|r| match r.get("name") {
            Some(SqlValue::Text(s)) => Some(s.as_str()),
            _ => None,
        })
        .collect();
    assert_eq!(names, vec!["Bo", "Alice", "Charlotte"]);
}

// ─── P10: Upsert with conflict on id column ─────────────────────────

#[test]
fn upsert_with_conflict_on_id() {
    let dir = tempfile::TempDir::new().unwrap();
    let db = open_test_db(&dir);
    create_users_table(&db);

    let mut data1 = HashMap::new();
    data1.insert("id".into(), SqlValue::Text("fixed-id".into()));
    data1.insert("name".into(), SqlValue::Text("Alice".into()));
    data1.insert("email".into(), SqlValue::Text("a@t.com".into()));
    data1.insert("status".into(), SqlValue::Text("active".into()));
    data1.insert("score".into(), SqlValue::Real(80.0));
    db.query_upsert("users".into(), data1, "id".into()).unwrap();

    // Second upsert with same id — should update
    let mut data2 = HashMap::new();
    data2.insert("id".into(), SqlValue::Text("fixed-id".into()));
    data2.insert("name".into(), SqlValue::Text("Updated".into()));
    data2.insert("email".into(), SqlValue::Text("new@t.com".into()));
    data2.insert("status".into(), SqlValue::Text("active".into()));
    data2.insert("score".into(), SqlValue::Real(99.0));
    db.query_upsert("users".into(), data2, "id".into()).unwrap();

    assert_eq!(db.query_count(query("users")).unwrap(), 1);
    let row = db.query_first(query("users")).unwrap().unwrap();
    assert!(matches!(row.get("name"), Some(SqlValue::Text(n)) if n == "Updated"));
}
