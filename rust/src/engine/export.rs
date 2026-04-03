use crate::api::types::{ExportFormat, SqlValue};
use crate::engine::{convert, query_builder};
use anyhow::{Context, Result};
use r2d2::PooledConnection;
use r2d2_sqlite::SqliteConnectionManager;

/// Export a table's data to the specified format.
///
/// Respects tenant isolation and soft-delete guard.
/// Returns the formatted data as bytes.
pub(crate) fn export_table(
    conn: &PooledConnection<SqliteConnectionManager>,
    table: &str,
    tenant_id: &str,
    format: &ExportFormat,
    columns: &Option<Vec<String>>,
) -> Result<Vec<u8>> {
    let spec = crate::api::types::QuerySpec {
        table: table.into(),
        conditions: vec![],
        or_conditions: vec![],
        order_by: vec![],
        limit: None,
        offset: None,
        joins: vec![],
        columns: columns.clone().unwrap_or_default(),
        include_deleted: false,
    };

    let (sql, params) = query_builder::build_select(&spec, tenant_id)?;
    let mut stmt = conn
        .prepare(&sql)
        .with_context(|| format!("ExportException: failed to prepare query for {}", table))?;

    let col_names: Vec<String> = stmt.column_names().iter().map(|c| c.to_string()).collect();
    let col_count = col_names.len();

    let rows = stmt.query_map(rusqlite::params_from_iter(params), |row| {
        let mut values = Vec::with_capacity(col_count);
        for i in 0..col_count {
            let v: rusqlite::types::Value = row.get(i)?;
            values.push(convert::from_rusqlite(v));
        }
        Ok(values)
    })?;

    let mut all_rows = Vec::new();
    for row in rows {
        all_rows.push(row?);
    }

    match format {
        ExportFormat::Csv => export_csv(&col_names, &all_rows),
        ExportFormat::Json => export_json(&col_names, &all_rows),
        ExportFormat::Jsonl => export_jsonl(&col_names, &all_rows),
    }
}

fn export_csv(columns: &[String], rows: &[Vec<SqlValue>]) -> Result<Vec<u8>> {
    let mut output = String::new();

    // Header
    output.push_str(&columns.join(","));
    output.push('\n');

    // Rows
    for row in rows {
        let cells: Vec<String> = row
            .iter()
            .map(|v| match v {
                SqlValue::Null => "".to_string(),
                SqlValue::Integer(i) => i.to_string(),
                SqlValue::Real(f) => f.to_string(),
                SqlValue::Text(s) => csv_escape(s),
                SqlValue::Blob(b) => format!(
                    "base64:{}",
                    base64::Engine::encode(&base64::engine::general_purpose::STANDARD, b)
                ),
            })
            .collect();
        output.push_str(&cells.join(","));
        output.push('\n');
    }

    Ok(output.into_bytes())
}

fn export_json(columns: &[String], rows: &[Vec<SqlValue>]) -> Result<Vec<u8>> {
    let mut output = String::from("[");
    for (ri, row) in rows.iter().enumerate() {
        if ri > 0 {
            output.push(',');
        }
        output.push('{');
        for (ci, val) in row.iter().enumerate() {
            if ci > 0 {
                output.push(',');
            }
            output.push('"');
            output.push_str(&columns[ci]);
            output.push_str("\":");
            output.push_str(&sql_value_to_json(val));
        }
        output.push('}');
    }
    output.push(']');
    Ok(output.into_bytes())
}

fn export_jsonl(columns: &[String], rows: &[Vec<SqlValue>]) -> Result<Vec<u8>> {
    let mut output = String::new();
    for row in rows {
        output.push('{');
        for (ci, val) in row.iter().enumerate() {
            if ci > 0 {
                output.push(',');
            }
            output.push('"');
            output.push_str(&columns[ci]);
            output.push_str("\":");
            output.push_str(&sql_value_to_json(val));
        }
        output.push_str("}\n");
    }
    Ok(output.into_bytes())
}

fn sql_value_to_json(val: &SqlValue) -> String {
    match val {
        SqlValue::Null => "null".to_string(),
        SqlValue::Integer(i) => i.to_string(),
        SqlValue::Real(f) => f.to_string(),
        SqlValue::Text(s) => format!("\"{}\"", json_escape(s)),
        SqlValue::Blob(b) => format!(
            "\"base64:{}\"",
            base64::Engine::encode(&base64::engine::general_purpose::STANDARD, b)
        ),
    }
}

fn csv_escape(s: &str) -> String {
    if s.contains(',') || s.contains('"') || s.contains('\n') || s.contains('\r') {
        format!("\"{}\"", s.replace('"', "\"\""))
    } else {
        s.to_string()
    }
}

fn json_escape(s: &str) -> String {
    s.replace('\\', "\\\\")
        .replace('"', "\\\"")
        .replace('\n', "\\n")
        .replace('\r', "\\r")
        .replace('\t', "\\t")
}
