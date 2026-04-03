/// Shared test helpers for Phase 2+ integration tests.
use iron_vault_core::api::types::*;
use iron_vault_core::api::vault::IronVaultDb;

pub fn test_key() -> Vec<u8> {
    vec![0xABu8; 32]
}

pub fn open_test_db(dir: &tempfile::TempDir) -> IronVaultDb {
    open_test_db_with_tenant(dir, "tenant_test")
}

pub fn open_test_db_with_tenant(dir: &tempfile::TempDir, tenant: &str) -> IronVaultDb {
    let path = dir.path().join("test.db").to_str().unwrap().to_string();
    IronVaultDb::open(path, test_key(), tenant.into(), VaultConfig::test_config())
        .expect("failed to open test database")
}

/// Create the standard `users` table used across query builder tests.
pub fn create_users_table(db: &IronVaultDb) {
    db.execute_raw(
        "CREATE TABLE users (\
            id TEXT PRIMARY KEY, \
            name TEXT NOT NULL, \
            email TEXT, \
            role TEXT DEFAULT 'member', \
            status TEXT DEFAULT 'active', \
            score REAL, \
            tenant_id TEXT NOT NULL, \
            created_at INTEGER NOT NULL, \
            updated_at INTEGER NOT NULL, \
            deleted_at INTEGER\
        )"
        .into(),
        vec![],
    )
    .unwrap();
}

/// Create the standard `orders` table with FK to users.
pub fn create_orders_table(db: &IronVaultDb) {
    db.execute_raw(
        "CREATE TABLE orders (\
            id TEXT PRIMARY KEY, \
            user_id TEXT NOT NULL, \
            amount REAL NOT NULL, \
            status TEXT DEFAULT 'pending', \
            category TEXT, \
            tenant_id TEXT NOT NULL, \
            created_at INTEGER NOT NULL, \
            updated_at INTEGER NOT NULL, \
            deleted_at INTEGER, \
            FOREIGN KEY (user_id) REFERENCES users(id)\
        )"
        .into(),
        vec![],
    )
    .unwrap();
}

/// Default empty QuerySpec for a table.
pub fn query(table: &str) -> QuerySpec {
    QuerySpec {
        table: table.into(),
        conditions: vec![],
        or_conditions: vec![],
        order_by: vec![],
        limit: None,
        offset: None,
        joins: vec![],
        columns: vec![],
        include_deleted: false,
    }
}

/// Insert a user row using the query builder, returning the id.
pub fn insert_user(db: &IronVaultDb, name: &str, email: &str, role: &str, score: f64) -> String {
    let mut data = std::collections::HashMap::new();
    data.insert("name".into(), SqlValue::Text(name.into()));
    data.insert("email".into(), SqlValue::Text(email.into()));
    data.insert("role".into(), SqlValue::Text(role.into()));
    data.insert("score".into(), SqlValue::Real(score));
    data.insert("status".into(), SqlValue::Text("active".into()));
    db.query_insert("users".into(), data).unwrap()
}
