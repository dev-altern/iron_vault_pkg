use crate::api::types::*;
use crate::engine::{convert, validate};
use anyhow::Result;
use rusqlite::types::Value;

/// Build a SELECT query from a QuerySpec, injecting tenant_id and soft-delete guard.
///
/// Returns `(sql, params)` ready for `conn.prepare(&sql)` + `params_from_iter(params)`.
pub(crate) fn build_select(spec: &QuerySpec, tenant_id: &str) -> Result<(String, Vec<Value>)> {
    validate::table_name(&spec.table)?;
    let mut sql = String::with_capacity(256);
    let mut params: Vec<Value> = Vec::new();

    // ── SELECT ──
    if spec.columns.is_empty() {
        sql.push_str("SELECT *");
    } else {
        sql.push_str("SELECT ");
        for (i, col) in spec.columns.iter().enumerate() {
            validate::column_name(col)?;
            if i > 0 {
                sql.push_str(", ");
            }
            sql.push_str(col);
        }
    }

    // ── FROM ──
    sql.push_str(" FROM ");
    sql.push_str(&spec.table);

    // ── JOINs ──
    build_joins(&spec.joins, &mut sql)?;

    // ── WHERE ──
    sql.push_str(" WHERE ");
    build_system_conditions(
        &spec.table,
        tenant_id,
        spec.include_deleted,
        !spec.joins.is_empty(),
        &mut sql,
        &mut params,
    );
    build_user_conditions(&spec.conditions, &spec.or_conditions, &mut sql, &mut params)?;

    // ── ORDER BY ──
    build_order_by(&spec.order_by, &mut sql)?;

    // ── LIMIT / OFFSET ──
    if let Some(limit) = spec.limit {
        sql.push_str(&format!(" LIMIT {}", limit));
    }
    if let Some(offset) = spec.offset {
        sql.push_str(&format!(" OFFSET {}", offset));
    }

    Ok((sql, params))
}

/// Build a SELECT COUNT(*) query from a QuerySpec.
pub(crate) fn build_count(spec: &QuerySpec, tenant_id: &str) -> Result<(String, Vec<Value>)> {
    validate::table_name(&spec.table)?;
    let mut sql = String::with_capacity(256);
    let mut params: Vec<Value> = Vec::new();

    sql.push_str("SELECT COUNT(*) AS _count FROM ");
    sql.push_str(&spec.table);

    build_joins(&spec.joins, &mut sql)?;

    sql.push_str(" WHERE ");
    build_system_conditions(
        &spec.table,
        tenant_id,
        spec.include_deleted,
        !spec.joins.is_empty(),
        &mut sql,
        &mut params,
    );
    build_user_conditions(&spec.conditions, &spec.or_conditions, &mut sql, &mut params)?;

    Ok((sql, params))
}

/// Build a SELECT with aggregate expressions.
pub(crate) fn build_aggregate(
    spec: &QuerySpec,
    expressions: &[AggExpr],
    tenant_id: &str,
) -> Result<(String, Vec<Value>)> {
    validate::table_name(&spec.table)?;
    if expressions.is_empty() {
        return Err(anyhow::anyhow!(
            "At least one aggregate expression is required"
        ));
    }

    let mut sql = String::with_capacity(256);
    let mut params: Vec<Value> = Vec::new();

    sql.push_str("SELECT ");
    for (i, expr) in expressions.iter().enumerate() {
        if i > 0 {
            sql.push_str(", ");
        }
        let (func, col, alias) = match expr {
            AggExpr::Count { column, alias } => ("COUNT", column.as_str(), alias.as_str()),
            AggExpr::Sum { column, alias } => ("SUM", column.as_str(), alias.as_str()),
            AggExpr::Avg { column, alias } => ("AVG", column.as_str(), alias.as_str()),
            AggExpr::Min { column, alias } => ("MIN", column.as_str(), alias.as_str()),
            AggExpr::Max { column, alias } => ("MAX", column.as_str(), alias.as_str()),
        };
        validate::column_name(col)?;
        validate::column_name(alias)?;
        sql.push_str(&format!("{}({}) AS {}", func, col, alias));
    }

    sql.push_str(" FROM ");
    sql.push_str(&spec.table);

    build_joins(&spec.joins, &mut sql)?;

    sql.push_str(" WHERE ");
    build_system_conditions(
        &spec.table,
        tenant_id,
        spec.include_deleted,
        !spec.joins.is_empty(),
        &mut sql,
        &mut params,
    );
    build_user_conditions(&spec.conditions, &spec.or_conditions, &mut sql, &mut params)?;

    Ok((sql, params))
}

