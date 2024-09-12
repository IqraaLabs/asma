/// # Version
/// TODO: make versions comparable
/// TODO: is it good to make field public ?
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Version {
    pub major: u64,
    pub minor: u64,
}

impl Version {
    pub fn parse(version: String) -> Result<Version, &'static str> {
        let digits: Vec<&str> = version.split('.').collect();
        match digits.len() {
            2 => {
                let major = match digits[0].parse::<u64>() {
                    Ok(n) => n,
                    Err(_) => return Err("failed to parse major version"),
                };

                let minor = match digits[1].parse::<u64>() {
                    Ok(n) => n,
                    Err(_) => return Err("failed to parse minor version"),
                };

                Ok(Version { major, minor })
            }
            _ => {
                Err("version format is not valid. the version should only contains a major and a minor version")
            }
        }
    }
}


#[test]
fn test_parse_valid_version() {
    let string_version = "1.1".to_string();
    let version = Version::parse(string_version);
    assert_eq!(version, Ok(Version { major: 1, minor: 1 }));
}


#[test]
fn test_parse_version_with_too_many_dots() {
    let string_version = "1.1.1".to_string();
    let version = Version::parse(string_version);
    assert!(version.is_err());
}

#[test]
fn test_parse_version_with_no_digits() {
    let string_version = ".".to_string();
    let version = Version::parse(string_version);
    assert!(version.is_err());
}

#[test]
fn test_parse_version_with_no_dots() {
    let string_version = "1".to_string();
    let version = Version::parse(string_version);
    assert!(version.is_err());
}


#[test]
fn test_parse_version_with_multiple_digits() {
    let string_version = "11.11".to_string();
    let version = Version::parse(string_version);
    assert_eq!(version, Ok(Version { major: 11, minor: 11 }));
}
