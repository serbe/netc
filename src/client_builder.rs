use std::{convert::TryInto, time::Duration};

use bytes::Bytes;
use url::Url;

use crate::{utils::IntoUrl, Client, Error, Headers, HttpStream, Method, Request, Version};

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Config {
    pub nodelay: bool,
    pub timeout: Option<Duration>,
    pub connect_timeout: Option<Duration>,
    pub redirects: usize,
    pub max_redirects: usize,
}

impl Config {
    fn new() -> Self {
        Config {
            nodelay: false,
            timeout: None,
            connect_timeout: None,
            redirects: 0,
            max_redirects: 10,
        }
    }
}

impl Default for Config {
    fn default() -> Self {
        Config::new()
    }
}

#[derive(Debug, PartialEq, Eq)]
pub struct ClientBuilder {
    pub(crate) url: Option<Url>,
    pub(crate) headers: Headers,
    pub(crate) method: Method,
    pub(crate) version: Version,
    pub(crate) body: Option<Bytes>,
    pub(crate) proxy: Option<Url>,
    pub(crate) config: Config,
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
            config: Config::new(),
        }
    }

    pub fn from_client(client: &Client) -> Self {
        ClientBuilder {
            url: Some(client.request.url.clone()),
            headers: client.request.headers.clone(),
            method: client.request.method.clone(),
            version: client.request.version,
            body: client.request.body.clone(),
            proxy: client.request.proxy.clone(),
            config: client.config.clone(),
        }
    }

    pub async fn build(self) -> Result<Client, Error> {
        let url = self.url.ok_or(Error::EmptyUrl)?;
        let mut request = Request::new(Method::Get, &url);
        request.proxy(self.proxy.as_ref());
        request.headers(self.headers);
        request.method(self.method);
        request.version(self.version);
        request.opt_body(self.body);
        let mut stream = HttpStream::from_request(&request).await?;
        if self.config.nodelay {
            stream.set_nodelay(true)?;
        };
        Ok(Client::new(request, stream, None, self.config))
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
            Ok(body) => self.body(body).content_type("application/json"),
            Err(_) => {
                self.body = None;
                self.header_remove("Content-Type")
            }
        }
    }

    pub fn set_nodelay(mut self, nodelay: bool) -> ClientBuilder {
        self.config.nodelay = nodelay;
        self
    }

    pub fn timeout(mut self, timeout: Duration) -> ClientBuilder {
        self.config.timeout = Some(timeout);
        self
    }

    pub fn connect_timeout(mut self, timeout: Duration) -> ClientBuilder {
        self.config.connect_timeout = Some(timeout);
        self
    }

    pub fn max_redirects(mut self, max_redirects: usize) -> ClientBuilder {
        self.config.max_redirects = max_redirects;
        self
    }

    pub fn referer<U>(self, value: U) -> ClientBuilder
    where
        U: IntoUrl,
    {
        match value.into_url() {
            Ok(url) => self.header("Referer", &url),
            _ => self,
        }
    }

    pub fn origin<U>(self, value: U) -> ClientBuilder
    where
        U: IntoUrl,
    {
        match value.into_url() {
            Ok(url) => self.header("Origin", &url),
            _ => self,
        }
    }

    pub fn content_type<S>(self, value: &S) -> ClientBuilder
    where
        S: ToString + ?Sized,
    {
        self.header("Content-Type", value)
    }
}

#[cfg(test)]
mod tests {
    // use super::*;

    // const HTTPBIN: &str = "https://httpbin.org/";
    // const ACCEPT: &str = "accept";
    // const ACCEPT_JSON: &str = "application/json";

    // #[tokio::test]
    // async fn delete_client() {
    //     let url = format!("{}{}", HTTPBIN, "delete");
    //     let mut client = delete(&url)
    //         .unwrap()
    //         .header(ACCEPT, ACCEPT_JSON)
    //         .build()
    //         .await
    //         .unwrap();
    //     let response = client.send().await.unwrap();
    //     let body = response.body();
    //     assert!(!body.is_empty());
    //     assert_eq!(
    //         Some("application/json".to_string()),
    //         response.header("content-type")
    //     );
    // }

    // #[tokio::test]
    // async fn get_client() {
    //     let url = format!("{}{}", HTTPBIN, "get");
    //     let client_builder = get(&url).unwrap().header(ACCEPT, ACCEPT_JSON);
    //     let mut client = client_builder.build().await.unwrap();
    //     let response = client.send().await.unwrap();
    //     let body = response.body();
    //     assert!(!body.is_empty());
    //     assert_eq!(
    //         Some("application/json".to_string()),
    //         response.header("content-type")
    //     );
    // }
}
