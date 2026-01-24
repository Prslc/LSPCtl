mod constants;
mod module;
mod utils;

use clap::{Parser, Subcommand};
use module::{select_module, switch_module};
use utils::print_query_results;

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

   match cli.command {
        Command::ModuleList => select_module()?,
        Command::ModuleEnable { name } => switch_module(&name, 1)?,
        Command::ModuleDisable { name } => switch_module(&name, 0)?,
        Command::DebugSql { sql } => print_query_results(&sql)?,
    }

    Ok(())
}