use crate::common::*;

use iron_vault_core::api::types::*;
use std::collections::HashMap;

// ─── Condition Types ─────────────────────────────────────────────────

#[test]
fn condition_eq() {
    let dir = tempfile::TempDir::new().unwrap();
    let db = open_test_db(&dir);
    create_users_table(&db);
    insert_user(&db, "Alice", "alice@test.com", "admin", 95.0);
    insert_user(&db, "Bob", "bob@test.com", "member", 80.0);

    let mut spec = query("users");
    spec.conditions.push(Condition::Eq {
        column: "name".into(),
        value: SqlValue::Text("Alice".into()),
    });
    let rows = db.query_get(spec).unwrap();
    assert_eq!(rows.len(), 1);
    assert!(matches!(rows[0].get("name"), Some(SqlValue::Text(n)) if n == "Alice"));
}

#[test]
fn condition_not_eq() {
    let dir = tempfile::TempDir::new().unwrap();
    let db = open_test_db(&dir);
    create_users_table(&db);
    insert_user(&db, "Alice", "alice@test.com", "admin", 95.0);
    insert_user(&db, "Bob", "bob@test.com", "member", 80.0);

    let mut spec = query("users");
    spec.conditions.push(Condition::NotEq {
        column: "name".into(),
        value: SqlValue::Text("Alice".into()),
    });
    let rows = db.query_get(spec).unwrap();
    assert_eq!(rows.len(), 1);
    assert!(matches!(rows[0].get("name"), Some(SqlValue::Text(n)) if n == "Bob"));
}

#[test]
fn condition_gt_gte_lt_lte() {
    let dir = tempfile::TempDir::new().unwrap();
    let db = open_test_db(&dir);
    create_users_table(&db);
    insert_user(&db, "Low", "lo@t.com", "member", 10.0);
    insert_user(&db, "Mid", "mi@t.com", "member", 50.0);
    insert_user(&db, "High", "hi@t.com", "member", 90.0);

    // GT
    let mut spec = query("users");
    spec.conditions.push(Condition::Gt {
        column: "score".into(),
        value: SqlValue::Real(50.0),
    });
    assert_eq!(db.query_get(spec).unwrap().len(), 1); // High only

    // GTE
    let mut spec = query("users");
    spec.conditions.push(Condition::Gte {
        column: "score".into(),
        value: SqlValue::Real(50.0),
    });
    assert_eq!(db.query_get(spec).unwrap().len(), 2); // Mid + High

    // LT
    let mut spec = query("users");
    spec.conditions.push(Condition::Lt {
        column: "score".into(),
        value: SqlValue::Real(50.0),
    });
    assert_eq!(db.query_get(spec).unwrap().len(), 1); // Low only

    // LTE
    let mut spec = query("users");
    spec.conditions.push(Condition::Lte {
        column: "score".into(),
        value: SqlValue::Real(50.0),
    });
    assert_eq!(db.query_get(spec).unwrap().len(), 2); // Low + Mid
}

#[test]
fn condition_like() {
    let dir = tempfile::TempDir::new().unwrap();
    let db = open_test_db(&dir);
    create_users_table(&db);
    insert_user(&db, "Alice", "alice@example.com", "member", 0.0);
    insert_user(&db, "Bob", "bob@test.com", "member", 0.0);
    insert_user(&db, "Carol", "carol@example.com", "member", 0.0);

    let mut spec = query("users");
    spec.conditions.push(Condition::Like {
        column: "email".into(),
        pattern: "%@example.com".into(),
    });
    let rows = db.query_get(spec).unwrap();
    assert_eq!(rows.len(), 2);
}

#[test]
fn condition_between() {
    let dir = tempfile::TempDir::new().unwrap();
    let db = open_test_db(&dir);
    create_users_table(&db);
    insert_user(&db, "A", "a@t.com", "m", 10.0);
    insert_user(&db, "B", "b@t.com", "m", 50.0);
    insert_user(&db, "C", "c@t.com", "m", 90.0);

    let mut spec = query("users");
    spec.conditions.push(Condition::Between {
        column: "score".into(),
        low: SqlValue::Real(20.0),
        high: SqlValue::Real(80.0),
    });
    assert_eq!(db.query_get(spec).unwrap().len(), 1); // B only
}

#[test]
fn condition_in() {
    let dir = tempfile::TempDir::new().unwrap();
    let db = open_test_db(&dir);
    create_users_table(&db);
    insert_user(&db, "Alice", "a@t.com", "admin", 0.0);
    insert_user(&db, "Bob", "b@t.com", "member", 0.0);
    insert_user(&db, "Carol", "c@t.com", "viewer", 0.0);

    let mut spec = query("users");
    spec.conditions.push(Condition::In {
        column: "role".into(),
        values: vec![
            SqlValue::Text("admin".into()),
            SqlValue::Text("viewer".into()),
        ],
    });
    let rows = db.query_get(spec).unwrap();
    assert_eq!(rows.len(), 2);
}

#[test]
fn condition_in_empty_returns_nothing() {
    let dir = tempfile::TempDir::new().unwrap();
    let db = open_test_db(&dir);
    create_users_table(&db);
    insert_user(&db, "Alice", "a@t.com", "admin", 0.0);

    let mut spec = query("users");
    spec.conditions.push(Condition::In {
        column: "role".into(),
        values: vec![], // empty IN → 0 results
    });
    assert_eq!(db.query_get(spec).unwrap().len(), 0);
}

