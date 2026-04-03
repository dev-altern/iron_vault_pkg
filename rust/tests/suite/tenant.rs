use crate::common::*;

use iron_vault_core::api::types::*;
use std::collections::HashMap;

// ─── Tenant Isolation ────────────────────────────────────────────────

#[test]
fn tenant_id_auto_injected_on_insert() {
    let dir = tempfile::TempDir::new().unwrap();
    let db = open_test_db(&dir);
    create_users_table(&db);

    let id = insert_user(&db, "Alice", "a@t.com", "admin", 90.0);

    // Verify tenant_id was set by checking with raw SQL (bypasses tenant filter)
    let rows = db
        .query_raw(
            "SELECT tenant_id FROM users WHERE id = ?1".into(),
            vec![SqlValue::Text(id)],
        )
        .unwrap();
    assert!(matches!(
        rows[0].get("tenant_id"),
        Some(SqlValue::Text(t)) if t == "tenant_test"
    ));
}

#[test]
fn tenant_id_cannot_be_forged_by_caller() {
    let dir = tempfile::TempDir::new().unwrap();
    let db = open_test_db(&dir);
    create_users_table(&db);

    // Try to insert with a different tenant_id — should be overwritten
    let mut data = HashMap::new();
    data.insert("name".into(), SqlValue::Text("Hacker".into()));
    data.insert("email".into(), SqlValue::Text("h@evil.com".into()));
    data.insert("status".into(), SqlValue::Text("active".into()));
    data.insert("tenant_id".into(), SqlValue::Text("evil_tenant".into()));
    let id = db.query_insert("users".into(), data).unwrap();

    // Verify the stored tenant_id is the DbContext one, not the forged one
    let rows = db
        .query_raw(
            "SELECT tenant_id FROM users WHERE id = ?1".into(),
            vec![SqlValue::Text(id)],
        )
        .unwrap();
    assert!(matches!(
        rows[0].get("tenant_id"),
        Some(SqlValue::Text(t)) if t == "tenant_test"
    ));
}

#[test]
fn query_only_returns_own_tenant_data() {
    let dir = tempfile::TempDir::new().unwrap();

    // Two DB handles with different tenant_ids on the same file
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

    // Insert data for each tenant
    insert_user_on(&db_a, "Alice");
    insert_user_on(&db_a, "AliceTwo");
    insert_user_on(&db_b, "Bob");

    // Each tenant should only see their own data
    let a_rows = db_a.query_get(query("users")).unwrap();
    let b_rows = db_b.query_get(query("users")).unwrap();
    assert_eq!(a_rows.len(), 2, "Tenant A should see 2 users");
    assert_eq!(b_rows.len(), 1, "Tenant B should see 1 user");

    // Count is also tenant-scoped
    assert_eq!(db_a.query_count(query("users")).unwrap(), 2);
    assert_eq!(db_b.query_count(query("users")).unwrap(), 1);
}

#[test]
fn update_only_affects_own_tenant() {
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
    let id_a = insert_user_on(&db_a, "Alice");
    let _id_b = insert_user_on(&db_b, "Bob");

    // Tenant B tries to update tenant A's row — should affect 0 rows
    let mut data = HashMap::new();
    data.insert("name".into(), SqlValue::Text("HACKED".into()));
    let affected = db_b
        .query_update("users".into(), id_a.clone(), data)
        .unwrap();
    assert_eq!(
        affected, 0,
        "Tenant B should not be able to update tenant A's row"
    );

    // Verify Alice's name is unchanged
    let rows = db_a
        .query_raw(
            "SELECT name FROM users WHERE id = ?1 AND tenant_id = 'tenant_a'".into(),
            vec![SqlValue::Text(id_a)],
        )
        .unwrap();
    assert!(matches!(rows[0].get("name"), Some(SqlValue::Text(n)) if n == "Alice"));
}

#[test]
fn delete_only_affects_own_tenant() {
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
    let id_a = insert_user_on(&db_a, "Alice");

    // Tenant B tries to delete tenant A's row
    let affected = db_b.query_delete("users".into(), id_a.clone()).unwrap();
    assert_eq!(
        affected, 0,
        "Tenant B should not be able to delete tenant A's row"
    );

    // Alice should still exist for tenant A
    assert_eq!(db_a.query_count(query("users")).unwrap(), 1);
}

// ─── Soft Delete ─────────────────────────────────────────────────────

