use std::{convert::TryInto, time::Duration};

use bytes::Bytes;
use uri::{IntoUri, Uri};

use crate::{Client, Error, Headers, HttpStream, Method, Request, Version};

#[derive(Debug, PartialEq)]
pub struct ClientBuilder {
    pub(crate) uri: Option<Uri>,
    pub(crate) headers: Headers,
    pub(crate) method: Method,
    pub(crate) version: Version,
    pub(crate) body: Option<Bytes>,
    pub(crate) proxy: Option<Uri>,
    pub(crate) nodelay: bool,
    pub(crate) timeout: Option<Duration>,
    pub(crate) connect_timeout: Option<Duration>,
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
            method: Method::Get,
            version: Version::Http11,
            body: None,
            proxy: None,
            nodelay: false,
            timeout: None,
            connect_timeout: None,
        }
    }

    pub async fn build(self) -> Result<Client, Error> {
        let uri = self.uri.ok_or(Error::EmptyUri)?;
        let mut request = Request::new(Method::Get, &uri);
        request.proxy(self.proxy.as_ref());
        request.headers(self.headers);
        request.method(self.method);
        request.version(self.version);
        request.opt_body(self.body);
        let stream = HttpStream::from_request(&request).await?;
        Ok(Client::new(request, stream, None))
    }

    pub fn uri<U: IntoUri>(mut self, value: U) -> ClientBuilder {
        match value.into_uri() {
            Ok(uri) => self.uri = Some(uri),
            _ => self.uri = None,
        }
        self
    }

    pub fn proxy<P: IntoUri>(mut self, value: P) -> ClientBuilder {
        match value.into_uri() {
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
        if let Ok(method) = value.try_into() {
            self.method = method
        }
        self
    }

    pub fn get<U: IntoUri>(mut self, value: U) -> ClientBuilder {
        match value.into_uri() {
            Ok(uri) => self.uri = Some(uri),
            _ => self.uri = None,
        }
        self.method = Method::Get;
        self
    }

    pub fn post<U: IntoUri>(mut self, value: U) -> ClientBuilder {
        match value.into_uri() {
            Ok(uri) => self.uri = Some(uri),
            _ => self.uri = None,
        }
        self.method = Method::Post;
        self
    }

    pub fn options<U: IntoUri>(mut self, value: U) -> ClientBuilder {
        match value.into_uri() {
            Ok(uri) => self.uri = Some(uri),
            _ => self.uri = None,
        }
        self.method = Method::Options;
        self
    }

    pub fn delete<U: IntoUri>(mut self, value: U) -> ClientBuilder {
        match value.into_uri() {
            Ok(uri) => self.uri = Some(uri),
            _ => self.uri = None,
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
        U: TryInto<Uri>,
    {
        match value.try_into() {
            Ok(uri) => self.header("Referer", &uri),
            _ => self,
        }
    }
}

pub fn delete<U: IntoUri>(uri: U) -> Result<ClientBuilder, Error> {
    Ok(ClientBuilder::new().delete(uri.into_uri()?))
}

pub fn get<U: IntoUri>(uri: U) -> Result<ClientBuilder, Error> {
    Ok(ClientBuilder::new().get(uri.into_uri()?))
}

pub fn post<U: IntoUri>(uri: U) -> Result<ClientBuilder, Error> {
    Ok(ClientBuilder::new().post(uri.into_uri()?))
}

#[cfg(test)]
mod tests {
    use super::*;

    const HTTPBIN: &str = "https://httpbin.org/";
    const ACCEPT: &str = "accept";
    const ACCEPT_JSON: &str = "application/json";

    #[tokio::test]
    async fn delete_client() {
        let uri = format!("{}{}", HTTPBIN, "delete");
        dbg!(&uri);
        let mut client = delete(uri)
            .unwrap()
            .header(ACCEPT, ACCEPT_JSON)
            .build()
            .await
            .unwrap();
        let response = client.send().await.unwrap();
        let body = response.body();
        assert!(!body.is_empty());
        assert_eq!(
            Some("application/json".to_string()),
            response.header("content-type")
        );
    }

    #[tokio::test]
    async fn get_client() {
        let uri = format!("{}{}", HTTPBIN, "get");
        dbg!(&uri);
        let client_builder = get(uri).unwrap().header(ACCEPT, ACCEPT_JSON);
        let mut client = client_builder.build().await.unwrap();
        let response = client.send().await.unwrap();
        let body = response.body();
        assert!(!body.is_empty());
        assert_eq!(
            Some("application/json".to_string()),
            response.header("content-type")
        );
    }
}
