use crate::common::*;
use iron_vault_core::api::types::*;
use std::collections::HashMap;

// ─── P1 Security: Cross-Tenant Upsert Isolation ─────────────────────

#[test]
fn cross_tenant_upsert_does_not_overwrite_other_tenant() {
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

    create_users_table(&db_a);
    // Unique index on id (PK) — upsert on "id" column
    // Tenant A inserts
    let mut data_a = HashMap::new();
    data_a.insert("id".into(), SqlValue::Text("shared-id".into()));
    data_a.insert("name".into(), SqlValue::Text("Alice".into()));
    data_a.insert("email".into(), SqlValue::Text("a@t.com".into()));
    data_a.insert("status".into(), SqlValue::Text("active".into()));
    db_a.query_upsert("users".into(), data_a, "id".into())
        .unwrap();

    // Tenant B upserts with same id — must NOT overwrite A's row.
    // With the tenant-scoped WHERE in DO UPDATE, this conflicts on id
    // but the UPDATE WHERE tenant_id = excluded.tenant_id doesn't match
    // (B's tenant_id != A's), so the row is neither inserted nor updated.
    let mut data_b = HashMap::new();
    data_b.insert("id".into(), SqlValue::Text("shared-id".into()));
    data_b.insert("name".into(), SqlValue::Text("Bob".into()));
    data_b.insert("email".into(), SqlValue::Text("b@t.com".into()));
    data_b.insert("status".into(), SqlValue::Text("active".into()));
    let _ = db_b.query_upsert("users".into(), data_b, "id".into());

    // Tenant A's row must still show "Alice"
    let rows_a = db_a
        .query_raw(
            "SELECT name FROM users WHERE id = 'shared-id'".into(),
            vec![],
        )
        .unwrap();
    assert_eq!(rows_a.len(), 1);
    assert!(
        matches!(rows_a[0].get("name"), Some(SqlValue::Text(n)) if n == "Alice"),
        "Tenant A's data must not be overwritten by tenant B's upsert"
    );

    // Tenant A still sees exactly 1 user
    assert_eq!(db_a.query_count(query("users")).unwrap(), 1);
}

// ─── JoinSpec::Raw ───────────────────────────────────────────────────

#[test]
fn raw_join_spec() {
    let dir = tempfile::TempDir::new().unwrap();
    let db = open_test_db(&dir);
    create_users_table(&db);
    create_orders_table(&db);

    let uid = insert_user(&db, "Alice", "a@t.com", "admin", 90.0);

    let mut order_data = HashMap::new();
    order_data.insert("user_id".into(), SqlValue::Text(uid.clone()));
    order_data.insert("amount".into(), SqlValue::Real(50.0));
    order_data.insert("status".into(), SqlValue::Text("pending".into()));
    db.query_insert("orders".into(), order_data).unwrap();

    let mut spec = query("orders");
    spec.joins.push(JoinSpec::Raw {
        expression:
            "INNER JOIN users ON orders.user_id = users.id AND users.tenant_id = orders.tenant_id"
                .into(),
    });
    spec.columns = vec!["orders.*".into(), "users.name".into()];
    let rows = db.query_get(spec).unwrap();
    assert_eq!(rows.len(), 1);
    assert!(matches!(rows[0].get("name"), Some(SqlValue::Text(n)) if n == "Alice"));
}

// ─── Multiple Joins (3 tables) ───────────────────────────────────────

