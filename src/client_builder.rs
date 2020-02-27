// use std::time::Duration;
use uri::Uri;

use crate::client::Client;
use crate::error::{Error, Result};
use crate::headers::Headers;
use crate::method::Method;
use crate::request::Request;
use crate::stream::MaybeHttpsStream;
use crate::version::Version;

#[derive(Debug)]
pub struct ClientBuilder {
    uri: Option<Uri>,
    headers: Headers,
    method: Method,
    version: Version,
    body: Option<Vec<u8>>,
    referer: bool,
    proxy: Option<Uri>,
    nodelay: bool,
    // timeout: Option<Duration>,
    // connect_timeout: Option<Duration>,
}

impl ClientBuilder {
    pub fn new() -> Self {
        let headers = Headers::new();
        ClientBuilder {
            uri: None,
            headers,
            method: Method::GET,
            version: Version::Http11,
            body: None,
            referer: true,
            proxy: None,
            nodelay: false,
            // timeout: None,
            // connect_timeout: None,
        }
    }

    pub async fn build(self) -> Result<Client> {
        let uri = self.uri.ok_or(Error::EmptyUri)?;
        let mut headers = self.headers;
        let stream = if let Some(proxy) = &self.proxy {
            if proxy.scheme() == "socks5" {
                MaybeHttpsStream::socks(&proxy, &uri).await?
            } else {
                if let Some(auth) = proxy.base64_auth() {
                    headers.insert("Proxy-Authorization", format!("Basic {}", auth).as_str());
                };
                MaybeHttpsStream::new(&uri).await?
            }
        } else {
            MaybeHttpsStream::new(&uri).await?
        };
        let mut request = Request::new(&uri, self.proxy.is_some());
        request.method(self.method);
        request.headers(headers);
        request.version(self.version);
        request.body(self.body);
        Ok(Client::from(request, uri, self.proxy, stream, None))
    }

    pub fn uri(mut self, uri: &str) -> ClientBuilder {
        match uri.parse() {
            Ok(uri) => self.uri = Some(uri),
            _ => self.uri = None,
        }
        self
    }

    pub fn proxy(mut self, proxy: &str) -> ClientBuilder {
        match proxy.parse() {
            Ok(uri) => self.proxy = Some(uri),
            _ => self.proxy = None,
        }
        self
    }

    pub fn headers(mut self, headers: Headers) -> ClientBuilder {
        for (key, value) in headers.iter() {
            self.headers.insert(key, &value);
        }
        self
    }

    pub fn header<T: ToString + ?Sized, U: ToString + ?Sized>(
        mut self,
        key: &T,
        value: &U,
    ) -> ClientBuilder {
        self.headers.insert(key, value);
        self
    }

    pub fn method(mut self, method: &str) -> ClientBuilder {
        if let Ok(method) = method.parse() {
            self.method = method;
        }
        self
    }

    pub fn get(mut self, uri: &str) -> ClientBuilder {
        if let Ok(uri) = uri.parse() {
            self.uri = Some(uri);
        }
        self.method = Method::GET;
        self
    }

    pub fn post(mut self, uri: &str) -> ClientBuilder {
        if let Ok(uri) = uri.parse() {
            self.uri = Some(uri);
        }
        self.method = Method::POST;
        self
    }

    pub fn put(mut self, uri: &str) -> ClientBuilder {
        if let Ok(uri) = uri.parse() {
            self.uri = Some(uri);
        }
        self.method = Method::PUT;
        self
    }

    pub fn patch(mut self, uri: &str) -> ClientBuilder {
        if let Ok(uri) = uri.parse() {
            self.uri = Some(uri);
        }
        self.method = Method::PATCH;
        self
    }

    pub fn delete(mut self, uri: &str) -> ClientBuilder {
        if let Ok(uri) = uri.parse() {
            self.uri = Some(uri);
        }
        self.method = Method::DELETE;
        self
    }

    pub fn head(mut self, uri: &str) -> ClientBuilder {
        if let Ok(uri) = uri.parse() {
            self.uri = Some(uri);
        }
        self.method = Method::HEAD;
        self
    }

    pub fn version(mut self, version: &str) -> ClientBuilder {
        if let Ok(version) = version.parse() {
            self.version = version;
        }
        self
    }

    pub fn body(mut self, body: &[u8]) -> ClientBuilder {
        self.body = Some(body.to_vec());
        self
    }

    pub fn tcp_nodelay(mut self) -> ClientBuilder {
        self.nodelay = true;
        self
    }

    pub fn referer(mut self, enable: bool) -> ClientBuilder {
        self.referer = enable;
        self
    }

    // pub fn timeout(mut self, timeout: Duration) -> ClientBuilder {
    //     self.timeout = Some(timeout);
    //     self
    // }

    // pub fn connect_timeout(mut self, timeout: Duration) -> ClientBuilder {
    //     self.connect_timeout = Some(timeout);
    //     self
    // }
}
