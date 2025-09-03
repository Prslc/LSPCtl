use crate::constants::MODULE_DB;
use rusqlite::{Connection, Result};
use prettytable::{Cell, Row, Table};

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

pub fn print_query_results(sql_command: &str) -> Result<()> {
    let conn = Connection::open(MODULE_DB)?;
    let mut stmt = conn.prepare(sql_command)?;
    let column_count = stmt.column_count();
    let column_names = stmt.column_names();

    let mut table = Table::new();

    let header: Vec<Cell> = column_names.iter().map(|name| Cell::new(name)).collect();
    table.add_row(Row::new(header));

    let rows = stmt.query_map([], |row| {
        let cells: Vec<Cell> = (0..column_count)
            .map(|i| {
                let value = match row.get_ref(i)? {
                    rusqlite::types::ValueRef::Null => "".to_string(),
                    rusqlite::types::ValueRef::Integer(i) => i.to_string(),
                    rusqlite::types::ValueRef::Real(f) => f.to_string(),
                    rusqlite::types::ValueRef::Text(t) => String::from_utf8_lossy(t).to_string(),
                    rusqlite::types::ValueRef::Blob(b) => format!("{:?}", b),
                };
                Ok(Cell::new(&value))
            })
            .collect::<Result<Vec<Cell>>>()?;
        Ok(Row::new(cells))
    })?;

    for row in rows {
        table.add_row(row?);
    }

    table.printstd();

    Ok(())
}