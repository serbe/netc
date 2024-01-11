use base64::{engine::general_purpose::STANDARD, Engine};
use bytes::Bytes;
use std::fmt::Write;
use url::Url;

use crate::{utils::request_uri, Headers, Method, Version};

#[derive(Clone, Debug)]
pub struct Request {
    pub(crate) url: Url,
    pub(crate) method: Method,
    pub(crate) version: Version,
    pub(crate) headers: Headers,
    pub(crate) body: Option<Bytes>,
    pub(crate) proxy: Option<Url>,
}

impl Request {
    pub fn new(method: Method, url: &Url) -> Request {
        Request {
            url: url.clone(),
            method,
            version: Version::Http11,
            headers: Headers::default_http(url),
            body: None,
            proxy: None,
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

    pub fn remove_header<T: ToString + ?Sized>(&mut self, key: &T) -> &mut Self {
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

    pub fn body<B: Into<Bytes>>(&mut self, value: B) -> &mut Self {
        let body = value.into();
        let content_len = body.len();
        self.body = Some(body);
        self.header("Content-Length", &content_len)
    }

    pub fn opt_body<B: Into<Bytes>>(&mut self, value: Option<B>) -> &mut Self {
        match value {
            Some(body) => self.body(body),
            None => {
                self.body = None;
                self.remove_header("Content-Length")
            }
        }
    }

    pub fn set_basic_auth(&mut self, username: &str, password: &str) -> &mut Self {
        self.header(
            "Authorization",
            &format!(
                "Basic {}",
                STANDARD.encode(format!("{username}:{password}"))
            ),
        );
        self
    }

    pub fn set_proxy_basic_auth(&mut self, username: &str, password: &str) -> &mut Self {
        self.header(
            "Proxy-Authorization",
            &format!(
                "Basic {}",
                STANDARD.encode(format!("{username}:{password}"))
            ),
        );
        self
    }

    pub fn to_vec(&self) -> Vec<u8> {
        let headers: String = self
            .headers
            .iter()
            .fold(String::new(), |mut output, (k, v)| {
                let _ = write!(output, "{}: {}\r\n", k, v);
                output
            });

        let mut request_msg = (self.request_line() + &headers + "\r\n")
            .as_bytes()
            .to_vec();

        if let Some(b) = &self.body {
            request_msg.extend(b);
        }

        request_msg
    }

    pub fn content_length(&self) -> Option<usize> {
        self.headers.content_length()
    }

    pub fn get_body(&self) -> Option<Bytes> {
        self.body.clone()
    }

    pub fn get_headers(&self) -> &Headers {
        &self.headers
    }

    pub fn proxy(&mut self, proxy: Option<&Url>) {
        match proxy {
            Some(proxy) => match (proxy.scheme(), proxy.password()) {
                ("http" | "https", Some(password)) => {
                    self.set_proxy_basic_auth(proxy.username(), password)
                }
                _ => self.remove_header("Proxy-Authorization"),
            },
            None => self.remove_header("Proxy-Authorization"),
        };
        self.proxy = proxy.cloned();
    }

    pub fn request_uri(&self) -> String {
        request_uri(&self.url, self.proxy.is_some())
    }

    pub fn url(&self) -> Url {
        self.url.clone()
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
        let mut request = Request::new(Method::Get, &url);
        request.body(BODY);
        assert_eq!(CONTENT_LENGTH, request.content_length().unwrap());
        assert_eq!(BODY, request.get_body().unwrap().to_owned());
        assert_eq!("/123/as", &request.request_uri());
    }

    // #[test]
    // fn delete_request() {
    //     let url = format!("{}{}", HTTPBIN, Method::Delete);
    //     let mut request = delete(url).unwrap();
    //     request.header(ACCEPT, ACCEPT_JSON);
    //     let body = request.get_body()
    //     assert_eq!(CONTENT_LENGTH, request.content_length());
    //     assert_eq!(BODY, request.get_body().unwrap().to_owned());
    //     assert_eq!("/123/as", &request.request_uri());
    // }
}
