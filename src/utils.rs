use rusqlite::{Connection};

/// Executes a SQL query and prints all rows of the result.
///
/// # Arguments
///
/// * `conn` - A reference to an open SQLite `Connection`.
/// * `sql_command` - A SQL command to execute (e.g., `SELECT * FROM modules`).
///
/// # Returns
///
/// Returns a `Result<()>`, propagating any SQLite errors encountered.
pub fn print_query_results(conn: &Connection, sql_command: &str) -> rusqlite::Result<()> {
    let mut stmt = conn.prepare(sql_command)?;
    let column_count = stmt.column_count();
    let mut idx: i64 = 1;

    let rows = stmt.query_map([], |row| {
        for i in 0..column_count {
            let value: String = match row.get_ref(i)? {
                rusqlite::types::ValueRef::Null => "".to_string(),
                rusqlite::types::ValueRef::Integer(i) => i.to_string(),
                rusqlite::types::ValueRef::Real(f) => f.to_string(),
                rusqlite::types::ValueRef::Text(t) => String::from_utf8_lossy(t).to_string(),
                rusqlite::types::ValueRef::Blob(b) => format!("{:?}", b),
            };
            print!("[{}] {} ", idx, value);
            idx = idx+1;
        }
        println!();
        Ok(())
    })?;

    for row in rows {
        row?;
    }

    Ok(())
}