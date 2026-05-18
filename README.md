<div align="center">

# LSPCtl

</div>

## Overview

LSPCtl is a command-line utility for managing LSPosed modules directly from a terminal or adb shell.  
It reads and writes LSPosed's `modules_config.db` database, letting you list, enable, disable, and query modules without opening the LSPosed manager UI.

## Notes

- **Requires root** on Android — the LSPosed config database lives at `/data/adb/lspd/config/modules_config.db`.
- **TWRP rescue** — since SQLite3 is bundled statically into the binary (via `rusqlite`'s `bundled` feature), LSPCtl can run in TWRP recovery to disable a misbehaving module that caused a bootloop.
- Tested on **LSPosed with SQLite3 module database**; compatibility with other Xposed frameworks is not supported.
- Use at your own risk — directly modifying the module database while LSPosed is running may have unintended side effects.

## Features

- **List modules** — display all installed LSPosed modules with their enabled/disabled status.
- **Enable / Disable modules** — toggle individual modules on or off by package name.
- **Raw SQL queries** — run arbitrary `SELECT` queries against the module database for advanced inspection.

## Usage

```
LSPCtl list                  List all modules and their status
LSPCtl enable  <package>     Enable a module by package name
LSPCtl disable <package>     Disable a module by package name
LSPCtl sql     "<query>"     Run a raw SQL query against the module database
```

## Build

```bash
# For Android (aarch64)
cargo build --release --target aarch64-linux-android

# For local testing (x86_64)
cargo build --release
```

## Credit

- **[LSPosed](https://github.com/LSPosed/LSPosed)** — The Xposed framework this tool complements.
- **[rusqlite](https://github.com/rusqlite/rusqlite)** — SQLite bindings for Rust, with bundled SQLite for easy cross-compilation.
- **[clap](https://github.com/clap-rs/clap)** — Command-line argument parser for Rust.
