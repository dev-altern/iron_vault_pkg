use crate::common::*;

use iron_vault_core::api::types::*;
use std::collections::HashMap;

// ─── query_first ─────────────────────────────────────────────────────

#[test]
fn query_first_returns_one_row() {
    let dir = tempfile::TempDir::new().unwrap();
    let db = open_test_db(&dir);
    create_users_table(&db);
    insert_user(&db, "Alice", "a@t.com", "admin", 90.0);
    insert_user(&db, "Bob", "b@t.com", "member", 80.0);

    let row = db.query_first(query("users")).unwrap();
    assert!(row.is_some());
}

#[test]
fn query_first_returns_none_on_empty() {
    let dir = tempfile::TempDir::new().unwrap();
    let db = open_test_db(&dir);
    create_users_table(&db);

    let row = db.query_first(query("users")).unwrap();
    assert!(row.is_none());
}

// ─── query_count ─────────────────────────────────────────────────────

#[test]
fn query_count_returns_correct_count() {
    let dir = tempfile::TempDir::new().unwrap();
    let db = open_test_db(&dir);
    create_users_table(&db);

    assert_eq!(db.query_count(query("users")).unwrap(), 0);

    insert_user(&db, "Alice", "a@t.com", "admin", 90.0);
    assert_eq!(db.query_count(query("users")).unwrap(), 1);

    insert_user(&db, "Bob", "b@t.com", "member", 80.0);
    assert_eq!(db.query_count(query("users")).unwrap(), 2);
}

#[test]
fn query_count_respects_conditions() {
    let dir = tempfile::TempDir::new().unwrap();
    let db = open_test_db(&dir);
    create_users_table(&db);
    insert_user(&db, "Alice", "a@t.com", "admin", 90.0);
    insert_user(&db, "Bob", "b@t.com", "member", 80.0);
    insert_user(&db, "Carol", "c@t.com", "admin", 70.0);

    let mut spec = query("users");
    spec.conditions.push(Condition::Eq {
        column: "role".into(),
        value: SqlValue::Text("admin".into()),
    });
    assert_eq!(db.query_count(spec).unwrap(), 2);
}

// ─── query_exists ────────────────────────────────────────────────────

#[test]
fn query_exists_true_and_false() {
    let dir = tempfile::TempDir::new().unwrap();
    let db = open_test_db(&dir);
    create_users_table(&db);

    assert!(!db.query_exists(query("users")).unwrap());

    insert_user(&db, "Alice", "a@t.com", "admin", 90.0);
    assert!(db.query_exists(query("users")).unwrap());
}

// ─── Ordering ────────────────────────────────────────────────────────

#[test]
fn order_by_asc() {
    let dir = tempfile::TempDir::new().unwrap();
    let db = open_test_db(&dir);
    create_users_table(&db);
    insert_user(&db, "Charlie", "c@t.com", "member", 30.0);
    insert_user(&db, "Alice", "a@t.com", "member", 10.0);
    insert_user(&db, "Bob", "b@t.com", "member", 20.0);

    let mut spec = query("users");
    spec.order_by.push(OrderBy::Asc {
        column: "name".into(),
    });
    let rows = db.query_get(spec).unwrap();
    let names: Vec<&str> = rows
        .iter()
        .map(|r| match r.get("name") {
            Some(SqlValue::Text(s)) => s.as_str(),
            _ => "",
        })
        .collect();
    assert_eq!(names, vec!["Alice", "Bob", "Charlie"]);
}

#[test]
fn order_by_desc() {
    let dir = tempfile::TempDir::new().unwrap();
    let db = open_test_db(&dir);
    create_users_table(&db);
    insert_user(&db, "Alice", "a@t.com", "member", 10.0);
    insert_user(&db, "Bob", "b@t.com", "member", 20.0);
    insert_user(&db, "Charlie", "c@t.com", "member", 30.0);

    let mut spec = query("users");
    spec.order_by.push(OrderBy::Desc {
        column: "score".into(),
    });
    let rows = db.query_get(spec).unwrap();
    let scores: Vec<f64> = rows
        .iter()
        .filter_map(|r| match r.get("score") {
            Some(SqlValue::Real(s)) => Some(*s),
            _ => None,
        })
        .collect();
    assert_eq!(scores, vec![30.0, 20.0, 10.0]);
}

#[test]
fn order_by_multiple_columns() {
    let dir = tempfile::TempDir::new().unwrap();
    let db = open_test_db(&dir);
    create_users_table(&db);
    insert_user(&db, "Alice", "a@t.com", "admin", 90.0);
    insert_user(&db, "Bob", "b@t.com", "admin", 80.0);
    insert_user(&db, "Carol", "c@t.com", "member", 95.0);

    let mut spec = query("users");
    spec.order_by.push(OrderBy::Asc {
        column: "role".into(),
    });
    spec.order_by.push(OrderBy::Desc {
        column: "score".into(),
    });
    let rows = db.query_get(spec).unwrap();
    let names: Vec<&str> = rows
        .iter()
        .map(|r| match r.get("name") {
            Some(SqlValue::Text(s)) => s.as_str(),
            _ => "",
        })
        .collect();
    assert_eq!(names, vec!["Alice", "Bob", "Carol"]); // admin(90,80), member(95)
}

