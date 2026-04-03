use crate::api::types::*;
use crate::engine::{convert, validate, write_ops};
use anyhow::{anyhow, Context, Result};
use r2d2::PooledConnection;
use r2d2_sqlite::SqliteConnectionManager;
use std::collections::HashSet;

/// Execute a list of operations in a single `BEGIN IMMEDIATE` transaction.
///
/// Any failure rolls back the entire transaction. Savepoints provide
/// partial rollback within the transaction.
pub(crate) fn execute_transaction(
    conn: &PooledConnection<SqliteConnectionManager>,
    ops: &[Op],
    tenant_id: &str,
) -> Result<TransactionResult> {
    let mut inserted_ids = Vec::new();
    let mut affected_tables = HashSet::new();
    let mut rows_affected: u64 = 0;

    conn.execute_batch("BEGIN IMMEDIATE")
        .context("TransactionRollbackException: failed to begin")?;

    for (i, op) in ops.iter().enumerate() {
        let result = execute_single_op(conn, op, tenant_id);
        match result {
            Ok(op_result) => {
                if let Some(id) = op_result.inserted_id {
                    inserted_ids.push(id);
                }
                if let Some(table) = op_result.table {
                    affected_tables.insert(table);
                }
                rows_affected += op_result.rows_affected;
            }
            Err(e) => {
                let _ = conn.execute_batch("ROLLBACK");
                return Err(anyhow!(
                    "TransactionRollbackException: op {} failed — rolled back all: {}",
                    i,
                    e,
                ));
            }
        }
    }

    conn.execute_batch("COMMIT")
        .context("TransactionRollbackException: failed to commit")?;

    let mut tables: Vec<String> = affected_tables.into_iter().collect();
    tables.sort();

    Ok(TransactionResult {
        inserted_ids,
        affected_tables: tables,
        rows_affected,
    })
}

/// Execute optimistic locking update.
///
/// `UPDATE table SET ..., version = version + 1 WHERE id = ? AND version = ? AND tenant_id = ? AND deleted_at IS NULL`
/// Returns error if 0 rows affected (version mismatch).
pub(crate) fn update_with_version(
    conn: &PooledConnection<SqliteConnectionManager>,
    table: &str,
    id: &str,
    expected_version: i64,
    mut data: std::collections::HashMap<String, SqlValue>,
    tenant_id: &str,
) -> Result<()> {
    validate::table_name(table)?;

    // Strip protected fields
    data.remove("tenant_id");
    data.remove("id");
    data.remove("version"); // we handle version separately

    // Auto-set updated_at
    data.entry("updated_at".into())
        .or_insert(SqlValue::Integer(write_ops::current_epoch_ms()));

    if data.is_empty() {
        return Err(anyhow!("Update data cannot be empty"));
    }

    let mut set_parts = Vec::with_capacity(data.len() + 1);
    let mut params: Vec<rusqlite::types::Value> = Vec::with_capacity(data.len() + 3);

    for (col, val) in &data {
        validate::column_name(col)?;
        set_parts.push(format!("{} = ?", col));
        params.push(convert::to_rusqlite(val));
    }

    // version = version + 1 (server-side increment, not parameterized)
    set_parts.push("version = version + 1".into());

    // WHERE id = ? AND version = ? AND tenant_id = ? AND deleted_at IS NULL
    params.push(rusqlite::types::Value::Text(id.to_string()));
    params.push(rusqlite::types::Value::Integer(expected_version));
    params.push(rusqlite::types::Value::Text(tenant_id.to_string()));

    let sql = format!(
        "UPDATE {} SET {} WHERE id = ? AND version = ? AND tenant_id = ? AND deleted_at IS NULL",
        table,
        set_parts.join(", "),
    );

    let affected = conn
        .execute(&sql, rusqlite::params_from_iter(params))
        .with_context(|| format!("OptimisticLockException: update failed on {}", table))?;

    if affected == 0 {
        return Err(anyhow!(
            "OptimisticLockException: version mismatch on {} id={} (expected version {})",
            table,
            id,
            expected_version,
        ));
    }

    Ok(())
}

