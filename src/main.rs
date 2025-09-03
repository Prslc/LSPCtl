mod constants;
mod module;
mod utils;

use module::{select_module, switch_module};
use utils::print_query_results;

fn main() -> rusqlite::Result<()> {
    let args: Vec<String> = std::env::args().collect();

    if args.len() < 2 {
        print_usage();
        return Ok(());
    }

    match args[1].as_str() {
        "module" => {
            if args.len() < 3 {
                eprintln!("Error: Please provide a subcommand for 'module'.");
                print_sub_usage();
            } else {
                match args[2].as_str() {
                    "list" => {
                        select_module()?;
                    }
                    "enable" | "disable" => {
                        if args.len() < 4 {
                            eprintln!("Error: Please provide a module name.");
                        } else {
                            let pack_name = &args[3];
                            let status = if args[2] == "enable" { 1 } else { 0 };
                            if let Err(e) = switch_module(pack_name, status) {
                                eprintln!("Failed to {} '{}': {}", args[2], pack_name, e);
                            }
                        }
                    }
                    _ => {
                        eprintln!("Unknown subcommand for 'module': '{}'", args[2]);
                        print_sub_usage();
                    }
                }
            }
        }
        "debug" => {
            if args.len() < 3 {
                eprintln!("Error: Please provide a subcommand for 'debug'.");
                print_sub_usage();
            } else {
                match args[2].as_str() {
                    "sql" => {
                        if args.len() < 4 {
                            eprintln!("Error: SQL command is empty.");
                            print_sub_usage();
                        } else {
                            let sql_command = &args[3];
                            print_query_results(sql_command)?;
                        }
                    }
                    _ => print_sub_usage(),
                }
            }
        }
        _ => {
            eprintln!("Unknown command: '{}'", args[1]);
            print_usage();
        }
    }

    Ok(())
}

fn print_usage() {
    eprintln!("Usage:");
    eprintln!("  SLCM module <subcommand>");
    eprintln!("  SLCM debug <subcommand>");
    eprintln!();
    print_sub_usage();
}

fn print_sub_usage() {
    eprintln!("Module subcommands:");
    eprintln!("  list");
    eprintln!("  enable <module_name>");
    eprintln!("  disable <module_name>\n");
    eprintln!("Debug subcommands:");
    eprintln!("  sql <sql_command>");
}