// ─── Limit / Offset ─────────────────────────────────────────────────

#[test]
fn limit_restricts_results() {
    let dir = tempfile::TempDir::new().unwrap();
    let db = open_test_db(&dir);
    create_users_table(&db);
    for i in 0..10 {
        insert_user(
            &db,
            &format!("User{}", i),
            &format!("u{}@t.com", i),
            "member",
            i as f64,
        );
    }

    let mut spec = query("users");
    spec.limit = Some(3);
    assert_eq!(db.query_get(spec).unwrap().len(), 3);
}

#[test]
fn offset_skips_rows() {
    let dir = tempfile::TempDir::new().unwrap();
    let db = open_test_db(&dir);
    create_users_table(&db);
    for i in 0..5 {
        insert_user(
            &db,
            &format!("User{}", i),
            &format!("u{}@t.com", i),
            "member",
            i as f64,
        );
    }

    let mut spec = query("users");
    spec.order_by.push(OrderBy::Asc {
        column: "score".into(),
    });
    spec.limit = Some(2);
    spec.offset = Some(3);
    let rows = db.query_get(spec).unwrap();
    assert_eq!(rows.len(), 2);
    // With offset 3, we should get users with score 3.0 and 4.0
    let scores: Vec<f64> = rows
        .iter()
        .filter_map(|r| match r.get("score") {
            Some(SqlValue::Real(s)) => Some(*s),
            _ => None,
        })
        .collect();
    assert_eq!(scores, vec![3.0, 4.0]);
}

// ─── Select Columns ─────────────────────────────────────────────────

#[test]
fn select_specific_columns() {
    let dir = tempfile::TempDir::new().unwrap();
    let db = open_test_db(&dir);
    create_users_table(&db);
    insert_user(&db, "Alice", "a@t.com", "admin", 90.0);

    let mut spec = query("users");
    spec.columns = vec!["name".into(), "role".into()];
    let rows = db.query_get(spec).unwrap();
    assert_eq!(rows.len(), 1);
    assert!(rows[0].contains_key("name"));
    assert!(rows[0].contains_key("role"));
    assert!(!rows[0].contains_key("email")); // not selected
}

// ─── Pagination ──────────────────────────────────────────────────────

#[test]
fn paginate_basic() {
    let dir = tempfile::TempDir::new().unwrap();
    let db = open_test_db(&dir);
    create_users_table(&db);
    for i in 0..50 {
        insert_user(
            &db,
            &format!("User{:02}", i),
            &format!("u{}@t.com", i),
            "member",
            i as f64,
        );
    }

    let page0 = db.query_paginate(query("users"), 0, 20).unwrap();
    assert_eq!(page0.items.len(), 20);
    assert_eq!(page0.total, 50);
    assert_eq!(page0.total_pages, 3);
    assert_eq!(page0.page, 0);
    assert_eq!(page0.page_size, 20);

    let page1 = db.query_paginate(query("users"), 1, 20).unwrap();
    assert_eq!(page1.items.len(), 20);

    let page2 = db.query_paginate(query("users"), 2, 20).unwrap();
    assert_eq!(page2.items.len(), 10); // remainder
}

#[test]
fn paginate_no_overlap() {
    let dir = tempfile::TempDir::new().unwrap();
    let db = open_test_db(&dir);
    create_users_table(&db);
    for i in 0..30 {
        insert_user(
            &db,
            &format!("User{:02}", i),
            &format!("u{}@t.com", i),
            "member",
            i as f64,
        );
    }

    let mut spec = query("users");
    spec.order_by.push(OrderBy::Asc {
        column: "name".into(),
    });

    let p0 = db.query_paginate(spec.clone(), 0, 10).unwrap();
    let p1 = db.query_paginate(spec.clone(), 1, 10).unwrap();
    let p2 = db.query_paginate(spec, 2, 10).unwrap();

    let ids0: Vec<_> = p0.items.iter().filter_map(|r| r.get("id")).collect();
    let ids1: Vec<_> = p1.items.iter().filter_map(|r| r.get("id")).collect();
    let ids2: Vec<_> = p2.items.iter().filter_map(|r| r.get("id")).collect();

    // No overlap between pages
    for id in &ids0 {
        assert!(!ids1.contains(id));
        assert!(!ids2.contains(id));
    }
    for id in &ids1 {
        assert!(!ids2.contains(id));
    }
}