// ─── Private Helpers ─────────────────────────────────────────────────

/// Append tenant_id and deleted_at conditions (always top-level AND).
///
/// When JOINs are present, qualifies with the main table name to avoid
/// ambiguous column errors (both tables may have tenant_id / deleted_at).
fn build_system_conditions(
    table: &str,
    tenant_id: &str,
    include_deleted: bool,
    has_joins: bool,
    sql: &mut String,
    params: &mut Vec<Value>,
) {
    if has_joins {
        sql.push_str(&format!("{}.tenant_id = ?", table));
    } else {
        sql.push_str("tenant_id = ?");
    }
    params.push(Value::Text(tenant_id.to_string()));

    if !include_deleted {
        if has_joins {
            sql.push_str(&format!(" AND {}.deleted_at IS NULL", table));
        } else {
            sql.push_str(" AND deleted_at IS NULL");
        }
    }
}

/// Append user WHERE conditions, properly wrapping OR groups.
///
/// If there are OR groups, generates:
///   AND ((main_conds) OR (group1_conds) OR (group2_conds))
/// If no OR groups, generates:
///   AND cond1 AND cond2
fn build_user_conditions(
    conditions: &[Condition],
    or_conditions: &[Vec<Condition>],
    sql: &mut String,
    params: &mut Vec<Value>,
) -> Result<()> {
    if conditions.is_empty() && or_conditions.is_empty() {
        return Ok(());
    }

    if or_conditions.is_empty() {
        // Simple case: just AND conditions
        for cond in conditions {
            sql.push_str(" AND ");
            build_single_condition(cond, sql, params)?;
        }
    } else {
        // OR groups present — wrap everything safely
        sql.push_str(" AND (");

        // Main AND group
        if !conditions.is_empty() {
            sql.push('(');
            for (i, cond) in conditions.iter().enumerate() {
                if i > 0 {
                    sql.push_str(" AND ");
                }
                build_single_condition(cond, sql, params)?;
            }
            sql.push(')');
        }

        // OR groups
        for (gi, group) in or_conditions.iter().enumerate() {
            if !conditions.is_empty() || gi > 0 {
                sql.push_str(" OR ");
            }
            sql.push('(');
            for (i, cond) in group.iter().enumerate() {
                if i > 0 {
                    sql.push_str(" AND ");
                }
                build_single_condition(cond, sql, params)?;
            }
            sql.push(')');
        }

        sql.push(')');
    }

    Ok(())
}

