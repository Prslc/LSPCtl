use rusqlite::{Connection, Result, params};

#[derive(Debug, Clone, Copy)]
pub enum ModuleStatus {
    Enabled,
    Disabled,
    Unknown,
}

impl ModuleStatus {
    pub fn from_i32(value: i32) -> Self {
        match value {
            0 => ModuleStatus::Disabled,
            1 => ModuleStatus::Enabled,
            _ => ModuleStatus::Unknown,
        }
    }

    pub fn as_str(&self) -> &'static str {
        match self {
            ModuleStatus::Disabled => "Disabled ❌",
            ModuleStatus::Enabled => "Enabled  ✅",
            ModuleStatus::Unknown => "Unknown  ⚠️",
        }
    }
}

// select all modules
pub fn list_modules(conn: &Connection) -> Result<()> {
    let mut stmt = conn.prepare(
        "SELECT module_pkg_name, enabled
        FROM modules
        WHERE module_pkg_name != 'lspd'
        ORDER BY enabled DESC, module_pkg_name ASC",
    )?;

    let module_iter = stmt.query_map([], |row| {
        let name: String = row.get(0)?;
        let status = ModuleStatus::from_i32(row.get(1)?);
        Ok((name, status))
    })?;

    // collect data to compute column widths
    let mut rows = vec![];
    let mut name_width = "Module".len();
    let mut status_width = "Status".len();

    for module in module_iter {
        let (name, status) = module?;
        name_width = name_width.max(name.len());
        status_width = status_width.max(status.as_str().len());
        rows.push((name, status));
    }

    // header
    println!(
        "{:<name_width$}  {:<status_width$}",
        "Module",
        "Status",
        name_width = name_width,
        status_width = status_width
    );
    println!(
        "{:-<name_width$}  {:-<status_width$}",
        "",
        "",
        name_width = name_width,
        status_width = status_width
    );

    // rows
    for (name, status) in rows {
        println!(
            "{:<name_width$}  {:<status_width$}",
            name,
            status.as_str(),
            name_width = name_width,
            status_width = status_width
        );
    }

    Ok(())
}

pub fn switch_module(conn: &Connection, pack_name: &str, status: i32) -> Result<()> {
    let affected = conn.execute(
        "UPDATE modules SET enabled = ?1 WHERE module_pkg_name = ?2",
        params![status, pack_name],
    )?;

    let status_enum = ModuleStatus::from_i32(status);

    if affected == 0 {
        eprintln!("Warning: module '{}' not found", pack_name);
    } else {
        println!("{} {}", pack_name, status_enum.as_str());
    }

    Ok(())
}
