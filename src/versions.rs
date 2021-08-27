use regex::Regex;

pub fn is_version_up_to_or(required_version: &str, version: &str) -> bool {
    let version_regex = Regex::new(r"^(\d+)?(.(\d+))?(.(\d+))$").unwrap();
    let req_captures = version_regex.captures(required_version)
        .expect(&format!("Invalid required version '{}'!", required_version));
    let ver_captures = version_regex.captures(version)
        .expect(&format!("Invalid version '{}'!", required_version));

    if req_captures[1]
        .parse::<u8>()
        .expect(&format!("Invalid required version '{}'!", required_version))
        > ver_captures[1]
        .parse::<u8>()
        .expect(&format!("Invalid version '{}'!", version)) {
        return false;
    }

    if let Some(req_minor) = req_captures.get(2) {
        if req_minor
            .as_str()
            .parse::<u8>()
            .expect(&format!("Invalid required version '{}'!", required_version))
            > ver_captures.get(2)
            .expect(&format!("Invalid version '{}'!", version))
            .as_str()
            .parse::<u8>()
            .expect(&format!("Invalid version '{}'!", version)) {
            return false;
        }
    }

    if let Some(req_patch) = req_captures.get(3) {
        if req_patch
            .as_str()
            .parse::<u8>()
            .expect(&format!("Invalid required version '{}'!", required_version))
            > ver_captures.get(3)
            .expect(&format!("Invalid version '{}'!", version))
            .as_str()
            .parse::<u8>()
            .expect(&format!("Invalid version '{}'!", version)) {
            return false;
        }
    }

    true
}

pub fn check_version_requirement(required_version: &str, version: &str) -> bool {
    let up_to_or_regex: Regex = Regex::new(r"^\^").unwrap();
    let no_patch_regex: Regex = Regex::new(r"^\d+.\d+$").unwrap();

    let (is_up_to_or, required_version) = if let Some(res) = up_to_or_regex.find(required_version) {
        (true, &required_version[res.end()..required_version.len()])
    } else {
        (false, required_version)
    };

    let version = if no_patch_regex.is_match(required_version) {
        let res = Regex::new(r"^\d+.\d+").unwrap().find(version)
            .expect(&format!("Invalid version '{}'", version));
        &version[res.start()..res.end()]
    } else {
        version
    };

    //if is_up_to_or {
    //    is_version_up_to_or(required_version, version)
    //} else {
    //    required_version == version
    //}
    required_version == version
}
