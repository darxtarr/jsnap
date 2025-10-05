use std::path::PathBuf;

pub const BRAND: &str = env!("SNAP_BRAND");
pub const VERSION: &str = env!("CARGO_PKG_VERSION");

pub fn user_capture_dir() -> PathBuf {
    let mut dir = dirs::data_local_dir().unwrap_or_else(|| PathBuf::from("."));
    dir.push(BRAND);
    dir.push("captures");
    dir
}

pub fn system_policy_path() -> PathBuf {
    PathBuf::from(format!(r"C:\ProgramData\{}\policy.json", BRAND))
}

pub fn registry_base() -> String {
    format!(r"Software\\{}", BRAND)
}
