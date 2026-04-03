use crate::common::*;

use iron_vault_core::api::types::*;
use std::collections::HashMap;

// ─── Insert ──────────────────────────────────────────────────────────

#[test]
fn insert_returns_uuid() {
    let dir = tempfile::TempDir::new().unwrap();
    let db = open_test_db(&dir);
    create_users_table(&db);

    let id = insert_user(&db, "Alice", "a@t.com", "admin", 90.0);
    assert!(!id.is_empty());
    // UUID v4 format: 8-4-4-4-12
    assert_eq!(id.len(), 36);
    assert_eq!(id.chars().filter(|c| *c == '-').count(), 4);
}

#[test]
fn insert_auto_sets_timestamps() {
    let dir = tempfile::TempDir::new().unwrap();
    let db = open_test_db(&dir);
    create_users_table(&db);

    let before = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap()
        .as_millis() as i64;

    let id = insert_user(&db, "Alice", "a@t.com", "admin", 90.0);

    let after = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap()
        .as_millis() as i64;

    let rows = db
        .query_raw(
            "SELECT created_at, updated_at FROM users WHERE id = ?1".into(),
            vec![SqlValue::Text(id)],
        )
        .unwrap();
    if let (Some(SqlValue::Integer(created)), Some(SqlValue::Integer(updated))) =
        (rows[0].get("created_at"), rows[0].get("updated_at"))
    {
        assert!(*created >= before && *created <= after);
        assert!(*updated >= before && *updated <= after);
    } else {
        panic!("Expected integer timestamps");
    }
}

#[test]
fn insert_with_explicit_id() {
    let dir = tempfile::TempDir::new().unwrap();
    let db = open_test_db(&dir);
    create_users_table(&db);

    let mut data = HashMap::new();
    data.insert("id".into(), SqlValue::Text("custom-id-123".into()));
    data.insert("name".into(), SqlValue::Text("Alice".into()));
    data.insert("email".into(), SqlValue::Text("a@t.com".into()));
    data.insert("status".into(), SqlValue::Text("active".into()));
    let id = db.query_insert("users".into(), data).unwrap();
    assert_eq!(id, "custom-id-123");
}

// ─── Update ──────────────────────────────────────────────────────────

#[test]
fn update_modifies_row() {
    let dir = tempfile::TempDir::new().unwrap();
    let db = open_test_db(&dir);
    create_users_table(&db);
    let id = insert_user(&db, "Alice", "a@t.com", "admin", 90.0);

    let mut data = HashMap::new();
    data.insert("name".into(), SqlValue::Text("Alicia".into()));
    data.insert("score".into(), SqlValue::Real(99.0));
    let affected = db.query_update("users".into(), id.clone(), data).unwrap();
    assert_eq!(affected, 1);

    let mut spec = query("users");
    spec.conditions.push(Condition::Eq {
        column: "id".into(),
        value: SqlValue::Text(id),
    });
    let row = db.query_first(spec).unwrap().unwrap();
    assert!(matches!(row.get("name"), Some(SqlValue::Text(n)) if n == "Alicia"));
    assert!(matches!(row.get("score"), Some(SqlValue::Real(s)) if (*s - 99.0).abs() < 0.01));
}

#[test]
fn update_auto_sets_updated_at() {
    let dir = tempfile::TempDir::new().unwrap();
    let db = open_test_db(&dir);
    create_users_table(&db);
    let id = insert_user(&db, "Alice", "a@t.com", "admin", 90.0);

    // Sleep briefly to ensure timestamp difference
    std::thread::sleep(std::time::Duration::from_millis(10));

    let before_update = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap()
        .as_millis() as i64;

    let mut data = HashMap::new();
    data.insert("name".into(), SqlValue::Text("Updated".into()));
    db.query_update("users".into(), id.clone(), data).unwrap();

    let rows = db
        .query_raw(
            "SELECT updated_at FROM users WHERE id = ?1".into(),
            vec![SqlValue::Text(id)],
        )
        .unwrap();
    if let Some(SqlValue::Integer(updated)) = rows[0].get("updated_at") {
        assert!(
            *updated >= before_update,
            "updated_at should be >= time before update call"
        );
    }
}

#[test]
fn update_nonexistent_id_returns_zero() {
    let dir = tempfile::TempDir::new().unwrap();
    let db = open_test_db(&dir);
    create_users_table(&db);

    let mut data = HashMap::new();
    data.insert("name".into(), SqlValue::Text("Ghost".into()));
    let affected = db
        .query_update("users".into(), "nonexistent-id".into(), data)
        .unwrap();
    assert_eq!(affected, 0);
}

