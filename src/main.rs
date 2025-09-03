mod constants;
mod module;
mod utils;

use module::{select_module, switch_module};

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
                print_module_usage();
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
                        print_module_usage();
                    }
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
    eprintln!();
    print_module_usage();
}

fn print_module_usage() {
    eprintln!("Module subcommands:");
    eprintln!("  list");
    eprintln!("  enable <module_name>");
    eprintln!("  disable <module_name>");
}
