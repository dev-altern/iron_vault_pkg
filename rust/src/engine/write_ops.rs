use crate::api::types::SqlValue;
use crate::engine::{convert, validate};
use anyhow::{anyhow, Result};
use rusqlite::types::Value;
use std::collections::HashMap;

/// Build an INSERT statement, injecting tenant_id and auto-generating id if needed.
///
/// Returns `(sql, params, id)`.
pub(crate) fn build_insert(
    table: &str,
    mut data: HashMap<String, SqlValue>,
    tenant_id: &str,
) -> Result<(String, Vec<Value>, String)> {
    validate::table_name(table)?;

    // Auto-inject tenant_id (overwrite if user provided it)
    data.insert("tenant_id".into(), SqlValue::Text(tenant_id.to_string()));

    // Auto-generate id if not provided
    let id = match data.get("id") {
        Some(SqlValue::Text(existing)) => existing.clone(),
        _ => {
            let generated = uuid::Uuid::new_v4().to_string();
            data.insert("id".into(), SqlValue::Text(generated.clone()));
            generated
        }
    };

    // Auto-set timestamps if not provided
    let now = current_epoch_ms();
    data.entry("created_at".into())
        .or_insert(SqlValue::Integer(now));
    data.entry("updated_at".into())
        .or_insert(SqlValue::Integer(now));

    // Build SQL — collect in deterministic order
    // Sort columns for deterministic SQL (enables query plan caching)
    let mut sorted_cols: Vec<(&String, &SqlValue)> = data.iter().collect();
    sorted_cols.sort_by_key(|(k, _)| k.as_str());

    let mut columns = Vec::with_capacity(sorted_cols.len());
    let mut values = Vec::with_capacity(sorted_cols.len());
    for (col, val) in &sorted_cols {
        validate::column_name(col)?;
        columns.push(col.as_str());
        values.push(convert::to_rusqlite(val));
    }

    let placeholders: String = (0..columns.len())
        .map(|_| "?")
        .collect::<Vec<_>>()
        .join(", ");

    let sql = format!(
        "INSERT INTO {} ({}) VALUES ({})",
        table,
        columns.join(", "),
        placeholders,
    );

    Ok((sql, values, id))
}

/// Build an UPDATE statement for a single row by id.
///
/// Injects `tenant_id` and `deleted_at IS NULL` into WHERE clause.
/// Auto-sets `updated_at` if not in data.
/// Returns `(sql, params)`.
pub(crate) fn build_update(
    table: &str,
    id: &str,
    mut data: HashMap<String, SqlValue>,
    tenant_id: &str,
) -> Result<(String, Vec<Value>)> {
    validate::table_name(table)?;

    if data.is_empty() {
        return Err(anyhow!("Update data cannot be empty"));
    }

    // Prevent caller from changing tenant_id or id
    data.remove("tenant_id");
    data.remove("id");

    // Auto-set updated_at
    data.entry("updated_at".into())
        .or_insert(SqlValue::Integer(current_epoch_ms()));

    let mut sorted_cols: Vec<(&String, &SqlValue)> = data.iter().collect();
    sorted_cols.sort_by_key(|(k, _)| k.as_str());

    let mut set_parts = Vec::with_capacity(sorted_cols.len());
    let mut params = Vec::with_capacity(sorted_cols.len() + 2);
    for (col, val) in &sorted_cols {
        validate::column_name(col)?;
        set_parts.push(format!("{} = ?", col));
        params.push(convert::to_rusqlite(val));
    }

    // WHERE id = ? AND tenant_id = ? AND deleted_at IS NULL
    params.push(Value::Text(id.to_string()));
    params.push(Value::Text(tenant_id.to_string()));

    let sql = format!(
        "UPDATE {} SET {} WHERE id = ? AND tenant_id = ? AND deleted_at IS NULL",
        table,
        set_parts.join(", "),
    );

    Ok((sql, params))
}

