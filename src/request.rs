use std::convert::TryInto;

use base64::encode;
use bytes::Bytes;
use url::Url;

use crate::{
    utils::{host_header, host_port},
    Headers, Method, Version,
};

#[derive(Clone, Debug)]
pub struct Request {
    pub(crate) method: Method,
    pub(crate) request_uri: String,
    pub(crate) version: Version,
    pub(crate) headers: Headers,
    pub(crate) host: String,
    pub(crate) body: Option<Bytes>,
}

impl Request {
    pub fn new(url: &Url, proxy: Option<&Url>) -> Request {
        let request_uri = match proxy {
            Some(_) => url.to_string(),
            None => url.path().to_string(),
        };
        Request {
            method: Method::Get,
            request_uri,
            version: Version::Http11,
            headers: Headers::default_http(&host_header(url)),
            host: host_port(url),
            body: None,
        }
    }

    /// Request-Line   = Method SP Request-URI SP HTTP-Version CRLF
    pub fn request_line(&self) -> String {
        format!(
            "{} {} {}\r\n",
            self.method,
            self.request_uri(),
            self.version
        )
    }

    pub fn user_agent(&self) -> Option<String> {
        self.headers.get("User-Agent")
    }

    pub fn referer(&self) -> Option<String> {
        self.headers.get("Referer")
    }

    pub fn headers(&mut self, headers: Headers) -> &mut Self {
        for (key, value) in headers.iter() {
            self.headers.insert(key, &value);
        }
        self
    }

    pub fn header<T: ToString + ?Sized, U: ToString + ?Sized>(
        &mut self,
        key: &T,
        val: &U,
    ) -> &mut Self {
        self.headers.insert(key, val);
        self
    }

    pub fn header_remove<T: ToString + ?Sized>(&mut self, key: &T) -> &mut Self {
        self.headers.remove(key);
        self
    }

    pub fn method(&mut self, method: Method) -> &mut Self {
        self.method = method;
        self
    }

    pub fn version(&mut self, version: Version) -> &mut Self {
        self.version = version;
        self
    }

    pub fn body<B>(&mut self, value: B) -> &mut Self
    where
        B: TryInto<Bytes>,
    {
        match value.try_into() {
            Ok(body) => {
                let content_len = body.len();
                self.body = Some(body);
                self.header("Content-Length", &content_len)
            }
            _ => {
                self.body = None;
                self.header_remove("Content-Length")
            }
        }
    }

    pub fn opt_body<B>(&mut self, value: Option<B>) -> &mut Self
    where
        B: TryInto<Bytes>,
    {
        match value {
            Some(body) => self.body(body),
            None => {
                self.body = None;
                self.header_remove("Content-Length")
            }
        }
    }

    pub fn set_basic_auth(&mut self, username: &str, password: &str) -> &mut Self {
        self.header(
            "Authorization",
            &format!("Basic {}", encode(&format!("{}:{}", username, password))),
        );
        self
    }

    pub fn set_proxy_basic_auth(&mut self, username: &str, password: &str) -> &mut Self {
        self.header(
            "Proxy-Authorization",
            &format!("Basic {}", encode(&format!("{}:{}", username, password))),
        );
        self
    }

    pub fn to_vec(&self) -> Vec<u8> {
        let request_line = format!(
            "{} {} {}{}",
            self.method, self.request_uri, self.version, "\r\n"
        );

        let headers: String = self
            .headers
            .iter()
            .map(|(k, v)| format!("{}: {}{}", k, v, "\r\n"))
            .collect();

        let mut request_msg = (request_line + &headers + "\r\n").as_bytes().to_vec();

        if let Some(b) = &self.body {
            request_msg.extend(b);
        }

        request_msg
    }

    pub fn content_length(&self) -> usize {
        self.headers
            .get("Content-Length")
            .map_or(0, |v| v.parse().map_or(0, |v| v))
    }

    pub fn get_body(&self) -> Option<Bytes> {
        self.body.clone()
    }

    pub fn get_headers(&self) -> Headers {
        self.headers.clone()
    }

    pub fn request_uri(&self) -> String {
        self.request_uri.clone()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const BODY: &str = "<html>hello</html>\r\n\r\nhello";
    const CONTENT_LENGTH: usize = 27;

    #[test]
    fn new_request() {
        let url = "https://api.ipify.org:1234/123/as".parse().unwrap();
        let mut request = Request::new(&url, None);
        request.body(BODY);
        assert_eq!(CONTENT_LENGTH, request.content_length());
        assert_eq!(BODY, request.get_body().unwrap().to_owned());
        assert_eq!("/123/as", &request.request_uri);
    }
}
