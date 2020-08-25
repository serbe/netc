use std::fmt;
use std::str::FromStr;

use crate::error::{Error, Result};

#[derive(PartialEq, Copy, Clone)]
pub enum Version {
    Http09,
    Http10,
    Http11,
    H2,
    H3,
}

impl Default for Version {
    fn default() -> Self {
        Version::Http11
    }
}

impl FromStr for Version {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self> {
        match s.to_uppercase().as_str() {
            "HTTP/0.9" => Ok(Version::Http09),
            "HTTP/1.0" => Ok(Version::Http10),
            "HTTP/1.1" => Ok(Version::Http11),
            "HTTP/2.0" => Ok(Version::H2),
            "HTTP/3.0" => Ok(Version::H3),
            _ => Err(Error::UnsupportedVersion(s.to_owned())),
        }
    }
}

impl fmt::Debug for Version {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str(match self {
            Version::Http09 => "HTTP/0.9",
            Version::Http10 => "HTTP/1.0",
            Version::Http11 => "HTTP/1.1",
            Version::H2 => "HTTP/2.0",
            Version::H3 => "HTTP/3.0",
        })
    }
}

impl fmt::Display for Version {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let version = match self {
            Version::Http09 => "HTTP/0.9",
            Version::Http10 => "HTTP/1.0",
            Version::Http11 => "HTTP/1.1",
            Version::H2 => "HTTP/2.0",
            Version::H3 => "HTTP/3.0",
        };

        write!(f, "{}", version)
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
        let version2_expect: Version = "HTTP/2.0".parse().unwrap();
        let version3 = Version::H3;
        let version3_expect: Version = "HTTP/3.0".parse().unwrap();

        assert_eq!(version09_expect, version09);
        assert_eq!(version10_expect, version10);
        assert_eq!(version11_expect, version11);
        assert_eq!(version2_expect, version2);
        assert_eq!(version3_expect, version3);
    }
}