/// Build an UPSERT (INSERT ... ON CONFLICT ... DO UPDATE).
///
/// Returns `(sql, params, id)`.
pub(crate) fn build_upsert(
    table: &str,
    mut data: HashMap<String, SqlValue>,
    conflict_column: &str,
    tenant_id: &str,
) -> Result<(String, Vec<Value>, String)> {
    validate::table_name(table)?;
    validate::column_name(conflict_column)?;

    // Auto-inject tenant_id
    data.insert("tenant_id".into(), SqlValue::Text(tenant_id.to_string()));

    // Auto-generate id if not provided
    let id = match data.get("id") {
        Some(SqlValue::Text(existing)) => existing.clone(),
        _ => {
            let generated = uuid::Uuid::new_v4().to_string();
            data.insert("id".into(), SqlValue::Text(generated.clone()));
            generated
        }
    };

    let now = current_epoch_ms();
    data.entry("created_at".into())
        .or_insert(SqlValue::Integer(now));
    data.entry("updated_at".into())
        .or_insert(SqlValue::Integer(now));

    let mut sorted_cols: Vec<(&String, &SqlValue)> = data.iter().collect();
    sorted_cols.sort_by_key(|(k, _)| k.as_str());

    let mut columns = Vec::with_capacity(sorted_cols.len());
    let mut values = Vec::with_capacity(sorted_cols.len());
    for (col, val) in &sorted_cols {
        validate::column_name(col)?;
        columns.push(col.as_str());
        values.push(convert::to_rusqlite(val));
    }

    let placeholders: String = (0..columns.len())
        .map(|_| "?")
        .collect::<Vec<_>>()
        .join(", ");

    // Build ON CONFLICT ... DO UPDATE SET for all columns except the conflict column, id, and tenant_id
    let update_parts: Vec<String> = columns
        .iter()
        .filter(|c| **c != conflict_column && **c != "id" && **c != "tenant_id")
        .map(|c| format!("{} = excluded.{}", c, c))
        .collect();

    // WHERE tenant_id = excluded.tenant_id ensures cross-tenant upserts
    // insert a new row instead of overwriting another tenant's data.
    let sql = format!(
        "INSERT INTO {} ({}) VALUES ({}) ON CONFLICT({}) DO UPDATE SET {} WHERE {}.tenant_id = excluded.tenant_id",
        table,
        columns.join(", "),
        placeholders,
        conflict_column,
        update_parts.join(", "),
        table,
    );

    Ok((sql, values, id))
}

/// Build a soft-DELETE (SET deleted_at = now).
///
/// Returns `(sql, params)`.
pub(crate) fn build_soft_delete(
    table: &str,
    id: &str,
    tenant_id: &str,
) -> Result<(String, Vec<Value>)> {
    validate::table_name(table)?;

    let now = current_epoch_ms();
    let sql = format!(
        "UPDATE {} SET deleted_at = ?, updated_at = ? \
         WHERE id = ? AND tenant_id = ? AND deleted_at IS NULL",
        table,
    );
    let params = vec![
        Value::Integer(now),
        Value::Integer(now),
        Value::Text(id.to_string()),
        Value::Text(tenant_id.to_string()),
    ];

    Ok((sql, params))
}

/// Build a hard DELETE (permanent removal).
///
/// Returns `(sql, params)`.
pub(crate) fn build_hard_delete(
    table: &str,
    id: &str,
    tenant_id: &str,
) -> Result<(String, Vec<Value>)> {
    validate::table_name(table)?;

    let sql = format!("DELETE FROM {} WHERE id = ? AND tenant_id = ?", table,);
    let params = vec![
        Value::Text(id.to_string()),
        Value::Text(tenant_id.to_string()),
    ];

    Ok((sql, params))
}

pub(crate) fn current_epoch_ms() -> i64 {
    std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap()
        .as_millis() as i64
}
