use crate::api::types::SqlValue;
use rusqlite::types::Value;

/// Convert our `SqlValue` to rusqlite's `Value` for parameterized queries.
pub(crate) fn to_rusqlite(val: &SqlValue) -> Value {
    match val {
        SqlValue::Null => Value::Null,
        SqlValue::Integer(i) => Value::Integer(*i),
        SqlValue::Real(f) => Value::Real(*f),
        SqlValue::Text(s) => Value::Text(s.clone()),
        SqlValue::Blob(b) => Value::Blob(b.clone()),
    }
}

/// Convert rusqlite's `Value` back to our `SqlValue`.
pub(crate) fn from_rusqlite(val: Value) -> SqlValue {
    match val {
        Value::Null => SqlValue::Null,
        Value::Integer(i) => SqlValue::Integer(i),
        Value::Real(f) => SqlValue::Real(f),
        Value::Text(s) => SqlValue::Text(s),
        Value::Blob(b) => SqlValue::Blob(b),
    }
}
