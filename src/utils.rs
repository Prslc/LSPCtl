use crate::constants::MODULE_DB;
use rusqlite::{Connection, Result};

/// Executes a SQL query and prints all rows as a plain-text table.
///
/// # Arguments
///
/// * `sql_command` - A SQL command to execute (e.g., `SELECT * FROM modules`).
///
/// # Returns
///
/// Returns a `Result<()>`, propagating any SQLite errors encountered.
pub fn show_query_results(sql_command: &str) -> Result<()> {
    let conn = Connection::open(MODULE_DB)?;
    let mut stmt = conn.prepare(sql_command)?;
    let column_count = stmt.column_count();
    let column_names = stmt.column_names();

    // header
    println!("{}", column_names.join(" | "));
    println!(
        "{}",
        "-".repeat(column_names.iter().map(|s| s.len() + 3).sum())
    );

    let rows = stmt.query_map([], |row| {
        let values: Result<Vec<String>, rusqlite::Error> = (0..column_count)
            .map(|i| {
                let value = match row.get_ref(i)? {
                    rusqlite::types::ValueRef::Null => "".to_string(),
                    rusqlite::types::ValueRef::Integer(i) => i.to_string(),
                    rusqlite::types::ValueRef::Real(f) => f.to_string(),
                    rusqlite::types::ValueRef::Text(t) => String::from_utf8_lossy(t).to_string(),
                    rusqlite::types::ValueRef::Blob(b) => format!("{:?}", b),
                };
                Ok(value)
            })
            .collect();
        values
    })?;

    for row in rows {
        println!("{}", row?.join(" | "));
    }

    Ok(())
}