#[test]
fn condition_not_in() {
    let dir = tempfile::TempDir::new().unwrap();
    let db = open_test_db(&dir);
    create_users_table(&db);
    insert_user(&db, "Alice", "a@t.com", "admin", 0.0);
    insert_user(&db, "Bob", "b@t.com", "member", 0.0);
    insert_user(&db, "Carol", "c@t.com", "viewer", 0.0);

    let mut spec = query("users");
    spec.conditions.push(Condition::NotIn {
        column: "role".into(),
        values: vec![SqlValue::Text("admin".into())],
    });
    assert_eq!(db.query_get(spec).unwrap().len(), 2); // Bob + Carol
}

#[test]
fn condition_is_null_and_is_not_null() {
    let dir = tempfile::TempDir::new().unwrap();
    let db = open_test_db(&dir);
    create_users_table(&db);

    // Insert with email = NULL
    let mut data = HashMap::new();
    data.insert("name".into(), SqlValue::Text("NoEmail".into()));
    data.insert("email".into(), SqlValue::Null);
    data.insert("status".into(), SqlValue::Text("active".into()));
    db.query_insert("users".into(), data).unwrap();

    insert_user(&db, "WithEmail", "has@email.com", "member", 0.0);

    // IS NULL
    let mut spec = query("users");
    spec.conditions.push(Condition::IsNull {
        column: "email".into(),
    });
    assert_eq!(db.query_get(spec).unwrap().len(), 1);

    // IS NOT NULL
    let mut spec = query("users");
    spec.conditions.push(Condition::IsNotNull {
        column: "email".into(),
    });
    assert_eq!(db.query_get(spec).unwrap().len(), 1);
}

#[test]
fn condition_raw() {
    let dir = tempfile::TempDir::new().unwrap();
    let db = open_test_db(&dir);
    create_users_table(&db);
    insert_user(&db, "Alice", "a@t.com", "admin", 95.0);
    insert_user(&db, "Bob", "b@t.com", "member", 50.0);

    let mut spec = query("users");
    spec.conditions.push(Condition::Raw {
        sql: "score > ? AND role = ?".into(),
        params: vec![SqlValue::Real(60.0), SqlValue::Text("admin".into())],
    });
    let rows = db.query_get(spec).unwrap();
    assert_eq!(rows.len(), 1);
    assert!(matches!(rows[0].get("name"), Some(SqlValue::Text(n)) if n == "Alice"));
}

#[test]
fn multiple_and_conditions() {
    let dir = tempfile::TempDir::new().unwrap();
    let db = open_test_db(&dir);
    create_users_table(&db);
    insert_user(&db, "Alice", "a@t.com", "admin", 95.0);
    insert_user(&db, "Bob", "b@t.com", "admin", 50.0);
    insert_user(&db, "Carol", "c@t.com", "member", 95.0);

    let mut spec = query("users");
    spec.conditions.push(Condition::Eq {
        column: "role".into(),
        value: SqlValue::Text("admin".into()),
    });
    spec.conditions.push(Condition::Gte {
        column: "score".into(),
        value: SqlValue::Real(90.0),
    });
    let rows = db.query_get(spec).unwrap();
    assert_eq!(rows.len(), 1); // Alice only (admin AND score >= 90)
}

#[test]
fn or_conditions() {
    let dir = tempfile::TempDir::new().unwrap();
    let db = open_test_db(&dir);
    create_users_table(&db);
    insert_user(&db, "Alice", "a@t.com", "admin", 30.0);
    insert_user(&db, "Bob", "b@t.com", "member", 95.0);
    insert_user(&db, "Carol", "c@t.com", "member", 30.0);

    // Main conditions: role = 'admin'
    // OR group: score > 90
    // Means: admin OR high-scorer
    let mut spec = query("users");
    spec.conditions.push(Condition::Eq {
        column: "role".into(),
        value: SqlValue::Text("admin".into()),
    });
    spec.or_conditions.push(vec![Condition::Gt {
        column: "score".into(),
        value: SqlValue::Real(90.0),
    }]);
    let rows = db.query_get(spec).unwrap();
    assert_eq!(rows.len(), 2); // Alice (admin) + Bob (high score)
}

#[test]
fn or_conditions_with_multiple_groups() {
    let dir = tempfile::TempDir::new().unwrap();
    let db = open_test_db(&dir);
    create_users_table(&db);
    insert_user(&db, "Alice", "a@t.com", "admin", 30.0);
    insert_user(&db, "Bob", "b@t.com", "member", 95.0);
    insert_user(&db, "Carol", "c@t.com", "viewer", 30.0);

    // (role = admin) OR (role = viewer) OR (score > 90)
    let mut spec = query("users");
    spec.conditions.push(Condition::Eq {
        column: "role".into(),
        value: SqlValue::Text("admin".into()),
    });
    spec.or_conditions.push(vec![Condition::Eq {
        column: "role".into(),
        value: SqlValue::Text("viewer".into()),
    }]);
    spec.or_conditions.push(vec![Condition::Gt {
        column: "score".into(),
        value: SqlValue::Real(90.0),
    }]);
    let rows = db.query_get(spec).unwrap();
    assert_eq!(rows.len(), 3); // all three match
}