/// Build SQL for a single Condition, pushing values to params.
fn build_single_condition(
    cond: &Condition,
    sql: &mut String,
    params: &mut Vec<Value>,
) -> Result<()> {
    match cond {
        Condition::Eq { column, value } => {
            validate::column_name(column)?;
            sql.push_str(column);
            sql.push_str(" = ?");
            params.push(convert::to_rusqlite(value));
        }
        Condition::NotEq { column, value } => {
            validate::column_name(column)?;
            sql.push_str(column);
            sql.push_str(" != ?");
            params.push(convert::to_rusqlite(value));
        }
        Condition::Gt { column, value } => {
            validate::column_name(column)?;
            sql.push_str(column);
            sql.push_str(" > ?");
            params.push(convert::to_rusqlite(value));
        }
        Condition::Gte { column, value } => {
            validate::column_name(column)?;
            sql.push_str(column);
            sql.push_str(" >= ?");
            params.push(convert::to_rusqlite(value));
        }
        Condition::Lt { column, value } => {
            validate::column_name(column)?;
            sql.push_str(column);
            sql.push_str(" < ?");
            params.push(convert::to_rusqlite(value));
        }
        Condition::Lte { column, value } => {
            validate::column_name(column)?;
            sql.push_str(column);
            sql.push_str(" <= ?");
            params.push(convert::to_rusqlite(value));
        }
        Condition::Like { column, pattern } => {
            validate::column_name(column)?;
            sql.push_str(column);
            sql.push_str(" LIKE ?");
            params.push(Value::Text(pattern.clone()));
        }
        Condition::Between { column, low, high } => {
            validate::column_name(column)?;
            sql.push_str(column);
            sql.push_str(" BETWEEN ? AND ?");
            params.push(convert::to_rusqlite(low));
            params.push(convert::to_rusqlite(high));
        }
        Condition::In { column, values } => {
            validate::column_name(column)?;
            if values.is_empty() {
                // IN () is invalid SQL — use always-false condition
                sql.push_str("0 = 1");
            } else {
                sql.push_str(column);
                sql.push_str(" IN (");
                for (i, val) in values.iter().enumerate() {
                    if i > 0 {
                        sql.push_str(", ");
                    }
                    sql.push('?');
                    params.push(convert::to_rusqlite(val));
                }
                sql.push(')');
            }
        }
        Condition::NotIn { column, values } => {
            validate::column_name(column)?;
            if values.is_empty() {
                // NOT IN () is always true — skip condition
                sql.push_str("1 = 1");
            } else {
                sql.push_str(column);
                sql.push_str(" NOT IN (");
                for (i, val) in values.iter().enumerate() {
                    if i > 0 {
                        sql.push_str(", ");
                    }
                    sql.push('?');
                    params.push(convert::to_rusqlite(val));
                }
                sql.push(')');
            }
        }
        Condition::IsNull { column } => {
            validate::column_name(column)?;
            sql.push_str(column);
            sql.push_str(" IS NULL");
        }
        Condition::IsNotNull { column } => {
            validate::column_name(column)?;
            sql.push_str(column);
            sql.push_str(" IS NOT NULL");
        }
        Condition::Raw {
            sql: raw_sql,
            params: raw_params,
        } => {
            sql.push_str(raw_sql);
            for p in raw_params {
                params.push(convert::to_rusqlite(p));
            }
        }
    }
    Ok(())
}

/// Append JOIN clauses.
fn build_joins(joins: &[JoinSpec], sql: &mut String) -> Result<()> {
    for join in joins {
        match join {
            JoinSpec::Inner { table, on } => {
                validate::table_name(table)?;
                sql.push_str(" INNER JOIN ");
                sql.push_str(table);
                sql.push_str(" ON ");
                sql.push_str(on);
            }
            JoinSpec::Left { table, on } => {
                validate::table_name(table)?;
                sql.push_str(" LEFT JOIN ");
                sql.push_str(table);
                sql.push_str(" ON ");
                sql.push_str(on);
            }
            JoinSpec::Raw { expression } => {
                sql.push(' ');
                sql.push_str(expression);
            }
        }
    }
    Ok(())
}

/// Append ORDER BY clauses.
fn build_order_by(order_by: &[OrderBy], sql: &mut String) -> Result<()> {
    if order_by.is_empty() {
        return Ok(());
    }
    sql.push_str(" ORDER BY ");
    for (i, clause) in order_by.iter().enumerate() {
        if i > 0 {
            sql.push_str(", ");
        }
        match clause {
            OrderBy::Asc { column } => {
                validate::column_name(column)?;
                sql.push_str(column);
                sql.push_str(" ASC");
            }
            OrderBy::Desc { column } => {
                validate::column_name(column)?;
                sql.push_str(column);
                sql.push_str(" DESC");
            }
            OrderBy::Raw { expression } => {
                sql.push_str(expression);
            }
        }
    }
    Ok(())
}

