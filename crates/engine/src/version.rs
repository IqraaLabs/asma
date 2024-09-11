/// # Version
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Version {
    major: u64,
    minor: u64,
}

impl Version {
    pub fn parse(version: String) -> Result<Version, &'static str> {
        let digits: Vec<&str> = version.split('.').collect();
        match digits.len() {
            2 => {
                let major = digits[0].parse::<u64>().unwrap();
                let minor = digits[1].parse::<u64>().unwrap();
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
fn test_parse_invalid_version() {
    let string_version = "1.1.1".to_string();
    let version = Version::parse(string_version);
    assert!(version.is_err());
}
