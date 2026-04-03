use crate::common::*;
use iron_vault_core::api::types::*;
use std::collections::HashMap;

fn setup_versioned_table(db: &iron_vault_core::api::vault::IronVaultDb) {
    db.execute_raw(
        "CREATE TABLE invoices (\
            id TEXT PRIMARY KEY, amount REAL NOT NULL, status TEXT NOT NULL, \
            version INTEGER NOT NULL DEFAULT 1, \
            tenant_id TEXT NOT NULL, created_at INTEGER NOT NULL, \
            updated_at INTEGER NOT NULL, deleted_at INTEGER)"
            .into(),
        vec![],
    )
    .unwrap();
}

fn make_insert(name: &str, email: &str) -> Op {
    let mut data = HashMap::new();
    data.insert("name".into(), SqlValue::Text(name.into()));
    data.insert("email".into(), SqlValue::Text(email.into()));
    data.insert("status".into(), SqlValue::Text("active".into()));
    Op::Insert {
        table: "users".into(),
        data,
    }
}

// ─── Multi-Op Transaction ────────────────────────────────────────────

#[test]
fn five_op_transaction_commits_atomically() {
    let dir = tempfile::TempDir::new().unwrap();
    let db = open_test_db(&dir);
    create_users_table(&db);
    create_orders_table(&db);

    let uid = "user-1".to_string();
    let result = db
        .transaction(vec![
            Op::Insert {
                table: "users".into(),
                data: {
                    let mut d = HashMap::new();
                    d.insert("id".into(), SqlValue::Text(uid.clone()));
                    d.insert("name".into(), SqlValue::Text("Alice".into()));
                    d.insert("email".into(), SqlValue::Text("a@t.com".into()));
                    d.insert("status".into(), SqlValue::Text("active".into()));
                    d
                },
            },
            Op::Insert {
                table: "orders".into(),
                data: {
                    let mut d = HashMap::new();
                    d.insert("user_id".into(), SqlValue::Text(uid.clone()));
                    d.insert("amount".into(), SqlValue::Real(99.99));
                    d.insert("status".into(), SqlValue::Text("pending".into()));
                    d
                },
            },
            Op::Update {
                table: "users".into(),
                id: uid.clone(),
                data: {
                    let mut d = HashMap::new();
                    d.insert("status".into(), SqlValue::Text("has_orders".into()));
                    d
                },
            },
            Op::Insert {
                table: "orders".into(),
                data: {
                    let mut d = HashMap::new();
                    d.insert("user_id".into(), SqlValue::Text(uid.clone()));
                    d.insert("amount".into(), SqlValue::Real(50.0));
                    d.insert("status".into(), SqlValue::Text("pending".into()));
                    d
                },
            },
            Op::Raw {
                sql: "UPDATE users SET score = 42.0 WHERE id = ?1 AND tenant_id = ?2".into(),
                params: vec![
                    SqlValue::Text(uid.clone()),
                    SqlValue::Text("tenant_test".into()),
                ],
            },
        ])
        .unwrap();

    assert_eq!(result.inserted_ids.len(), 3); // 1 user + 2 orders
    assert!(result.affected_tables.contains(&"users".to_string()));
    assert!(result.affected_tables.contains(&"orders".to_string()));
    assert_eq!(result.rows_affected, 5);

    assert_eq!(db.query_count(query("users")).unwrap(), 1);
    assert_eq!(db.query_count(query("orders")).unwrap(), 2);
}

#[test]
fn failure_in_op_3_rolls_back_ops_1_and_2() {
    let dir = tempfile::TempDir::new().unwrap();
    let db = open_test_db(&dir);
    create_users_table(&db);

    let result = db.transaction(vec![
        make_insert("Alice", "a@t.com"),
        make_insert("Bob", "b@t.com"),
        Op::Raw {
            sql: "THIS IS INVALID SQL".into(),
            params: vec![],
        },
        make_insert("Carol", "c@t.com"),
    ]);

    assert!(result.is_err());
    let msg = result.unwrap_err().to_string();
    assert!(msg.contains("TransactionRollbackException"), "got: {}", msg);

    // ALL inserts should be rolled back
    assert_eq!(db.query_count(query("users")).unwrap(), 0);
}

