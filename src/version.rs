use std::{fmt, str::FromStr};

use crate::Error;

#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub enum Version {
    Http09,
    Http10,
    #[default]
    Http11,
    H2,
    H3,
}

// HTTP-Version   = "HTTP" "/" 1*DIGIT "." 1*DIGIT
impl Version {
    fn as_str(&self) -> &str {
        match self {
            Version::Http09 => "HTTP/0.9",
            Version::Http10 => "HTTP/1.0",
            Version::Http11 => "HTTP/1.1",
            Version::H2 => "HTTP/2",
            Version::H3 => "HTTP/3",
        }
    }
}

impl FromStr for Version {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Error> {
        match s.to_uppercase().as_str() {
            "HTTP/0.9" => Ok(Version::Http09),
            "HTTP/1.0" => Ok(Version::Http10),
            "HTTP/1.1" => Ok(Version::Http11),
            "HTTP/2" => Ok(Version::H2),
            "HTTP/3" => Ok(Version::H3),
            _ => Err(Error::UnsupportedVersion(s.to_owned())),
        }
    }
}

impl fmt::Debug for Version {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.as_str())
    }
}

impl fmt::Display for Version {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.as_str())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn version_parse() {
        let version09 = Version::Http09;
        let version09_expect: Version = "HTTP/0.9".parse().unwrap();
        let version10 = Version::Http10;
        let version10_expect: Version = "HTTP/1.0".parse().unwrap();
        let version11 = Version::Http11;
        let version11_expect: Version = "HTTP/1.1".parse().unwrap();
        let version2 = Version::H2;
        let version2_expect: Version = "HTTP/2".parse().unwrap();
        let version3 = Version::H3;
        let version3_expect: Version = "HTTP/3".parse().unwrap();
        let version_error_str = "HTT/4";
        let version_error = Error::UnsupportedVersion(version_error_str.to_string());
        let version_error_expect = version_error_str.parse::<Version>();

        assert_eq!(version09_expect, version09);
        assert_eq!(version10_expect, version10);
        assert_eq!(version11_expect, version11);
        assert_eq!(version2_expect, version2);
        assert_eq!(version3_expect, version3);
        assert_eq!(version_error_expect, Err(version_error));
    }

    #[test]
    fn version_to_string() {
        let version09 = Version::Http09;
        let version09_str = "HTTP/0.9";
        let version10 = Version::Http10;
        let version10_str = "HTTP/1.0";
        let version11 = Version::Http11;
        let version11_str = "HTTP/1.1";
        let version2 = Version::H2;
        let version2_str = "HTTP/2";
        let version3 = Version::H3;
        let version3_str = "HTTP/3";

        assert_eq!(version09.as_str(), version09_str);
        assert_eq!(version10.as_str(), version10_str);
        assert_eq!(version11.as_str(), version11_str);
        assert_eq!(version2.as_str(), version2_str);
        assert_eq!(version3.as_str(), version3_str);
        assert_eq!(format!("{version11}"), version11_str.to_string());
        assert_eq!(format!("{version11:?}"), version11_str.to_string());
    }
}
