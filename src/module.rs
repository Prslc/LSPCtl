use crate::constants::MODULE_DB;

use rusqlite::{Connection, params};
use prettytable::{Table, Row, Cell, format};

// select all module
pub fn select_module() -> rusqlite::Result<()> {
    let conn = Connection::open(MODULE_DB)?;

    let mut table = Table::new();
    table.set_format(*format::consts::FORMAT_NO_LINESEP_WITH_TITLE);
    table.set_titles(Row::new(vec![Cell::new("Module"), Cell::new("Status")]));

    // 查询模块，已启用排在前面
    let mut stmt = conn.prepare(
        "SELECT module_pkg_name, enabled
        FROM modules
        WHERE module_pkg_name != 'lspd'
        ORDER BY enabled DESC, module_pkg_name ASC"
    )?;

    let module_iter = stmt.query_map([], |row| {
        let name: String = row.get(0)?;
        let enabled: i32 = row.get(1)?;
        Ok((name, enabled))
    })?;

    for module in module_iter {
        let (name, enabled) = module?;
        let status = if enabled == 1 { "Enabled  ✅" } else { "Disabled ❌" };
        table.add_row(Row::new(vec![Cell::new(&name), Cell::new(status)]));
    }

    table.printstd();

    Ok(())
}

pub fn switch_module(pack_name: &str, status: i32) -> rusqlite::Result<()> {
    let conn = Connection::open(MODULE_DB)?;
    conn.execute(
        "UPDATE modules SET enabled = ?1 WHERE module_pkg_name = ?2",
        params![status, pack_name],
    )?;
    Ok(())
}