#[test]
fn empty_transaction_succeeds() {
    let dir = tempfile::TempDir::new().unwrap();
    let db = open_test_db(&dir);

    let result = db.transaction(vec![]).unwrap();
    assert!(result.inserted_ids.is_empty());
    assert!(result.affected_tables.is_empty());
    assert_eq!(result.rows_affected, 0);
}

#[test]
fn transaction_enforces_tenant_isolation() {
    let dir = tempfile::TempDir::new().unwrap();
    let path = dir.path().join("shared.db").to_str().unwrap().to_string();
    let db_a = iron_vault_core::api::vault::IronVaultDb::open(
        path.clone(), test_key(), "tenant_a".into(), VaultConfig::test_config(),
    ).unwrap();
    let db_b = iron_vault_core::api::vault::IronVaultDb::open(
        path, test_key(), "tenant_b".into(), VaultConfig::test_config(),
    ).unwrap();

    create_users_table(&db_a);

    // Tenant A inserts via transaction
    db_a.transaction(vec![make_insert("Alice", "a@t.com")]).unwrap();

    // Tenant B should not see tenant A's data
    assert_eq!(db_b.query_count(query("users")).unwrap(), 0);
    assert_eq!(db_a.query_count(query("users")).unwrap(), 1);
}

#[test]
fn transaction_after_close_fails() {
    let dir = tempfile::TempDir::new().unwrap();
    let mut db = open_test_db(&dir);
    db.close().unwrap();

    assert!(db.transaction(vec![]).is_err());
    assert!(db.update_with_version(
        "t".into(), "id".into(), 1, HashMap::new()
    ).is_err());
}

// ─── Savepoints ──────────────────────────────────────────────────────

#[test]
fn savepoint_and_release_preserves_all_ops() {
    let dir = tempfile::TempDir::new().unwrap();
    let db = open_test_db(&dir);
    create_users_table(&db);

    db.transaction(vec![
        make_insert("Alice", "a@t.com"),
        Op::Savepoint { name: "sp1".into() },
        make_insert("Bob", "b@t.com"),
        Op::ReleaseSavepoint { name: "sp1".into() },
        make_insert("Carol", "c@t.com"),
    ])
    .unwrap();

    assert_eq!(db.query_count(query("users")).unwrap(), 3);
}

#[test]
fn rollback_to_savepoint_undoes_partial_ops() {
    let dir = tempfile::TempDir::new().unwrap();
    let db = open_test_db(&dir);
    create_users_table(&db);

    db.transaction(vec![
        make_insert("Alice", "a@t.com"),            // survives
        Op::Savepoint { name: "sp1".into() },
        make_insert("Bob", "b@t.com"),              // rolled back
        make_insert("Carol", "c@t.com"),            // rolled back
        Op::RollbackToSavepoint { name: "sp1".into() },
        Op::ReleaseSavepoint { name: "sp1".into() },
        make_insert("Dave", "d@t.com"),             // survives (after rollback)
    ])
    .unwrap();

    // Alice + Dave survived, Bob + Carol rolled back
    assert_eq!(db.query_count(query("users")).unwrap(), 2);

    let mut spec = query("users");
    spec.order_by.push(OrderBy::Asc { column: "name".into() });
    let rows = db.query_get(spec).unwrap();
    let names: Vec<&str> = rows.iter().filter_map(|r| match r.get("name") {
        Some(SqlValue::Text(s)) => Some(s.as_str()),
        _ => None,
    }).collect();
    assert_eq!(names, vec!["Alice", "Dave"]);
}

#[test]
fn nested_savepoints() {
    let dir = tempfile::TempDir::new().unwrap();
    let db = open_test_db(&dir);
    create_users_table(&db);

    db.transaction(vec![
        make_insert("A", "a@t.com"),
        Op::Savepoint { name: "outer".into() },
        make_insert("B", "b@t.com"),
        Op::Savepoint { name: "inner".into() },
        make_insert("C", "c@t.com"),
        Op::RollbackToSavepoint { name: "inner".into() },  // undo C
        Op::ReleaseSavepoint { name: "inner".into() },
        Op::ReleaseSavepoint { name: "outer".into() },
    ])
    .unwrap();

    // A + B survived, C rolled back
    assert_eq!(db.query_count(query("users")).unwrap(), 2);
}

