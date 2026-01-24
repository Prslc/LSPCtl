#[cfg(debug_assertions)]
pub const MODULE_DB: &str = "./modules_config.db";

#[cfg(not(debug_assertions))]
pub const MODULE_DB: &str = "/data/adb/lspd/config/modules_config.db";