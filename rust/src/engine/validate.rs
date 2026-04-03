use anyhow::{anyhow, Result};

/// Validate a SQL table name against injection.
///
/// Allows: `[a-zA-Z][a-zA-Z0-9_]*`
/// Blocks: spaces, quotes, semicolons, parentheses, etc.
pub(crate) fn table_name(name: &str) -> Result<()> {
    if name.is_empty() {
        return Err(anyhow!("Table name cannot be empty"));
    }
    if !name.starts_with(|c: char| c.is_ascii_alphabetic() || c == '_') {
        return Err(anyhow!(
            "Table name must start with a letter or underscore: '{}'",
            name
        ));
    }
    if !name.chars().all(|c| c.is_ascii_alphanumeric() || c == '_') {
        return Err(anyhow!(
            "Table name contains invalid characters (only [a-zA-Z0-9_] allowed): '{}'",
            name
        ));
    }
    Ok(())
}

/// Validate a SQL column name against injection.
///
/// Allows: `[a-zA-Z_][a-zA-Z0-9_]*` with optional table prefix (`table.column`)
/// and wildcard (`*`, `table.*`).
pub(crate) fn column_name(name: &str) -> Result<()> {
    if name.is_empty() {
        return Err(anyhow!("Column name cannot be empty"));
    }

    // Allow bare "*"
    if name == "*" {
        return Ok(());
    }

    // Split on '.' for table-qualified columns (e.g., "users.name", "users.*")
    let parts: Vec<&str> = name.split('.').collect();
    match parts.len() {
        1 => validate_identifier_part(parts[0], "Column"),
        2 => {
            validate_identifier_part(parts[0], "Table qualifier")?;
            if parts[1] == "*" {
                Ok(())
            } else {
                validate_identifier_part(parts[1], "Column")
            }
        }
        _ => Err(anyhow!(
            "Column name has too many dot-separated parts: '{}'",
            name
        )),
    }
}

fn validate_identifier_part(part: &str, label: &str) -> Result<()> {
    if part.is_empty() {
        return Err(anyhow!("{} part cannot be empty", label));
    }
    if !part.starts_with(|c: char| c.is_ascii_alphabetic() || c == '_') {
        return Err(anyhow!(
            "{} must start with a letter or underscore: '{}'",
            label,
            part
        ));
    }
    if !part.chars().all(|c| c.is_ascii_alphanumeric() || c == '_') {
        return Err(anyhow!("{} contains invalid characters: '{}'", label, part));
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn valid_table_names() {
        assert!(table_name("users").is_ok());
        assert!(table_name("_migrations").is_ok());
        assert!(table_name("order_items").is_ok());
        assert!(table_name("T1").is_ok());
    }

    #[test]
    fn invalid_table_names() {
        assert!(table_name("").is_err());
        assert!(table_name("1bad").is_err());
        assert!(table_name("users; DROP TABLE").is_err());
        assert!(table_name("user's").is_err());
        assert!(table_name("users--").is_err());
        assert!(table_name("table name").is_err());
        assert!(table_name("users.stuff").is_err());
    }

    #[test]
    fn valid_column_names() {
        assert!(column_name("name").is_ok());
        assert!(column_name("created_at").is_ok());
        assert!(column_name("*").is_ok());
        assert!(column_name("users.name").is_ok());
        assert!(column_name("users.*").is_ok());
        assert!(column_name("_id").is_ok());
    }

    #[test]
    fn invalid_column_names() {
        assert!(column_name("").is_err());
        assert!(column_name("1col").is_err());
        assert!(column_name("col; DROP").is_err());
        assert!(column_name("a.b.c").is_err());
        assert!(column_name("col'umn").is_err());
        assert!(column_name("col name").is_err());
    }
}
