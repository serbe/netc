use std::{convert::TryInto, time::Duration};

use bytes::Bytes;
use url::Url;

use crate::client::Client;
use crate::error::{Error, Result};
use crate::headers::Headers;
use crate::method::Method;
use crate::request::Request;
use crate::stream::MaybeHttpsStream;
use crate::utils::IntoUrl;
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
            method: Method::Get,
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
        let mut request = Request::new(&url, self.proxy.as_ref());
        request.headers(self.headers);
        let stream = match &self.proxy {
            Some(proxy) => match proxy.scheme() {
                "socks5" | "socks5h" => Ok(MaybeHttpsStream::socks(&proxy, &url).await?),
                "http" | "https" => {
                    if let (username, Some(password)) = (proxy.username(), proxy.password()) {
                        request.set_proxy_basic_auth(username, password);
                    };
                    Ok(MaybeHttpsStream::new(proxy).await?)
                }
                scheme => Err(Error::UnsupportedProxyScheme(scheme.to_owned())),
            },
            None => Ok(MaybeHttpsStream::new(&url).await?),
        }?;
        if let (username, Some(password)) = (url.username(), url.password()) {
            if let "http" | "https" = url.scheme() {
                request.set_basic_auth(username, password);
            }
        }
        request.method(self.method);
        request.version(self.version);
        request.opt_body(self.body);
        Ok(Client::new(request, url, self.proxy, stream, None))
    }

    pub fn url<U: IntoUrl>(mut self, value: U) -> ClientBuilder {
        match value.into_url() {
            Ok(url) => self.url = Some(url),
            _ => self.url = None,
        }
        self
    }

    pub fn proxy<P: IntoUrl>(mut self, value: P) -> ClientBuilder {
        match value.into_url() {
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

    pub fn get<U: IntoUrl>(mut self, value: U) -> ClientBuilder {
        match value.into_url() {
            Ok(url) => self.url = Some(url),
            _ => self.url = None,
        }
        self.method = Method::Get;
        self
    }

    pub fn post<U: IntoUrl>(mut self, value: U) -> ClientBuilder {
        match value.into_url() {
            Ok(url) => self.url = Some(url),
            _ => self.url = None,
        }
        self.method = Method::Post;
        self
    }

    pub fn options<U: IntoUrl>(mut self, value: U) -> ClientBuilder {
        match value.into_url() {
            Ok(url) => self.url = Some(url),
            _ => self.url = None,
        }
        self.method = Method::Options;
        self
    }

    pub fn delete<U: IntoUrl>(mut self, value: U) -> ClientBuilder {
        match value.into_url() {
            Ok(url) => self.url = Some(url),
            _ => self.url = None,
        }
        self.method = Method::Delete;
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