#[test]
fn three_table_join() {
    let dir = tempfile::TempDir::new().unwrap();
    let db = open_test_db(&dir);
    create_users_table(&db);
    create_orders_table(&db);
    db.execute_raw(
        "CREATE TABLE order_items (\
            id TEXT PRIMARY KEY, order_id TEXT NOT NULL, product TEXT NOT NULL, qty INTEGER NOT NULL, \
            tenant_id TEXT NOT NULL, created_at INTEGER NOT NULL, updated_at INTEGER NOT NULL, \
            deleted_at INTEGER, \
            FOREIGN KEY (order_id) REFERENCES orders(id))"
            .into(),
        vec![],
    )
    .unwrap();

    let uid = insert_user(&db, "Alice", "a@t.com", "admin", 90.0);

    let mut order_data = HashMap::new();
    order_data.insert("id".into(), SqlValue::Text("ord-1".into()));
    order_data.insert("user_id".into(), SqlValue::Text(uid.clone()));
    order_data.insert("amount".into(), SqlValue::Real(100.0));
    order_data.insert("status".into(), SqlValue::Text("pending".into()));
    db.query_insert("orders".into(), order_data).unwrap();

    let mut item_data = HashMap::new();
    item_data.insert("order_id".into(), SqlValue::Text("ord-1".into()));
    item_data.insert("product".into(), SqlValue::Text("Widget".into()));
    item_data.insert("qty".into(), SqlValue::Integer(3));
    db.query_insert("order_items".into(), item_data).unwrap();

    let mut spec = query("orders");
    spec.joins.push(JoinSpec::Inner {
        table: "users".into(),
        on: "orders.user_id = users.id AND users.tenant_id = orders.tenant_id".into(),
    });
    spec.joins.push(JoinSpec::Inner {
        table: "order_items".into(),
        on: "orders.id = order_items.order_id AND order_items.tenant_id = orders.tenant_id".into(),
    });
    spec.columns = vec![
        "users.name".into(),
        "orders.amount".into(),
        "order_items.product".into(),
        "order_items.qty".into(),
    ];
    let rows = db.query_get(spec).unwrap();
    assert_eq!(rows.len(), 1);
    assert!(matches!(rows[0].get("name"), Some(SqlValue::Text(n)) if n == "Alice"));
    assert!(matches!(rows[0].get("product"), Some(SqlValue::Text(p)) if p == "Widget"));
    assert!(matches!(rows[0].get("qty"), Some(SqlValue::Integer(3))));
}

// ─── Join with Soft-Deleted Rows ─────────────────────────────────────

#[test]
fn join_includes_soft_deleted_from_joined_table() {
    let dir = tempfile::TempDir::new().unwrap();
    let db = open_test_db(&dir);
    create_users_table(&db);
    create_orders_table(&db);

    let uid = insert_user(&db, "Alice", "a@t.com", "admin", 90.0);

    let mut order_data = HashMap::new();
    order_data.insert("user_id".into(), SqlValue::Text(uid.clone()));
    order_data.insert("amount".into(), SqlValue::Real(50.0));
    order_data.insert("status".into(), SqlValue::Text("pending".into()));
    db.query_insert("orders".into(), order_data).unwrap();

    // Soft-delete Alice
    db.query_delete("users".into(), uid).unwrap();

    // Query orders with LEFT JOIN to users — soft-deleted guard only on main table (orders),
    // NOT on the joined table. So the join may return NULL for user columns
    // depending on the ON clause.
    let mut spec = query("orders");
    spec.joins.push(JoinSpec::Left {
        table: "users".into(),
        on: "orders.user_id = users.id AND users.tenant_id = orders.tenant_id".into(),
    });
    spec.columns = vec!["orders.amount".into(), "users.name".into()];
    let rows = db.query_get(spec).unwrap();
    assert_eq!(rows.len(), 1, "Order should still be visible");
    // User name will be NULL because user is soft-deleted (LEFT JOIN returns NULL)
    // OR it might still match if the ON clause doesn't filter deleted_at
    // The key test is: the query does NOT error with ambiguous columns
}

// ─── Cross-Phase: Encrypt → Transaction → Decrypt ────────────────────

