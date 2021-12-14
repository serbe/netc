use std::{
    collections::{hash_map, HashMap},
    fmt::{Display, Formatter},
    str::FromStr,
};

use crate::Error;

#[derive(Debug, PartialEq, Clone)]
pub struct Headers(HashMap<String, String>);

impl Headers {
    pub fn new() -> Headers {
        Headers(HashMap::new())
    }

    pub fn with_capacity(capacity: usize) -> Headers {
        Headers(HashMap::with_capacity(capacity))
    }

    pub fn default_http(host: &str) -> Headers {
        let mut headers = Headers::with_capacity(2);
        headers.insert("Host", host);
        headers.insert("Connection", "Close");
        headers
    }

    pub fn iter(&self) -> hash_map::Iter<String, String> {
        self.0.iter()
    }

    pub fn get<T: ToString + ?Sized>(&self, key: &T) -> Option<String> {
        self.0
            .get(&key.to_string().to_lowercase())
            .map(|value| value.to_string())
    }

    pub fn insert<T: ToString + ?Sized, U: ToString + ?Sized>(
        &mut self,
        key: &T,
        value: &U,
    ) -> Option<String> {
        self.0
            .insert(key.to_string().to_lowercase(), value.to_string())
    }

    pub fn remove<T: ToString + ?Sized>(&mut self, key: &T) -> Option<String> {
        self.0.remove(&key.to_string().to_lowercase())
    }

    pub fn get_array<T: ToString + ?Sized>(&self, key: &T) -> Vec<String> {
        self.get(key).map_or_else(Vec::new, |value| {
            value
                .split(',')
                .map(|value| value.trim().to_string())
                .filter(|value| !value.is_empty())
                .collect()
        })
    }
}

impl Default for Headers {
    fn default() -> Self {
        Self::new()
    }
}

impl FromStr for Headers {
    type Err = Error;

    fn from_str(s: &str) -> Result<Headers, Error> {
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
        let headers = map
            .iter()
            .map(|(key, value)| (key.to_string().to_lowercase(), value.to_string()))
            .collect();
        Headers(headers)
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

#[cfg(test)]
mod tests {
    use super::*;

    const HEADERS: &str = "Date: Sat, 11 Jan 2003 02:44:04 GMT\r\n\
                           Content-Type: text/html\r\n\
                           Content-Length: 100\r\n";

    #[test]
    fn headers_new() {
        assert_eq!(Headers::new(), Headers(HashMap::new()));
    }

    #[test]
    fn headers_get() {
        let mut headers = Headers::with_capacity(2);
        headers.insert("Date", "Sat, 11 Jan 2003 02:44:04 GMT");

        assert_eq!(
            headers.get("Date"),
            Some("Sat, 11 Jan 2003 02:44:04 GMT".to_string())
        );
    }

    #[test]
    fn headers_insert() {
        let mut headers_expect = HashMap::new();
        headers_expect.insert("connection".to_string(), "Close".to_string());
        let headers_expect = Headers(headers_expect);
        let mut headers = Headers::new();
        headers.insert("Connection", "Close");

        assert_eq!(headers_expect, headers);
    }

    #[test]
    fn headers_default_http() {
        let host = "doc.rust-lang.org";
        let mut headers = Headers::with_capacity(2);
        headers.insert("Host", "doc.rust-lang.org");
        headers.insert("Connection", "Close");

        assert_eq!(Headers::default_http(&host), headers);
    }

    #[test]
    fn headers_from_str() {
        let mut headers_expect = HashMap::with_capacity(2);
        headers_expect.insert(
            "Date".to_string(),
            "Sat, 11 Jan 2003 02:44:04 GMT".to_string(),
        );
        headers_expect.insert("Content-Type".to_string(), "text/html".to_string());
        headers_expect.insert("Content-Length".to_string(), "100".to_string());
        let headers = HEADERS.parse::<Headers>().unwrap();

        assert_eq!(headers, Headers::from(headers_expect));
    }

    #[test]
    fn headers_from() {
        let mut headers_expect = HashMap::with_capacity(4);
        headers_expect.insert(
            "date".to_string(),
            "Sat, 11 Jan 2003 02:44:04 GMT".to_string(),
        );
        headers_expect.insert("content-type".to_string(), "text/html".to_string());
        headers_expect.insert("content-length".to_string(), "100".to_string());

        assert_eq!(
            Headers(headers_expect.clone()),
            Headers::from(headers_expect)
        );
    }

    #[test]
    fn headers_case_insensitive() {
        let header_names = ["Host", "host", "HOST", "HoSt"];
        let mut headers = Headers::with_capacity(1);
        headers.insert("Host", "doc.rust-lang.org");

        for name in header_names.iter() {
            assert_eq!(headers.get(name), Some("doc.rust-lang.org".to_string()));
        }
    }

    #[test]
    fn hash_map_from_headers() {
        let mut headers = Headers::with_capacity(4);
        headers.insert("Date", "Sat, 11 Jan 2003 02:44:04 GMT");
        headers.insert("Content-Type", "text/html");
        headers.insert("Content-Length", "100");

        let mut headers_expect = HashMap::with_capacity(4);
        headers_expect.insert(
            "date".to_string(),
            "Sat, 11 Jan 2003 02:44:04 GMT".to_string(),
        );
        headers_expect.insert("content-type".to_string(), "text/html".to_string());
        headers_expect.insert("content-length".to_string(), "100".to_string());

        assert_eq!(HashMap::from(headers), headers_expect);
    }

    #[test]
    fn headers_get_array() {
        let mut headers = Headers::with_capacity(2);
        headers.insert("Accept-Encoding", "compress, gzip");
        headers.insert("Accept-Language", "da, en-gb;q=0.8, en;q=0.7");

        assert_eq!(
            headers.get_array("accept-encoding"),
            vec!["compress".to_string(), "gzip".to_string()]
        );
        assert_eq!(
            headers.get_array("accept-language"),
            vec![
                "da".to_string(),
                "en-gb;q=0.8".to_string(),
                "en;q=0.7".to_string()
            ]
        );
    }

    #[test]
    fn headers_array_length() {
        let mut headers = Headers::with_capacity(2);
        headers.insert("Key1", "da, en-gb;q=0.8, en;q=0.7");
        headers.insert("Key2", "da, en-gb;q=0.8, , en;q=0.7");

        assert_eq!(headers.get_array("key1").len(), 3);
        assert_eq!(headers.get_array("key2").len(), 3);
    }
}
