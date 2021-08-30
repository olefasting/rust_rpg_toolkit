use regex::Regex;

pub fn check_version_requirement(required_version: &str, version: &str) -> bool {
    if required_version.starts_with("^") {
        return to_int_version(
            &required_version[1..required_version.len()]) <= to_int_version(version);
    }
    to_int_version(&required_version) == to_int_version(version)
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
    let regex = Regex::new(r"(?P<major>[0-9]+).(?P<minor>[0-9]+)(.(?P<patch>[0-9]+))?").unwrap();
    let captures = regex.captures(version)
        .expect(&format!("Invalid version string '{}'!", version));

    let major = captures["major"].parse::<u32>().unwrap();
    let minor = captures["minor"].parse::<u32>().unwrap();
    let patch = if let Some(res) = captures.name("patch") {
        res.as_str().parse::<u32>().unwrap()
    } else {
        0
    };

    (major << 24) + (minor << 16) + patch
}