#[test]
fn invalid_savepoint_name_rejected() {
    let dir = tempfile::TempDir::new().unwrap();
    let db = open_test_db(&dir);

    let result = db.transaction(vec![
        Op::Savepoint { name: "bad; DROP TABLE".into() },
    ]);
    assert!(result.is_err());
}

// ─── Optimistic Locking ─────────────────────────────────────────────

#[test]
fn optimistic_lock_succeeds_on_correct_version() {
    let dir = tempfile::TempDir::new().unwrap();
    let db = open_test_db(&dir);
    setup_versioned_table(&db);

    let mut data = HashMap::new();
    data.insert("amount".into(), SqlValue::Real(100.0));
    data.insert("status".into(), SqlValue::Text("draft".into()));
    let id = db.query_insert("invoices".into(), data).unwrap();

    // Version should be 1 (default)
    let mut update_data = HashMap::new();
    update_data.insert("status".into(), SqlValue::Text("approved".into()));
    db.update_with_version("invoices".into(), id.clone(), 1, update_data)
        .unwrap();

    // Version should now be 2
    let rows = db
        .query_raw(
            "SELECT version, status FROM invoices WHERE id = ?1 AND tenant_id = 'tenant_test'"
                .into(),
            vec![SqlValue::Text(id)],
        )
        .unwrap();
    assert!(matches!(rows[0].get("version"), Some(SqlValue::Integer(2))));
    assert!(matches!(rows[0].get("status"), Some(SqlValue::Text(s)) if s == "approved"));
}

#[test]
fn optimistic_lock_fails_on_wrong_version() {
    let dir = tempfile::TempDir::new().unwrap();
    let db = open_test_db(&dir);
    setup_versioned_table(&db);

    let mut data = HashMap::new();
    data.insert("amount".into(), SqlValue::Real(100.0));
    data.insert("status".into(), SqlValue::Text("draft".into()));
    let id = db.query_insert("invoices".into(), data).unwrap();

    // Try update with wrong version (99 instead of 1)
    let mut update_data = HashMap::new();
    update_data.insert("status".into(), SqlValue::Text("hacked".into()));
    let result = db.update_with_version("invoices".into(), id.clone(), 99, update_data);

    assert!(result.is_err());
    let msg = result.unwrap_err().to_string();
    assert!(msg.contains("OptimisticLockException"), "got: {}", msg);

    // Row should be unchanged
    let rows = db
        .query_raw(
            "SELECT version, status FROM invoices WHERE id = ?1 AND tenant_id = 'tenant_test'"
                .into(),
            vec![SqlValue::Text(id)],
        )
        .unwrap();
    assert!(matches!(rows[0].get("version"), Some(SqlValue::Integer(1))));
    assert!(matches!(rows[0].get("status"), Some(SqlValue::Text(s)) if s == "draft"));
}

#[test]
fn optimistic_lock_sequential_updates() {
    let dir = tempfile::TempDir::new().unwrap();
    let db = open_test_db(&dir);
    setup_versioned_table(&db);

    let mut data = HashMap::new();
    data.insert("amount".into(), SqlValue::Real(100.0));
    data.insert("status".into(), SqlValue::Text("draft".into()));
    let id = db.query_insert("invoices".into(), data).unwrap();

    // Update v1 → v2
    let mut d1 = HashMap::new();
    d1.insert("amount".into(), SqlValue::Real(200.0));
    db.update_with_version("invoices".into(), id.clone(), 1, d1).unwrap();

    // Update v2 → v3
    let mut d2 = HashMap::new();
    d2.insert("amount".into(), SqlValue::Real(300.0));
    db.update_with_version("invoices".into(), id.clone(), 2, d2).unwrap();

    // Stale update at v1 should fail
    let mut d3 = HashMap::new();
    d3.insert("amount".into(), SqlValue::Real(999.0));
    let result = db.update_with_version("invoices".into(), id.clone(), 1, d3);
    assert!(result.is_err());

    // Version should be 3, amount 300
    let rows = db
        .query_raw(
            "SELECT version, amount FROM invoices WHERE id = ?1 AND tenant_id = 'tenant_test'"
                .into(),
            vec![SqlValue::Text(id)],
        )
        .unwrap();
    assert!(matches!(rows[0].get("version"), Some(SqlValue::Integer(3))));
    assert!(matches!(rows[0].get("amount"), Some(SqlValue::Real(v)) if (*v - 300.0).abs() < 0.01));
}