// ─── Unit Tests ──────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;

    fn empty_spec(table: &str) -> QuerySpec {
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

    #[test]
    fn select_star_with_tenant_and_softdelete() {
        let (sql, params) = build_select(&empty_spec("users"), "t1").unwrap();
        assert_eq!(
            sql,
            "SELECT * FROM users WHERE tenant_id = ? AND deleted_at IS NULL"
        );
        assert_eq!(params.len(), 1);
        assert!(matches!(&params[0], Value::Text(t) if t == "t1"));
    }

    #[test]
    fn select_include_deleted_skips_guard() {
        let mut spec = empty_spec("users");
        spec.include_deleted = true;
        let (sql, _) = build_select(&spec, "t1").unwrap();
        assert!(sql.contains("tenant_id = ?"));
        assert!(!sql.contains("deleted_at"));
    }

    #[test]
    fn select_specific_columns() {
        let mut spec = empty_spec("users");
        spec.columns = vec!["name".into(), "email".into()];
        let (sql, _) = build_select(&spec, "t1").unwrap();
        assert!(sql.starts_with("SELECT name, email FROM users"));
    }

    #[test]
    fn select_with_eq_condition() {
        let mut spec = empty_spec("users");
        spec.conditions.push(Condition::Eq {
            column: "status".into(),
            value: SqlValue::Text("active".into()),
        });
        let (sql, params) = build_select(&spec, "t1").unwrap();
        assert!(sql.contains("AND status = ?"));
        assert_eq!(params.len(), 2);
    }

    #[test]
    fn select_with_in_condition() {
        let mut spec = empty_spec("users");
        spec.conditions.push(Condition::In {
            column: "role".into(),
            values: vec![SqlValue::Text("admin".into()), SqlValue::Text("mod".into())],
        });
        let (sql, params) = build_select(&spec, "t1").unwrap();
        assert!(sql.contains("role IN (?, ?)"));
        assert_eq!(params.len(), 3);
    }

    #[test]
    fn select_with_empty_in_generates_false() {
        let mut spec = empty_spec("users");
        spec.conditions.push(Condition::In {
            column: "role".into(),
            values: vec![],
        });
        let (sql, _) = build_select(&spec, "t1").unwrap();
        assert!(sql.contains("0 = 1"));
    }

    #[test]
    fn select_with_between() {
        let mut spec = empty_spec("orders");
        spec.conditions.push(Condition::Between {
            column: "amount".into(),
            low: SqlValue::Real(10.0),
            high: SqlValue::Real(100.0),
        });
        let (sql, params) = build_select(&spec, "t1").unwrap();
        assert!(sql.contains("amount BETWEEN ? AND ?"));
        assert_eq!(params.len(), 3);
    }

    #[test]
    fn select_or_conditions_wrapped_safely() {
        let mut spec = empty_spec("users");
        spec.conditions.push(Condition::Eq {
            column: "role".into(),
            value: SqlValue::Text("admin".into()),
        });
        spec.or_conditions.push(vec![Condition::Eq {
            column: "score".into(),
            value: SqlValue::Integer(100),
        }]);
        let (sql, _) = build_select(&spec, "t1").unwrap();
        assert!(
            sql.contains("AND ((role = ?) OR (score = ?))"),
            "SQL: {}",
            sql
        );
    }

    #[test]
    fn select_or_only_no_main_conditions() {
        let mut spec = empty_spec("users");
        spec.or_conditions.push(vec![Condition::Eq {
            column: "a".into(),
            value: SqlValue::Integer(1),
        }]);
        spec.or_conditions.push(vec![Condition::Eq {
            column: "b".into(),
            value: SqlValue::Integer(2),
        }]);
        let (sql, _) = build_select(&spec, "t1").unwrap();
        assert!(sql.contains("AND ((a = ?) OR (b = ?))"), "SQL: {}", sql);
    }

    #[test]
    fn select_order_by() {
        let mut spec = empty_spec("users");
        spec.order_by.push(OrderBy::Desc {
            column: "created_at".into(),
        });
        spec.order_by.push(OrderBy::Asc {
            column: "name".into(),
        });
        let (sql, _) = build_select(&spec, "t1").unwrap();
        assert!(sql.contains("ORDER BY created_at DESC, name ASC"));
    }

    #[test]
    fn select_limit_offset() {
        let mut spec = empty_spec("users");
        spec.limit = Some(20);
        spec.offset = Some(40);
        let (sql, _) = build_select(&spec, "t1").unwrap();
        assert!(sql.contains("LIMIT 20"));
        assert!(sql.contains("OFFSET 40"));
    }

    #[test]
    fn select_join_qualifies_system_conditions() {
        let mut spec = empty_spec("orders");
        spec.joins.push(JoinSpec::Inner {
            table: "users".into(),
            on: "orders.user_id = users.id".into(),
        });
        let (sql, _) = build_select(&spec, "t1").unwrap();
        assert!(sql.contains("orders.tenant_id = ?"));
        assert!(sql.contains("orders.deleted_at IS NULL"));
        assert!(sql.contains("INNER JOIN users ON orders.user_id = users.id"));
    }

    #[test]
    fn select_left_join() {
        let mut spec = empty_spec("users");
        spec.joins.push(JoinSpec::Left {
            table: "orders".into(),
            on: "users.id = orders.user_id".into(),
        });
        let (sql, _) = build_select(&spec, "t1").unwrap();
        assert!(sql.contains("LEFT JOIN orders ON users.id = orders.user_id"));
    }

    #[test]
    fn count_basic() {
        let (sql, params) = build_count(&empty_spec("users"), "t1").unwrap();
        assert!(sql.starts_with("SELECT COUNT(*) AS _count FROM users"));
        assert!(sql.contains("tenant_id = ?"));
        assert_eq!(params.len(), 1);
    }

    #[test]
    fn count_with_conditions() {
        let mut spec = empty_spec("users");
        spec.conditions.push(Condition::Eq {
            column: "role".into(),
            value: SqlValue::Text("admin".into()),
        });
        let (sql, params) = build_count(&spec, "t1").unwrap();
        assert!(sql.contains("AND role = ?"));
        assert_eq!(params.len(), 2);
    }

    #[test]
    fn aggregate_generates_correct_sql() {
        let exprs = vec![
            AggExpr::Count {
                column: "*".into(),
                alias: "cnt".into(),
            },
            AggExpr::Sum {
                column: "amount".into(),
                alias: "total".into(),
            },
        ];
        let (sql, _) = build_aggregate(&empty_spec("orders"), &exprs, "t1").unwrap();
        assert!(sql.contains("COUNT(*) AS cnt"));
        assert!(sql.contains("SUM(amount) AS total"));
    }

    #[test]
    fn aggregate_empty_expressions_errors() {
        assert!(build_aggregate(&empty_spec("orders"), &[], "t1").is_err());
    }

    #[test]
    fn invalid_table_name_rejected() {
        assert!(build_select(&empty_spec("users; DROP TABLE"), "t1").is_err());
    }

    #[test]
    fn invalid_column_in_condition_rejected() {
        let mut spec = empty_spec("users");
        spec.conditions.push(Condition::Eq {
            column: "col; DROP".into(),
            value: SqlValue::Integer(1),
        });
        assert!(build_select(&spec, "t1").is_err());
    }

    #[test]
    fn invalid_column_in_order_rejected() {
        let mut spec = empty_spec("users");
        spec.order_by.push(OrderBy::Asc {
            column: "bad col".into(),
        });
        assert!(build_select(&spec, "t1").is_err());
    }

    #[test]
    fn raw_condition_passes_through() {
        let mut spec = empty_spec("users");
        spec.conditions.push(Condition::Raw {
            sql: "json_extract(data, '$.score') > ?".into(),
            params: vec![SqlValue::Real(0.8)],
        });
        let (sql, params) = build_select(&spec, "t1").unwrap();
        assert!(sql.contains("json_extract(data, '$.score') > ?"));
        assert_eq!(params.len(), 2);
    }
}