#[test]
fn paginate_zero_page_size_errors() {
    let dir = tempfile::TempDir::new().unwrap();
    let db = open_test_db(&dir);
    create_users_table(&db);

    let result = db.query_paginate(query("users"), 0, 0);
    assert!(result.is_err());
}

// ─── Aggregates ──────────────────────────────────────────────────────

#[test]
fn aggregate_count_sum_avg_min_max() {
    let dir = tempfile::TempDir::new().unwrap();
    let db = open_test_db(&dir);
    create_users_table(&db);
    insert_user(&db, "Alice", "a@t.com", "admin", 100.0);
    insert_user(&db, "Bob", "b@t.com", "member", 200.0);
    insert_user(&db, "Carol", "c@t.com", "member", 300.0);

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
                    alias: "average".into(),
                },
                AggExpr::Min {
                    column: "score".into(),
                    alias: "lowest".into(),
                },
                AggExpr::Max {
                    column: "score".into(),
                    alias: "highest".into(),
                },
            ],
        )
        .unwrap();

    assert!(matches!(result.get("cnt"), Some(SqlValue::Integer(3))));
    assert!(matches!(result.get("total"), Some(SqlValue::Real(v)) if (*v - 600.0).abs() < 0.01));
    assert!(matches!(result.get("average"), Some(SqlValue::Real(v)) if (*v - 200.0).abs() < 0.01));
    assert!(matches!(result.get("lowest"), Some(SqlValue::Real(v)) if (*v - 100.0).abs() < 0.01));
    assert!(matches!(result.get("highest"), Some(SqlValue::Real(v)) if (*v - 300.0).abs() < 0.01));
}

#[test]
fn aggregate_with_conditions() {
    let dir = tempfile::TempDir::new().unwrap();
    let db = open_test_db(&dir);
    create_users_table(&db);
    insert_user(&db, "Alice", "a@t.com", "admin", 100.0);
    insert_user(&db, "Bob", "b@t.com", "member", 200.0);
    insert_user(&db, "Carol", "c@t.com", "member", 300.0);

    let mut spec = query("users");
    spec.conditions.push(Condition::Eq {
        column: "role".into(),
        value: SqlValue::Text("member".into()),
    });

    let result = db
        .query_aggregate(
            spec,
            vec![AggExpr::Count {
                column: "*".into(),
                alias: "cnt".into(),
            }],
        )
        .unwrap();
    assert!(matches!(result.get("cnt"), Some(SqlValue::Integer(2))));
}

// ─── Joins ───────────────────────────────────────────────────────────

#[test]
fn inner_join() {
    let dir = tempfile::TempDir::new().unwrap();
    let db = open_test_db(&dir);
    create_users_table(&db);
    create_orders_table(&db);

    let uid = insert_user(&db, "Alice", "a@t.com", "admin", 90.0);

    let mut order_data = HashMap::new();
    order_data.insert("user_id".into(), SqlValue::Text(uid.clone()));
    order_data.insert("amount".into(), SqlValue::Real(99.99));
    order_data.insert("status".into(), SqlValue::Text("pending".into()));
    db.query_insert("orders".into(), order_data).unwrap();

    let mut spec = query("orders");
    spec.joins.push(JoinSpec::Inner {
        table: "users".into(),
        on: "orders.user_id = users.id AND users.tenant_id = orders.tenant_id".into(),
    });
    spec.columns = vec!["orders.*".into(), "users.name".into()];
    let rows = db.query_get(spec).unwrap();
    assert_eq!(rows.len(), 1);
    assert!(matches!(rows[0].get("name"), Some(SqlValue::Text(n)) if n == "Alice"));
}

#[test]
fn left_join_includes_unmatched() {
    let dir = tempfile::TempDir::new().unwrap();
    let db = open_test_db(&dir);
    create_users_table(&db);
    create_orders_table(&db);

    let uid1 = insert_user(&db, "Alice", "a@t.com", "admin", 90.0);
    let _uid2 = insert_user(&db, "Bob", "b@t.com", "member", 80.0);

    // Only Alice has an order
    let mut order_data = HashMap::new();
    order_data.insert("user_id".into(), SqlValue::Text(uid1));
    order_data.insert("amount".into(), SqlValue::Real(50.0));
    order_data.insert("status".into(), SqlValue::Text("pending".into()));
    db.query_insert("orders".into(), order_data).unwrap();

    let mut spec = query("users");
    spec.joins.push(JoinSpec::Left {
        table: "orders".into(),
        on: "users.id = orders.user_id AND orders.tenant_id = users.tenant_id".into(),
    });
    spec.columns = vec!["users.name".into(), "orders.amount".into()];
    let rows = db.query_get(spec).unwrap();
    assert_eq!(rows.len(), 2); // Both users, Bob has NULL order
}
