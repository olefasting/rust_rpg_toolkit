use regex::Regex;

pub fn check_version_requirement(required_version: u32, version: u32) -> bool {
    required_version == version
}

pub fn get_toolkit_version() -> String {
    format!(
        "{}.{}.{}",
        env!("CARGO_PKG_VERSION_MAJOR"),
        env!("CARGO_PKG_VERSION_MINOR"),
        env!("CARGO_PKG_VERSION_PATCH"),
    )
}

pub fn to_int_version(version: &str) -> u32 {
    let parts: Vec<&str> = version.split('.').collect();
    let major = parts[0].parse::<u32>().unwrap();
    let minor = parts[1].parse::<u32>().unwrap();
    let patch = parts.get(2).cloned().unwrap_or("0").parse::<u32>().unwrap();
    (major << 24) + (minor << 16) + patch
}