#[test]
fn soft_delete_sets_deleted_at() {
    let dir = tempfile::TempDir::new().unwrap();
    let db = open_test_db(&dir);
    create_users_table(&db);
    let id = insert_user(&db, "Alice", "a@t.com", "admin", 90.0);

    let affected = db.query_delete("users".into(), id.clone()).unwrap();
    assert_eq!(affected, 1);

    // Row should be invisible to normal queries
    assert_eq!(db.query_count(query("users")).unwrap(), 0);

    // But still in the database (check with raw SQL)
    let rows = db
        .query_raw(
            "SELECT deleted_at FROM users WHERE id = ?1".into(),
            vec![SqlValue::Text(id)],
        )
        .unwrap();
    assert_eq!(rows.len(), 1);
    assert!(matches!(
        rows[0].get("deleted_at"),
        Some(SqlValue::Integer(_))
    ));
}

#[test]
fn soft_deleted_rows_hidden_from_query_get() {
    let dir = tempfile::TempDir::new().unwrap();
    let db = open_test_db(&dir);
    create_users_table(&db);
    let id1 = insert_user(&db, "Alice", "a@t.com", "admin", 90.0);
    let _id2 = insert_user(&db, "Bob", "b@t.com", "member", 80.0);

    db.query_delete("users".into(), id1).unwrap();

    let rows = db.query_get(query("users")).unwrap();
    assert_eq!(rows.len(), 1); // Only Bob
}

#[test]
fn include_deleted_shows_soft_deleted_rows() {
    let dir = tempfile::TempDir::new().unwrap();
    let db = open_test_db(&dir);
    create_users_table(&db);
    let id = insert_user(&db, "Alice", "a@t.com", "admin", 90.0);
    insert_user(&db, "Bob", "b@t.com", "member", 80.0);

    db.query_delete("users".into(), id).unwrap();

    let mut spec = query("users");
    spec.include_deleted = true;
    let rows = db.query_get(spec).unwrap();
    assert_eq!(rows.len(), 2); // Both Alice and Bob
}

#[test]
fn cannot_update_soft_deleted_row() {
    let dir = tempfile::TempDir::new().unwrap();
    let db = open_test_db(&dir);
    create_users_table(&db);
    let id = insert_user(&db, "Alice", "a@t.com", "admin", 90.0);

    db.query_delete("users".into(), id.clone()).unwrap();

    // Try to update deleted row — should affect 0 rows
    let mut data = HashMap::new();
    data.insert("name".into(), SqlValue::Text("Ghost".into()));
    let affected = db.query_update("users".into(), id, data).unwrap();
    assert_eq!(affected, 0);
}

#[test]
fn cannot_soft_delete_already_deleted_row() {
    let dir = tempfile::TempDir::new().unwrap();
    let db = open_test_db(&dir);
    create_users_table(&db);
    let id = insert_user(&db, "Alice", "a@t.com", "admin", 90.0);

    db.query_delete("users".into(), id.clone()).unwrap();

    // Second soft-delete should affect 0 rows
    let affected = db.query_delete("users".into(), id).unwrap();
    assert_eq!(affected, 0);
}

#[test]
fn hard_delete_permanently_removes() {
    let dir = tempfile::TempDir::new().unwrap();
    let db = open_test_db(&dir);
    create_users_table(&db);
    let id = insert_user(&db, "Alice", "a@t.com", "admin", 90.0);

    let affected = db.query_hard_delete("users".into(), id.clone()).unwrap();
    assert_eq!(affected, 1);

    // Row should be completely gone (even raw query finds nothing)
    let rows = db
        .query_raw(
            "SELECT * FROM users WHERE id = ?1".into(),
            vec![SqlValue::Text(id)],
        )
        .unwrap();
    assert!(rows.is_empty());
}

// ─── P1: Hard delete after soft delete (GDPR purge) ──────────────────

#[test]
fn hard_delete_after_soft_delete_succeeds() {
    let dir = tempfile::TempDir::new().unwrap();
    let db = open_test_db(&dir);
    create_users_table(&db);
    let id = insert_user(&db, "Alice", "a@t.com", "admin", 90.0);

    db.query_delete("users".into(), id.clone()).unwrap();
    let affected = db.query_hard_delete("users".into(), id.clone()).unwrap();
    assert_eq!(affected, 1, "hard_delete should work on soft-deleted rows");

    let rows = db
        .query_raw(
            "SELECT * FROM users WHERE id = ?1".into(),
            vec![SqlValue::Text(id)],
        )
        .unwrap();
    assert!(rows.is_empty(), "Row should be permanently gone");
}