#[test]
fn update_cannot_change_id_or_tenant() {
    let dir = tempfile::TempDir::new().unwrap();
    let db = open_test_db(&dir);
    create_users_table(&db);
    let id = insert_user(&db, "Alice", "a@t.com", "admin", 90.0);

    // Try to change id and tenant_id — they should be stripped
    let mut data = HashMap::new();
    data.insert("id".into(), SqlValue::Text("new-id".into()));
    data.insert("tenant_id".into(), SqlValue::Text("evil".into()));
    data.insert("name".into(), SqlValue::Text("Updated".into()));
    db.query_update("users".into(), id.clone(), data).unwrap();

    // Verify id and tenant_id are unchanged
    let rows = db
        .query_raw(
            "SELECT id, tenant_id FROM users WHERE id = ?1".into(),
            vec![SqlValue::Text(id.clone())],
        )
        .unwrap();
    assert_eq!(rows.len(), 1);
    assert!(matches!(rows[0].get("id"), Some(SqlValue::Text(i)) if i == &id));
    assert!(matches!(
        rows[0].get("tenant_id"),
        Some(SqlValue::Text(t)) if t == "tenant_test"
    ));
}

#[test]
fn update_empty_data_errors() {
    let dir = tempfile::TempDir::new().unwrap();
    let db = open_test_db(&dir);
    create_users_table(&db);
    let id = insert_user(&db, "Alice", "a@t.com", "admin", 90.0);

    let data = HashMap::new();
    let result = db.query_update("users".into(), id, data);
    assert!(result.is_err());
}

// ─── Upsert ──────────────────────────────────────────────────────────

#[test]
fn upsert_inserts_new_row() {
    let dir = tempfile::TempDir::new().unwrap();
    let db = open_test_db(&dir);
    create_users_table(&db);
    // Add unique index on email alone for upsert conflict detection
    db.execute_raw(
        "CREATE UNIQUE INDEX idx_users_email ON users(email)".into(),
        vec![],
    )
    .unwrap();

    let mut data = HashMap::new();
    data.insert("name".into(), SqlValue::Text("Alice".into()));
    data.insert("email".into(), SqlValue::Text("alice@t.com".into()));
    data.insert("status".into(), SqlValue::Text("active".into()));
    let id = db
        .query_upsert("users".into(), data, "email".into())
        .unwrap();
    assert!(!id.is_empty());
    assert_eq!(db.query_count(query("users")).unwrap(), 1);
}

#[test]
fn upsert_updates_on_conflict() {
    let dir = tempfile::TempDir::new().unwrap();
    let db = open_test_db(&dir);
    create_users_table(&db);
    db.execute_raw(
        "CREATE UNIQUE INDEX idx_users_email ON users(email)".into(),
        vec![],
    )
    .unwrap();

    // First insert
    let mut data1 = HashMap::new();
    data1.insert("name".into(), SqlValue::Text("Alice".into()));
    data1.insert("email".into(), SqlValue::Text("alice@t.com".into()));
    data1.insert("status".into(), SqlValue::Text("active".into()));
    data1.insert("score".into(), SqlValue::Real(80.0));
    db.query_upsert("users".into(), data1, "email".into())
        .unwrap();

    // Upsert with same email — should update
    let mut data2 = HashMap::new();
    data2.insert("name".into(), SqlValue::Text("Alice Updated".into()));
    data2.insert("email".into(), SqlValue::Text("alice@t.com".into()));
    data2.insert("status".into(), SqlValue::Text("active".into()));
    data2.insert("score".into(), SqlValue::Real(99.0));
    db.query_upsert("users".into(), data2, "email".into())
        .unwrap();

    // Should still be 1 row, but updated
    assert_eq!(db.query_count(query("users")).unwrap(), 1);
    let row = db.query_first(query("users")).unwrap().unwrap();
    assert!(matches!(row.get("name"), Some(SqlValue::Text(n)) if n == "Alice Updated"));
    assert!(matches!(row.get("score"), Some(SqlValue::Real(s)) if (*s - 99.0).abs() < 0.01));
}

// ─── Batch Operations ────────────────────────────────────────────────

#[test]
fn batch_insert() {
    let dir = tempfile::TempDir::new().unwrap();
    let db = open_test_db(&dir);
    create_users_table(&db);

    let rows: Vec<HashMap<String, SqlValue>> = (0..100)
        .map(|i| {
            let mut data = HashMap::new();
            data.insert("name".into(), SqlValue::Text(format!("User{}", i)));
            data.insert("email".into(), SqlValue::Text(format!("u{}@t.com", i)));
            data.insert("status".into(), SqlValue::Text("active".into()));
            data
        })
        .collect();

    let ids = db.query_insert_batch("users".into(), rows).unwrap();
    assert_eq!(ids.len(), 100);
    assert_eq!(db.query_count(query("users")).unwrap(), 100);
}

#[test]
fn batch_insert_atomicity_on_failure() {
    let dir = tempfile::TempDir::new().unwrap();
    let db = open_test_db(&dir);
    create_users_table(&db);
    db.execute_raw(
        "CREATE UNIQUE INDEX idx_users_email_tenant ON users(email, tenant_id)".into(),
        vec![],
    )
    .unwrap();

    // Second row has duplicate email — should fail and roll back all
    let rows = vec![
        {
            let mut d = HashMap::new();
            d.insert("name".into(), SqlValue::Text("Alice".into()));
            d.insert("email".into(), SqlValue::Text("dup@t.com".into()));
            d.insert("status".into(), SqlValue::Text("active".into()));
            d
        },
        {
            let mut d = HashMap::new();
            d.insert("name".into(), SqlValue::Text("Bob".into()));
            d.insert("email".into(), SqlValue::Text("dup@t.com".into())); // duplicate!
            d.insert("status".into(), SqlValue::Text("active".into()));
            d
        },
    ];

    let result = db.query_insert_batch("users".into(), rows);
    assert!(result.is_err());

    // Both rows should be rolled back — table should be empty
    assert_eq!(db.query_count(query("users")).unwrap(), 0);
}

