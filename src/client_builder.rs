use std::convert::TryInto;

use bytes::Bytes;
use std::time::Duration;
use uri::Uri;

use crate::client::Client;
use crate::error::{Error, Result};
use crate::headers::Headers;
use crate::method::Method;
use crate::request::Request;
use crate::stream::MaybeHttpsStream;
use crate::version::Version;

#[derive(Debug, PartialEq)]
pub struct ClientBuilder {
    uri: Option<Uri>,
    headers: Headers,
    method: Method,
    version: Version,
    body: Option<Bytes>,
    proxy: Option<Uri>,
    nodelay: bool,
    timeout: Option<Duration>,
    connect_timeout: Option<Duration>,
}

impl Default for ClientBuilder {
    fn default() -> Self {
        Self::new()
    }
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
            proxy: None,
            nodelay: false,
            timeout: None,
            connect_timeout: None,
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
                MaybeHttpsStream::new(&proxy).await?
            }
        } else {
            MaybeHttpsStream::new(&uri).await?
        };
        let mut request = Request::new(&uri, self.proxy.is_some());
        request.method(self.method);
        request.headers(headers);
        request.version(self.version);
        request.opt_body(self.body);
        Ok(Client::new(request, uri, self.proxy, stream, None))
    }

    pub fn uri<U>(mut self, value: U) -> ClientBuilder
    where
        U: TryInto<Uri>,
    {
        match value.try_into() {
            Ok(uri) => self.uri = Some(uri),
            _ => self.uri = None,
        }
        self
    }

    pub fn proxy<P>(mut self, value: P) -> ClientBuilder
    where
        P: TryInto<Uri>,
    {
        match value.try_into() {
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

    pub fn header_remove<T: ToString + ?Sized>(mut self, key: &T) -> ClientBuilder {
        self.headers.remove(key);
        self
    }

    pub fn method<M>(mut self, value: M) -> ClientBuilder
    where
        M: TryInto<Method>,
    {
        match value.try_into() {
            Ok(method) => self.method = method,
            _ => (),
        }
        self
    }

    pub fn get<U>(mut self, value: U) -> ClientBuilder
    where
        U: TryInto<Uri>,
    {
        match value.try_into() {
            Ok(uri) => self.uri = Some(uri),
            _ => self.uri = None,
        }
        self.method = Method::GET;
        self
    }

    pub fn post<U>(mut self, value: U) -> ClientBuilder
    where
        U: TryInto<Uri>,
    {
        match value.try_into() {
            Ok(uri) => self.uri = Some(uri),
            _ => self.uri = None,
        }
        self.method = Method::POST;
        self
    }

    pub fn put<U>(mut self, value: U) -> ClientBuilder
    where
        U: TryInto<Uri>,
    {
        match value.try_into() {
            Ok(uri) => self.uri = Some(uri),
            _ => self.uri = None,
        }
        self.method = Method::PUT;
        self
    }

    pub fn patch<U>(mut self, value: U) -> ClientBuilder
    where
        U: TryInto<Uri>,
    {
        match value.try_into() {
            Ok(uri) => self.uri = Some(uri),
            _ => self.uri = None,
        }
        self.method = Method::PATCH;
        self
    }

    pub fn delete<U>(mut self, value: U) -> ClientBuilder
    where
        U: TryInto<Uri>,
    {
        match value.try_into() {
            Ok(uri) => self.uri = Some(uri),
            _ => self.uri = None,
        }
        self.method = Method::DELETE;
        self
    }

    pub fn version<V>(mut self, value: V) -> ClientBuilder
    where
        V: TryInto<Version>,
    {
        match value.try_into() {
            Ok(version) => self.version = version,
            _ => (),
        }
        self
    }

    pub fn body<B>(mut self, value: B) -> ClientBuilder
    where
        B: TryInto<Bytes>,
    {
        match value.try_into() {
            Ok(body) => {
                self.body = Some(body);
                self
            }
            Err(_) => {
                self.body = None;
                self
            }
        }
    }

    pub fn json<B>(mut self, value: B) -> ClientBuilder
    where
        B: TryInto<Bytes>,
    {
        match value.try_into() {
            Ok(body) => self.body(body).header("Content-Type", "application/json"),
            Err(_) => {
                self.body = None;
                self.header_remove("Content-Type")
            }
        }
    }

    pub fn tcp_nodelay(mut self) -> ClientBuilder {
        self.nodelay = true;
        self
    }

    pub fn timeout(mut self, timeout: Duration) -> ClientBuilder {
        self.timeout = Some(timeout);
        self
    }

    pub fn connect_timeout(mut self, timeout: Duration) -> ClientBuilder {
        self.connect_timeout = Some(timeout);
        self
    }

    pub fn referer<U>(self, value: U) -> ClientBuilder
    where
        U: TryInto<Uri>,
    {
        match value.try_into() {
            Ok(uri) => self.header("Referer", &uri),
            _ => self,
        }
    }
}
