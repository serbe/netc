use std::{convert::TryInto, time::Duration};

use bytes::Bytes;
use url::Url;

use crate::client::Client;
use crate::error::{Error, Result};
use crate::headers::Headers;
use crate::method::Method;
use crate::request::Request;
use crate::stream::MaybeHttpsStream;
use crate::utils::{base64_auth, IntoUrl};
use crate::version::Version;

#[derive(Debug, PartialEq)]
pub struct ClientBuilder {
    url: Option<Url>,
    headers: Headers,
    method: Method,
    version: Version,
    body: Option<Bytes>,
    proxy: Option<Url>,
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
            url: None,
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
        let url = self.url.ok_or(Error::EmptyUrl)?;
        let mut headers = self.headers;
        let stream = match &self.proxy {
            Some(proxy) => match proxy.scheme() {
                "socks5" | "socks5h" => Ok(MaybeHttpsStream::socks(&proxy, &url).await?),
                "http" | "https" => {
                    if let Some(auth) = base64_auth(&proxy) {
                        headers.insert("Proxy-Authorization", format!("Basic {}", auth).as_str());
                    };
                    Ok(MaybeHttpsStream::new(&proxy).await?)
                }
                scheme => Err(Error::UnsupportedProxyScheme(scheme.to_owned())),
            },
            None => Ok(MaybeHttpsStream::new(&url).await?),
        }?;
        let mut request = Request::new(&url, self.proxy.as_ref());
        request.method(self.method);
        request.headers(headers);
        request.version(self.version);
        request.opt_body(self.body);
        Ok(Client::new(request, url, self.proxy, stream, None))
    }

    pub fn url<T: IntoUrl>(mut self, value: T) -> ClientBuilder {
        match value.try_into() {
            Ok(url) => self.url = Some(url),
            _ => self.url = None,
        }
        self
    }

    pub fn proxy<P>(mut self, value: P) -> ClientBuilder
    where
        P: TryInto<Url>,
    {
        match value.try_into() {
            Ok(url) => self.proxy = Some(url),
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
        if let Ok(method) = value.try_into() {
            self.method = method
        }
        self
    }

    pub fn get<U>(mut self, value: U) -> ClientBuilder
    where
        U: TryInto<Url>,
    {
        match value.try_into() {
            Ok(url) => self.url = Some(url),
            _ => self.url = None,
        }
        self.method = Method::GET;
        self
    }

    pub fn post<U>(mut self, value: U) -> ClientBuilder
    where
        U: TryInto<Url>,
    {
        match value.try_into() {
            Ok(url) => self.url = Some(url),
            _ => self.url = None,
        }
        self.method = Method::POST;
        self
    }

    pub fn options<U>(mut self, value: U) -> ClientBuilder
    where
        U: TryInto<Url>,
    {
        match value.try_into() {
            Ok(url) => self.url = Some(url),
            _ => self.url = None,
        }
        self.method = Method::OPTIONS;
        self
    }

    pub fn delete<U>(mut self, value: U) -> ClientBuilder
    where
        U: TryInto<Url>,
    {
        match value.try_into() {
            Ok(url) => self.url = Some(url),
            _ => self.url = None,
        }
        self.method = Method::DELETE;
        self
    }

    pub fn version<V>(mut self, value: V) -> ClientBuilder
    where
        V: TryInto<Version>,
    {
        if let Ok(version) = value.try_into() {
            self.version = version
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
        U: TryInto<Url>,
    {
        match value.try_into() {
            Ok(url) => self.header("Referer", &url),
            _ => self,
        }
    }
}