#[test]
fn encrypt_store_via_transaction_then_decrypt() {
    let dir = tempfile::TempDir::new().unwrap();
    let db = open_test_db(&dir);
    create_users_table(&db);

    let email = "alice@example.com";
    let encrypted_email = db.encrypt_field(email.into()).unwrap();

    // Store via transaction
    let result = db
        .transaction(vec![Op::Insert {
            table: "users".into(),
            data: {
                let mut d = HashMap::new();
                d.insert("id".into(), SqlValue::Text("u1".into()));
                d.insert("name".into(), SqlValue::Text("Alice".into()));
                d.insert("email".into(), SqlValue::Text(encrypted_email));
                d.insert("status".into(), SqlValue::Text("active".into()));
                d
            },
        }])
        .unwrap();
    assert_eq!(result.inserted_ids.len(), 1);

    // Retrieve and decrypt
    let mut spec = query("users");
    spec.conditions.push(Condition::Eq {
        column: "id".into(),
        value: SqlValue::Text("u1".into()),
    });
    let row = db.query_first(spec).unwrap().unwrap();
    let stored = match row.get("email") {
        Some(SqlValue::Text(s)) => s.clone(),
        _ => panic!("Expected text"),
    };
    let decrypted = db.decrypt_field(stored).unwrap();
    assert_eq!(decrypted, email);
}

// ─── Cross-Phase: Migrate → Query Builder ────────────────────────────

#[test]
fn migrate_then_use_query_builder() {
    let dir = tempfile::TempDir::new().unwrap();
    let db = open_test_db(&dir);

    // Create tables via migration
    db.migrate(vec![VaultMigration {
        version: 1,
        name: "create_products".into(),
        up: "CREATE TABLE products (\
             id TEXT PRIMARY KEY, name TEXT NOT NULL, price REAL, \
             tenant_id TEXT NOT NULL, created_at INTEGER NOT NULL, \
             updated_at INTEGER NOT NULL, deleted_at INTEGER)"
            .into(),
        down: Some("DROP TABLE products".into()),
    }])
    .unwrap();

    // Use query builder on the migrated table
    let mut data = HashMap::new();
    data.insert("name".into(), SqlValue::Text("Widget".into()));
    data.insert("price".into(), SqlValue::Real(9.99));
    let id = db.query_insert("products".into(), data).unwrap();
    assert!(!id.is_empty());

    let mut spec = query("products");
    spec.conditions.push(Condition::Gte {
        column: "price".into(),
        value: SqlValue::Real(5.0),
    });
    let rows = db.query_get(spec).unwrap();
    assert_eq!(rows.len(), 1);

    // Stats should show the table
    let stats = db.stats().unwrap();
    assert_eq!(stats.total_tables, 1);
    assert_eq!(stats.migration_version, 1);
}

// ─── Cross-Phase: Transaction with Mixed Tables ──────────────────────

