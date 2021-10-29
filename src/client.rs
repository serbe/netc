use bytes::Bytes;
use uri::Uri;

use crate::{ClientBuilder, Error, Headers, HttpStream, Request, Response};

#[derive(Debug)]
pub struct Client {
    pub(crate) request: Request,
    pub(crate) uri: Uri,
    pub(crate) proxy: Option<Uri>,
    pub(crate) stream: HttpStream,
    pub(crate) response: Option<Response>,
}

impl Client {
    pub fn builder() -> ClientBuilder {
        ClientBuilder::default()
    }

    pub fn new(
        request: Request,
        uri: Uri,
        proxy: Option<Uri>,
        stream: HttpStream,
        response: Option<Response>,
    ) -> Client {
        Client {
            request,
            uri,
            proxy,
            stream,
            response,
        }
    }

    pub async fn send(&mut self) -> Result<Response, Error> {
        self.stream.send_msg(&self.request.to_vec()).await?;
        let mut response = self.stream.get_response().await?;
        response.method = self.request.method.clone();
        self.response = Some(response.clone());
        Ok(response)
    }

    pub fn content_length(&self) -> usize {
        self.request.content_length()
    }

    pub fn body(&self) -> Option<Bytes> {
        self.request.get_body()
    }

    pub fn headers(&self) -> Headers {
        self.request.get_headers()
    }

    pub fn uri(&self) -> Uri {
        self.uri.clone()
    }

    pub fn request(&self) -> Request {
        self.request.clone()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use dotenv;

    const SIMPLE_URL: &'static str = "http://api.ipify.org";
    const SECURE_URL: &'static str = "https://www.socks-proxy.net";

    #[tokio::test]
    async fn client_http() {
        let mut client = Client::builder().get(SIMPLE_URL).build().await.unwrap();
        let response = client.send().await.unwrap();
        assert!(response.status_code().is_success());
        let body = response.text().unwrap();
        assert!(&body.contains(crate::tests::IP.as_str()));
    }

    #[tokio::test]
    async fn client_https() {
        let mut client = Client::builder().get(SECURE_URL).build().await.unwrap();
        let response = client.send().await.unwrap();
        assert!(response.status_code().is_success());
        let body = response.text().unwrap();
        assert!(&body.contains(crate::tests::IP.as_str()));
    }

    #[tokio::test]
    async fn client_http_proxy() {
        dotenv::dotenv().ok();
        let http_proxy = match dotenv::var("TEST_HTTP_PROXY") {
            Ok(it) => it,
            _ => return,
        };
        let mut client = Client::builder()
            .get(SIMPLE_URL)
            .proxy(http_proxy)
            .build()
            .await
            .unwrap();
        let request = client.request();
        assert_eq!(&request.request_uri(), "http://api.ipify.org/");
        let response = client.send().await.unwrap();
        assert!(response.status_code().is_success());
        let body = response.text().unwrap();
        assert!(&body.contains(crate::tests::IP.as_str()));
    }

    #[tokio::test]
    async fn client_http_proxy_auth() {
        dotenv::dotenv().ok();
        let http_auth_proxy = match dotenv::var("TEST_HTTP_AUTH_PROXY") {
            Ok(it) => it,
            _ => return,
        };
        let mut client = Client::builder()
            .get(SIMPLE_URL)
            .proxy(http_auth_proxy)
            .build()
            .await
            .unwrap();
        let response = client.send().await.unwrap();
        assert!(response.status_code().is_success());
        let body = response.text().unwrap();
        assert!(&body.contains(crate::tests::IP.as_str()));
    }

    #[tokio::test]
    async fn client_http_proxy_auth_err() {
        dotenv::dotenv().ok();
        let http_auth_proxy = match dotenv::var("TEST_HTTP_AUTH_ERR_PROXY") {
            Ok(it) => it,
            _ => return,
        };
        let mut client = Client::builder()
            .get(SIMPLE_URL)
            .proxy(http_auth_proxy)
            .build()
            .await
            .unwrap();
        let response = client.send().await.unwrap();
        assert!(response.status_code().is_client_err());
    }

    #[tokio::test]
    async fn client_socks_proxy() {
        dotenv::dotenv().ok();
        let socks5_proxy = match dotenv::var("TEST_SOCKS5_PROXY") {
            Ok(it) => it,
            _ => return,
        };
        let mut client = Client::builder()
            .get(SIMPLE_URL)
            .proxy(socks5_proxy)
            .build()
            .await
            .unwrap();
        let response = client.send().await.unwrap();
        assert!(response.status_code().is_success());
        let body = response.text().unwrap();
        assert!(&body.contains(crate::tests::IP.as_str()));
    }

    #[tokio::test]
    async fn client_socks_proxy_auth() {
        dotenv::dotenv().ok();
        let socks5_auth_proxy = match dotenv::var("TEST_SOCKS5_AUTH_PROXY") {
            Ok(it) => it,
            _ => return,
        };
        let mut client = Client::builder()
            .get(SIMPLE_URL)
            .proxy(socks5_auth_proxy)
            .build()
            .await
            .unwrap();
        let response = client.send().await.unwrap();
        assert!(response.status_code().is_success());
        let body = response.text().unwrap();
        assert!(&body.contains(crate::tests::IP.as_str()));
    }

    #[tokio::test]
    async fn client_socks_proxy_auth_err() {
        dotenv::dotenv().ok();
        let socks5_auth_proxy = match dotenv::var("TEST_SOCKS5_AUTH_ERR_PROXY") {
            Ok(it) => it,
            _ => return,
        };
        let client = Client::builder()
            .get(SIMPLE_URL)
            .proxy(socks5_auth_proxy)
            .build()
            .await;
        assert!(client.is_err());
    }

    #[test]
    fn client_builder() {
        let client_builder = Client::builder();
        assert_eq!(client_builder, ClientBuilder::new());
    }

    #[tokio::test]
    async fn client_content_len() {
        let client = Client::builder().build().await;
        assert!(client.is_err());
        let client = Client::builder().get(SIMPLE_URL).build().await;
        assert!(client.is_ok());
    }
}
