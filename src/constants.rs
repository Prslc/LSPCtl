#[cfg(target_arch = "x86_64")]
pub const MODULE_DB: &str = "./modules_config.db";

#[cfg(target_arch = "aarch64")]
pub const MODULE_DB: &str = "/data/adb/lspd/config/modules_config.db";