// ─── Internal ────────────────────────────────────────────────────────

struct OpResult {
    inserted_id: Option<String>,
    table: Option<String>,
    rows_affected: u64,
}

fn execute_single_op(
    conn: &PooledConnection<SqliteConnectionManager>,
    op: &Op,
    tenant_id: &str,
) -> Result<OpResult> {
    match op {
        Op::Insert { table, data } => {
            let (sql, params, id) = write_ops::build_insert(table, data.clone(), tenant_id)?;
            let affected = conn
                .execute(&sql, rusqlite::params_from_iter(params))
                .with_context(|| format!("Insert into {} failed", table))?;
            Ok(OpResult {
                inserted_id: Some(id),
                table: Some(table.clone()),
                rows_affected: affected as u64,
            })
        }

        Op::Update { table, id, data } => {
            let (sql, params) = write_ops::build_update(table, id, data.clone(), tenant_id)?;
            let affected = conn
                .execute(&sql, rusqlite::params_from_iter(params))
                .with_context(|| format!("Update {} id={} failed", table, id))?;
            Ok(OpResult {
                inserted_id: None,
                table: Some(table.clone()),
                rows_affected: affected as u64,
            })
        }

        Op::Upsert {
            table,
            data,
            conflict_column,
        } => {
            let (sql, params, id) =
                write_ops::build_upsert(table, data.clone(), conflict_column, tenant_id)?;
            let affected = conn
                .execute(&sql, rusqlite::params_from_iter(params))
                .with_context(|| format!("Upsert into {} failed", table))?;
            Ok(OpResult {
                inserted_id: Some(id),
                table: Some(table.clone()),
                rows_affected: affected as u64,
            })
        }

        Op::Delete { table, id } => {
            let (sql, params) = write_ops::build_soft_delete(table, id, tenant_id)?;
            let affected = conn
                .execute(&sql, rusqlite::params_from_iter(params))
                .with_context(|| format!("Soft-delete {} id={} failed", table, id))?;
            Ok(OpResult {
                inserted_id: None,
                table: Some(table.clone()),
                rows_affected: affected as u64,
            })
        }

        Op::HardDelete { table, id } => {
            let (sql, params) = write_ops::build_hard_delete(table, id, tenant_id)?;
            let affected = conn
                .execute(&sql, rusqlite::params_from_iter(params))
                .with_context(|| format!("Hard-delete {} id={} failed", table, id))?;
            Ok(OpResult {
                inserted_id: None,
                table: Some(table.clone()),
                rows_affected: affected as u64,
            })
        }

        Op::Raw { sql, params } => {
            let values: Vec<rusqlite::types::Value> =
                params.iter().map(convert::to_rusqlite).collect();
            let affected = conn
                .execute(sql, rusqlite::params_from_iter(values))
                .with_context(|| format!("Raw SQL failed: {}", sql))?;
            Ok(OpResult {
                inserted_id: None,
                table: None,
                rows_affected: affected as u64,
            })
        }

        Op::Savepoint { name } => {
            validate::table_name(name)?; // reuse identifier validation
            conn.execute_batch(&format!("SAVEPOINT {}", name))
                .with_context(|| format!("SAVEPOINT {} failed", name))?;
            Ok(OpResult {
                inserted_id: None,
                table: None,
                rows_affected: 0,
            })
        }

        Op::ReleaseSavepoint { name } => {
            validate::table_name(name)?;
            conn.execute_batch(&format!("RELEASE SAVEPOINT {}", name))
                .with_context(|| format!("RELEASE SAVEPOINT {} failed", name))?;
            Ok(OpResult {
                inserted_id: None,
                table: None,
                rows_affected: 0,
            })
        }

        Op::RollbackToSavepoint { name } => {
            validate::table_name(name)?;
            conn.execute_batch(&format!("ROLLBACK TO SAVEPOINT {}", name))
                .with_context(|| format!("ROLLBACK TO SAVEPOINT {} failed", name))?;
            Ok(OpResult {
                inserted_id: None,
                table: None,
                rows_affected: 0,
            })
        }
    }
}