// ─── P1: Cross-tenant hard delete isolation ──────────────────────────

#[test]
fn cross_tenant_hard_delete_blocked() {
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
    let id_a = insert_user_on(&db_a, "Alice");

    let affected = db_b
        .query_hard_delete("users".into(), id_a.clone())
        .unwrap();
    assert_eq!(affected, 0, "Tenant B must not hard-delete tenant A's row");

    // Verify row still exists for tenant A
    assert_eq!(db_a.query_count(query("users")).unwrap(), 1);
}

// ─── P1: Cross-tenant batch isolation ────────────────────────────────

#[test]
fn cross_tenant_batch_update_blocked() {
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
    let id_a = insert_user_on(&db_a, "Alice");

    let updates = vec![iron_vault_core::api::types::UpdateEntry {
        id: id_a,
        data: {
            let mut d = HashMap::new();
            d.insert("name".into(), SqlValue::Text("HACKED".into()));
            d
        },
    }];
    let affected = db_b.query_update_batch("users".into(), updates).unwrap();
    assert_eq!(
        affected, 0,
        "Tenant B batch_update must not affect tenant A"
    );
}

#[test]
fn cross_tenant_batch_delete_blocked() {
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
    let id_a = insert_user_on(&db_a, "Alice");

    let affected = db_b.query_delete_batch("users".into(), vec![id_a]).unwrap();
    assert_eq!(
        affected, 0,
        "Tenant B batch_delete must not affect tenant A"
    );
    assert_eq!(db_a.query_count(query("users")).unwrap(), 1);
}

// ─── P1: All query ops fail after close ──────────────────────────────

#[test]
fn query_builder_ops_fail_after_close() {
    let dir = tempfile::TempDir::new().unwrap();
    let mut db = open_test_db(&dir);
    create_users_table(&db);
    db.close().unwrap();

    assert!(db.query_get(query("users")).is_err());
    assert!(db.query_first(query("users")).is_err());
    assert!(db.query_count(query("users")).is_err());
    assert!(db.query_exists(query("users")).is_err());
    assert!(db.query_paginate(query("users"), 0, 10).is_err());
    assert!(db
        .query_aggregate(
            query("users"),
            vec![AggExpr::Count {
                column: "*".into(),
                alias: "c".into()
            },]
        )
        .is_err());
    assert!(db.query_insert("users".into(), HashMap::new()).is_err());
    assert!(db
        .query_update("users".into(), "x".into(), {
            let mut d = HashMap::new();
            d.insert("name".into(), SqlValue::Text("x".into()));
            d
        })
        .is_err());
    assert!(db.query_delete("users".into(), "x".into()).is_err());
    assert!(db.query_hard_delete("users".into(), "x".into()).is_err());
    assert!(db.query_insert_batch("users".into(), vec![]).is_err());
    assert!(db.query_update_batch("users".into(), vec![]).is_err());
    assert!(db.query_delete_batch("users".into(), vec![]).is_err());
}

// ─── P1: All rows soft-deleted → all queries return empty ────────────

#[test]
fn all_rows_deleted_returns_empty() {
    let dir = tempfile::TempDir::new().unwrap();
    let db = open_test_db(&dir);
    create_users_table(&db);
    let id1 = insert_user(&db, "A", "a@t.com", "m", 1.0);
    let id2 = insert_user(&db, "B", "b@t.com", "m", 2.0);

    db.query_delete("users".into(), id1).unwrap();
    db.query_delete("users".into(), id2).unwrap();

    assert!(db.query_get(query("users")).unwrap().is_empty());
    assert!(db.query_first(query("users")).unwrap().is_none());
    assert_eq!(db.query_count(query("users")).unwrap(), 0);
    assert!(!db.query_exists(query("users")).unwrap());
    let page = db.query_paginate(query("users"), 0, 10).unwrap();
    assert_eq!(page.total, 0);
    assert_eq!(page.total_pages, 0);
    assert!(page.items.is_empty());
}

// ─── Helpers ─────────────────────────────────────────────────────────

fn insert_user_on(db: &iron_vault_core::api::vault::IronVaultDb, name: &str) -> String {
    let mut data = HashMap::new();
    data.insert("name".into(), SqlValue::Text(name.into()));
    data.insert("email".into(), SqlValue::Text(format!("{}@t.com", name)));
    data.insert("status".into(), SqlValue::Text("active".into()));
    db.query_insert("users".into(), data).unwrap()
}