#[test]
fn optimistic_lock_respects_tenant_isolation() {
    let dir = tempfile::TempDir::new().unwrap();
    let path = dir.path().join("shared.db").to_str().unwrap().to_string();
    let db_a = iron_vault_core::api::vault::IronVaultDb::open(
        path.clone(), test_key(), "tenant_a".into(), VaultConfig::test_config(),
    ).unwrap();
    let db_b = iron_vault_core::api::vault::IronVaultDb::open(
        path, test_key(), "tenant_b".into(), VaultConfig::test_config(),
    ).unwrap();

    setup_versioned_table(&db_a);

    let mut data = HashMap::new();
    data.insert("amount".into(), SqlValue::Real(100.0));
    data.insert("status".into(), SqlValue::Text("draft".into()));
    let id = db_a.query_insert("invoices".into(), data).unwrap();

    // Tenant B tries optimistic update on tenant A's row — should fail
    let mut d = HashMap::new();
    d.insert("status".into(), SqlValue::Text("hacked".into()));
    let result = db_b.update_with_version("invoices".into(), id, 1, d);
    assert!(result.is_err());
}

#[test]
fn optimistic_lock_on_soft_deleted_row_fails() {
    let dir = tempfile::TempDir::new().unwrap();
    let db = open_test_db(&dir);
    setup_versioned_table(&db);

    let mut data = HashMap::new();
    data.insert("amount".into(), SqlValue::Real(100.0));
    data.insert("status".into(), SqlValue::Text("draft".into()));
    let id = db.query_insert("invoices".into(), data).unwrap();

    db.query_delete("invoices".into(), id.clone()).unwrap();

    let mut d = HashMap::new();
    d.insert("status".into(), SqlValue::Text("ghost".into()));
    let result = db.update_with_version("invoices".into(), id, 1, d);
    assert!(result.is_err());
}

// ─── Transaction with All Op Types ──────────────────────────────────

#[test]
fn transaction_with_delete_and_hard_delete() {
    let dir = tempfile::TempDir::new().unwrap();
    let db = open_test_db(&dir);
    create_users_table(&db);

    let id1 = insert_user(&db, "Alice", "a@t.com", "admin", 90.0);
    let id2 = insert_user(&db, "Bob", "b@t.com", "member", 80.0);

    db.transaction(vec![
        Op::Delete { table: "users".into(), id: id1.clone() },
        Op::HardDelete { table: "users".into(), id: id2.clone() },
    ])
    .unwrap();

    // id1 soft-deleted (invisible to queries, still in DB)
    assert_eq!(db.query_count(query("users")).unwrap(), 0);
    let raw = db.query_raw(
        "SELECT id FROM users WHERE tenant_id = 'tenant_test'".into(), vec![],
    ).unwrap();
    assert_eq!(raw.len(), 1); // only soft-deleted id1 remains
}

#[test]
fn transaction_with_upsert() {
    let dir = tempfile::TempDir::new().unwrap();
    let db = open_test_db(&dir);
    create_users_table(&db);

    db.transaction(vec![
        Op::Upsert {
            table: "users".into(),
            data: {
                let mut d = HashMap::new();
                d.insert("id".into(), SqlValue::Text("u1".into()));
                d.insert("name".into(), SqlValue::Text("Alice".into()));
                d.insert("email".into(), SqlValue::Text("a@t.com".into()));
                d.insert("status".into(), SqlValue::Text("active".into()));
                d
            },
            conflict_column: "id".into(),
        },
        Op::Upsert {
            table: "users".into(),
            data: {
                let mut d = HashMap::new();
                d.insert("id".into(), SqlValue::Text("u1".into()));
                d.insert("name".into(), SqlValue::Text("Alice Updated".into()));
                d.insert("email".into(), SqlValue::Text("new@t.com".into()));
                d.insert("status".into(), SqlValue::Text("active".into()));
                d
            },
            conflict_column: "id".into(),
        },
    ])
    .unwrap();

    assert_eq!(db.query_count(query("users")).unwrap(), 1);
    let row = db.query_first(query("users")).unwrap().unwrap();
    assert!(matches!(row.get("name"), Some(SqlValue::Text(n)) if n == "Alice Updated"));
}

