use std::convert::TryInto;

use base64::encode;
use bytes::Bytes;
use uri::Uri;

use crate::headers::Headers;
use crate::method::Method;
use crate::version::Version;

#[derive(Clone, Debug)]
pub struct Request {
    method: Method,
    request_uri: String,
    version: Version,
    headers: Headers,
    host: String,
    content_len: usize,
    body: Option<Bytes>,
    using_proxy: bool,
}

impl Request {
    pub fn new(uri: &Uri, using_proxy: bool) -> Request {
        let request_uri = if using_proxy {
            uri.request_uri().to_string()
        } else {
            uri.proxy_request_uri()
        };
        Request {
            method: Method::GET,
            request_uri,
            version: Version::Http11,
            headers: Headers::default_http(&uri.host_header()),
            host: uri.host_port(),
            content_len: 0,
            body: None,
            using_proxy,
        }
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

    pub fn method(&mut self, method: Method) -> &mut Self {
        self.method = method;
        self
    }

    pub fn version(&mut self, version: Version) -> &mut Self {
        self.version = version;
        self
    }

    pub fn body<B>(&mut self, value: Option<B>) -> &mut Self
    where
        B: TryInto<Bytes>,
    {
        match value {
            Some(some_value) => match some_value.try_into() {
                Ok(body) => self.body = Some(body),
                _ => self.body = None,
            },
            None => self.body = None,
        }
        self
    }

    pub fn set_basic_auth(&mut self, username: &str, password: &str) -> &mut Self {
        self.header(
            "Authorization",
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
}
