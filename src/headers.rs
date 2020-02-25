use std::collections::{hash_map, HashMap};
use std::fmt::{Display, Formatter};
use std::str::FromStr;

use crate::error::{Error, Result};

#[derive(Debug, PartialEq, Clone, Default)]
pub struct Headers(HashMap<String, String>);

impl Headers {
    pub fn new() -> Headers {
        Headers(HashMap::new())
    }

    pub fn with_capacity(capacity: usize) -> Headers {
        Headers(HashMap::with_capacity(capacity))
    }

    pub fn iter(&self) -> hash_map::Iter<String, String> {
        self.0.iter()
    }

    pub fn get<T: ToString + ?Sized>(&self, k: &T) -> Option<String> {
        match self.0.get(&k.to_string().to_lowercase()) {
            Some(value) => Some(value.to_string()),
            None => None,
        }
    }

    pub fn insert<T: ToString + ?Sized, U: ToString + ?Sized>(
        &mut self,
        key: &T,
        val: &U,
    ) -> Option<String> {
        self.0
            .insert(key.to_string().to_lowercase(), val.to_string())
    }

    pub fn default_http(host: &str) -> Headers {
        let mut headers = Headers::with_capacity(2);
        headers.insert("Host", host);
        headers.insert("Connection", "Close");
        headers
    }
}

impl FromStr for Headers {
    type Err = Error;

    fn from_str(s: &str) -> Result<Headers> {
        let headers = s.trim();

        if headers.lines().all(|e| e.contains(':')) {
            let headers = headers
                .lines()
                .map(|elem| {
                    let idx = elem.find(':').unwrap();
                    let (key, value) = elem.split_at(idx);
                    (
                        key.to_string().to_lowercase(),
                        value[1..].trim().to_string(),
                    )
                })
                .collect();

            Ok(Headers(headers))
        } else {
            Err(Error::ParseHeaders)
        }
    }
}

impl From<HashMap<String, String>> for Headers {
    fn from(map: HashMap<String, String>) -> Headers {
        Headers(map)
    }
}

impl From<Headers> for HashMap<String, String> {
    fn from(map: Headers) -> HashMap<String, String> {
        map.0
    }
}

impl Display for Headers {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        let headers: String = self
            .iter()
            .map(|(key, val)| format!("  {}: {}\r\n", key, val))
            .collect();

        write!(f, "{{\r\n{}}}", headers)
    }
}