// ─── Performance ─────────────────────────────────────────────────────

#[test]
fn transaction_10k_inserts_performance() {
    let dir = tempfile::TempDir::new().unwrap();
    let db = open_test_db(&dir);
    create_users_table(&db);

    let ops: Vec<Op> = (0..10_000)
        .map(|i| {
            let mut d = HashMap::new();
            d.insert("name".into(), SqlValue::Text(format!("User{}", i)));
            d.insert("email".into(), SqlValue::Text(format!("u{}@t.com", i)));
            d.insert("status".into(), SqlValue::Text("active".into()));
            Op::Insert {
                table: "users".into(),
                data: d,
            }
        })
        .collect();

    let start = std::time::Instant::now();
    let result = db.transaction(ops).unwrap();
    let elapsed = start.elapsed();

    assert_eq!(result.inserted_ids.len(), 10_000);
    assert_eq!(db.query_count(query("users")).unwrap(), 10_000);

    // Should complete well under 5 seconds (spec says < 200ms on device)
    assert!(
        elapsed.as_secs() < 5,
        "10k inserts took {:?}, expected < 5s",
        elapsed
    );
}

// ─── Edge Cases ──────────────────────────────────────────────────────

#[test]
fn transaction_single_op() {
    let dir = tempfile::TempDir::new().unwrap();
    let db = open_test_db(&dir);
    create_users_table(&db);

    let result = db.transaction(vec![make_insert("Solo", "s@t.com")]).unwrap();
    assert_eq!(result.inserted_ids.len(), 1);
    assert_eq!(db.query_count(query("users")).unwrap(), 1);
}

#[test]
fn transaction_raw_op() {
    let dir = tempfile::TempDir::new().unwrap();
    let db = open_test_db(&dir);
    create_users_table(&db);

    insert_user(&db, "Alice", "a@t.com", "admin", 10.0);

    db.transaction(vec![Op::Raw {
        sql: "UPDATE users SET score = score + ?1 WHERE tenant_id = ?2".into(),
        params: vec![SqlValue::Real(5.0), SqlValue::Text("tenant_test".into())],
    }])
    .unwrap();

    let row = db.query_first(query("users")).unwrap().unwrap();
    assert!(matches!(row.get("score"), Some(SqlValue::Real(v)) if (*v - 15.0).abs() < 0.01));
}

#[test]
fn optimistic_lock_empty_data_only_bumps_version() {
    let dir = tempfile::TempDir::new().unwrap();
    let db = open_test_db(&dir);
    setup_versioned_table(&db);

    let mut data = HashMap::new();
    data.insert("amount".into(), SqlValue::Real(100.0));
    data.insert("status".into(), SqlValue::Text("draft".into()));
    let id = db.query_insert("invoices".into(), data).unwrap();

    // Pass only protected fields (id, tenant_id, version) — stripped, auto updated_at added
    let mut d = HashMap::new();
    d.insert("id".into(), SqlValue::Text("ignored".into()));
    d.insert("tenant_id".into(), SqlValue::Text("ignored".into()));
    d.insert("version".into(), SqlValue::Integer(999));
    db.update_with_version("invoices".into(), id.clone(), 1, d).unwrap();

    let rows = db.query_raw(
        "SELECT version FROM invoices WHERE id = ?1 AND tenant_id = 'tenant_test'".into(),
        vec![SqlValue::Text(id)],
    ).unwrap();
    assert!(matches!(rows[0].get("version"), Some(SqlValue::Integer(2))));
}
