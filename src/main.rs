mod constants;
mod module;
mod utils;

use crate::constants::MODULE_DB;
use clap::{Parser, Subcommand};
use module::{list_modules, switch_module};
use rusqlite::Connection;
use utils::show_query_results;

#[derive(Parser)]
#[command(name = "slcm", version, about = "Simple LSPosed Module Control")]
pub struct Cli {
    #[command(subcommand)]
    pub command: Command,
}

#[derive(Subcommand)]
pub enum Command {
    ModuleList,
    ModuleEnable { name: String },
    ModuleDisable { name: String },
    DebugSql { sql: String },
}

fn main() -> rusqlite::Result<()> {
    let cli = Cli::parse();
    let conn = Connection::open(MODULE_DB)?;
    match cli.command {
        Command::ModuleList => list_modules(&conn)?,
        Command::ModuleEnable { name } => switch_module(&conn, &name, 1)?,
        Command::ModuleDisable { name } => switch_module(&conn, &name, 0)?,
        Command::DebugSql { sql } => show_query_results(&sql)?,
    }

    Ok(())
}