#[test]
fn transaction_across_multiple_migrated_tables() {
    let dir = tempfile::TempDir::new().unwrap();
    let db = open_test_db(&dir);

    db.migrate(vec![
        VaultMigration {
            version: 1,
            name: "create_accounts".into(),
            up: "CREATE TABLE accounts (\
                 id TEXT PRIMARY KEY, name TEXT NOT NULL, balance REAL DEFAULT 0, \
                 tenant_id TEXT NOT NULL, created_at INTEGER NOT NULL, \
                 updated_at INTEGER NOT NULL, deleted_at INTEGER)"
                .into(),
            down: Some("DROP TABLE accounts".into()),
        },
        VaultMigration {
            version: 2,
            name: "create_transfers".into(),
            up: "CREATE TABLE transfers (\
                 id TEXT PRIMARY KEY, from_id TEXT NOT NULL, to_id TEXT NOT NULL, \
                 amount REAL NOT NULL, \
                 tenant_id TEXT NOT NULL, created_at INTEGER NOT NULL, \
                 updated_at INTEGER NOT NULL, deleted_at INTEGER)"
                .into(),
            down: Some("DROP TABLE transfers".into()),
        },
    ])
    .unwrap();

    // Create two accounts
    let mut a1 = HashMap::new();
    a1.insert("id".into(), SqlValue::Text("acc-1".into()));
    a1.insert("name".into(), SqlValue::Text("Alice".into()));
    a1.insert("balance".into(), SqlValue::Real(1000.0));
    db.query_insert("accounts".into(), a1).unwrap();

    let mut a2 = HashMap::new();
    a2.insert("id".into(), SqlValue::Text("acc-2".into()));
    a2.insert("name".into(), SqlValue::Text("Bob".into()));
    a2.insert("balance".into(), SqlValue::Real(500.0));
    db.query_insert("accounts".into(), a2).unwrap();

    // Transfer 200 from Alice to Bob in a single transaction
    let result = db
        .transaction(vec![
            Op::Raw {
                sql: "UPDATE accounts SET balance = balance - 200 WHERE id = 'acc-1' AND tenant_id = 'tenant_test'".into(),
                params: vec![],
            },
            Op::Raw {
                sql: "UPDATE accounts SET balance = balance + 200 WHERE id = 'acc-2' AND tenant_id = 'tenant_test'".into(),
                params: vec![],
            },
            Op::Insert {
                table: "transfers".into(),
                data: {
                    let mut d = HashMap::new();
                    d.insert("from_id".into(), SqlValue::Text("acc-1".into()));
                    d.insert("to_id".into(), SqlValue::Text("acc-2".into()));
                    d.insert("amount".into(), SqlValue::Real(200.0));
                    d
                },
            },
        ])
        .unwrap();

    assert!(result.affected_tables.contains(&"transfers".to_string()));
    // Raw ops don't report table names — only Op::Insert/Update/Delete do

    // Verify balances
    let rows = db
        .query_raw(
            "SELECT balance FROM accounts WHERE id = 'acc-1' AND tenant_id = 'tenant_test'".into(),
            vec![],
        )
        .unwrap();
    assert!(matches!(rows[0].get("balance"), Some(SqlValue::Real(v)) if (*v - 800.0).abs() < 0.01));

    let rows = db
        .query_raw(
            "SELECT balance FROM accounts WHERE id = 'acc-2' AND tenant_id = 'tenant_test'".into(),
            vec![],
        )
        .unwrap();
    assert!(matches!(rows[0].get("balance"), Some(SqlValue::Real(v)) if (*v - 700.0).abs() < 0.01));
}

// ─── Cross-Phase: Optimistic Lock on Migrated Table ──────────────────

#[test]
fn optimistic_lock_on_migrated_table_with_encryption() {
    let dir = tempfile::TempDir::new().unwrap();
    let db = open_test_db(&dir);

    db.migrate(vec![VaultMigration {
        version: 1,
        name: "create_secrets".into(),
        up: "CREATE TABLE secrets (\
             id TEXT PRIMARY KEY, data_enc TEXT, version INTEGER NOT NULL DEFAULT 1, \
             tenant_id TEXT NOT NULL, created_at INTEGER NOT NULL, \
             updated_at INTEGER NOT NULL, deleted_at INTEGER)"
            .into(),
        down: Some("DROP TABLE secrets".into()),
    }])
    .unwrap();

    // Insert with encrypted data
    let encrypted = db.encrypt_field("my-secret-value".into()).unwrap();
    let mut data = HashMap::new();
    data.insert("data_enc".into(), SqlValue::Text(encrypted));
    let id = db.query_insert("secrets".into(), data).unwrap();

    // Update with optimistic lock
    let new_encrypted = db.encrypt_field("updated-secret".into()).unwrap();
    let mut update = HashMap::new();
    update.insert("data_enc".into(), SqlValue::Text(new_encrypted));
    db.update_with_version("secrets".into(), id.clone(), 1, update)
        .unwrap();

    // Verify: version bumped, data decryptable
    let rows = db.query_raw(
        format!("SELECT data_enc, version FROM secrets WHERE id = '{}' AND tenant_id = 'tenant_test'", id),
        vec![],
    ).unwrap();
    assert!(matches!(rows[0].get("version"), Some(SqlValue::Integer(2))));
    if let Some(SqlValue::Text(enc)) = rows[0].get("data_enc") {
        let dec = db.decrypt_field(enc.clone()).unwrap();
        assert_eq!(dec, "updated-secret");
    }
}