#[test]
fn batch_update() {
    let dir = tempfile::TempDir::new().unwrap();
    let db = open_test_db(&dir);
    create_users_table(&db);
    let id1 = insert_user(&db, "Alice", "a@t.com", "admin", 10.0);
    let id2 = insert_user(&db, "Bob", "b@t.com", "member", 20.0);

    let updates = vec![
        UpdateEntry {
            id: id1.clone(),
            data: {
                let mut d = HashMap::new();
                d.insert("score".into(), SqlValue::Real(99.0));
                d
            },
        },
        UpdateEntry {
            id: id2.clone(),
            data: {
                let mut d = HashMap::new();
                d.insert("score".into(), SqlValue::Real(88.0));
                d
            },
        },
    ];

    let affected = db.query_update_batch("users".into(), updates).unwrap();
    assert_eq!(affected, 2);

    let mut spec = query("users");
    spec.conditions.push(Condition::Eq {
        column: "id".into(),
        value: SqlValue::Text(id1),
    });
    let row = db.query_first(spec).unwrap().unwrap();
    assert!(matches!(row.get("score"), Some(SqlValue::Real(s)) if (*s - 99.0).abs() < 0.01));
}

#[test]
fn batch_delete() {
    let dir = tempfile::TempDir::new().unwrap();
    let db = open_test_db(&dir);
    create_users_table(&db);
    let id1 = insert_user(&db, "Alice", "a@t.com", "admin", 10.0);
    let id2 = insert_user(&db, "Bob", "b@t.com", "member", 20.0);
    let _id3 = insert_user(&db, "Carol", "c@t.com", "member", 30.0);

    let affected = db
        .query_delete_batch("users".into(), vec![id1, id2])
        .unwrap();
    assert_eq!(affected, 2);

    // Only Carol should remain visible
    assert_eq!(db.query_count(query("users")).unwrap(), 1);
}

// ─── SQL Injection Prevention ────────────────────────────────────────

#[test]
fn sql_injection_in_table_name_rejected() {
    let dir = tempfile::TempDir::new().unwrap();
    let db = open_test_db(&dir);
    create_users_table(&db);

    let result = db.query_get(QuerySpec {
        table: "users; DROP TABLE users --".into(),
        ..query("users")
    });
    assert!(result.is_err());
}

#[test]
fn sql_injection_in_column_name_rejected() {
    let dir = tempfile::TempDir::new().unwrap();
    let db = open_test_db(&dir);
    create_users_table(&db);

    let mut spec = query("users");
    spec.conditions.push(Condition::Eq {
        column: "name; DROP TABLE users --".into(),
        value: SqlValue::Text("x".into()),
    });
    let result = db.query_get(spec);
    assert!(result.is_err());
}

#[test]
fn sql_injection_payloads_in_values_are_safe() {
    let dir = tempfile::TempDir::new().unwrap();
    let db = open_test_db(&dir);
    create_users_table(&db);
    insert_user(&db, "Alice", "a@t.com", "admin", 90.0);

    let long_string = "a".repeat(100000);
    let payloads = vec![
        "'; DROP TABLE users; --",
        "' OR 1=1 --",
        "' UNION SELECT * FROM sqlite_master --",
        "\x00\x01\x02",
        long_string.as_str(),
        "{\"key\": \"value\"}",
        "<script>alert(1)</script>",
        "NULL",
        "undefined",
        "NaN",
        "1e999",
    ];

    for payload in payloads {
        let mut spec = query("users");
        spec.conditions.push(Condition::Eq {
            column: "email".into(),
            value: SqlValue::Text(payload.into()),
        });
        let result = db.query_get(spec);
        // Must not crash, must not return unexpected rows
        assert!(result.is_ok(), "Payload '{}' caused error", payload);
        assert!(
            result.unwrap().is_empty(),
            "Payload '{}' returned rows",
            payload
        );
    }

    // users table must still exist and have 1 row
    assert_eq!(db.query_count(query("users")).unwrap(), 1);
}

#[test]
fn sql_injection_in_raw_condition_params_are_safe() {
    let dir = tempfile::TempDir::new().unwrap();
    let db = open_test_db(&dir);
    create_users_table(&db);
    insert_user(&db, "Alice", "a@t.com", "admin", 90.0);

    let malicious = "' OR 1=1 --";
    let mut spec = query("users");
    spec.conditions.push(Condition::Raw {
        sql: "email = ?".into(),
        params: vec![SqlValue::Text(malicious.into())],
    });
    let rows = db.query_get(spec).unwrap();
    // Malicious value is parameterized — shouldn't match any row
    assert!(rows.is_empty());

    // Table should be fine
    assert_eq!(db.query_count(query("users")).unwrap(), 1);
}
