use std::fmt;
use std::str::FromStr;

use crate::error::{Error, Result};

#[derive(PartialEq, Copy, Clone)]
pub enum Version {
    Http09,
    Http10,
    Http11,
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
        })
    }
}

impl fmt::Display for Version {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let version = match self {
            Version::Http09 => "HTTP/0.9",
            Version::Http10 => "HTTP/1.0",
            Version::Http11 => "HTTP/1.1",
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

        assert_eq!(version09_expect, version09);
        assert_eq!(version10_expect, version10);
        assert_eq!(version11_expect, version11);
    }
}
