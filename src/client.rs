use bytes::Bytes;
use uri::Uri;

use crate::client_builder::ClientBuilder;
use crate::error::Result;
use crate::request::Request;
use crate::response::Response;
use crate::stream::MaybeHttpsStream;
use crate::headers::Headers;

#[derive(Debug)]
pub struct Client {
    request: Request,
    uri: Uri,
    proxy: Option<Uri>,
    stream: MaybeHttpsStream,
    response: Option<Response>,
}

impl Client {
    pub fn builder() -> ClientBuilder {
        ClientBuilder::default()
    }

    pub fn new(
        request: Request,
        uri: Uri,
        proxy: Option<Uri>,
        stream: MaybeHttpsStream,
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

    pub async fn send(&mut self) -> Result<Response> {
        self.stream.send_msg(&self.request.to_vec()).await?;
        let response = self.stream.get_response().await?;
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
    const SECURE_URL: &'static str = "https://api.ipify.org";

    #[tokio::test]
    async fn client_http() {
        let mut client = Client::builder()
            .get(SIMPLE_URL)
            .build()
            .await
            .unwrap();
        let response = client.send().await.unwrap();
        assert!(response.status_code().is_success());
        let body = response.text().unwrap();
        assert!(&body.contains(crate::tests::IP.as_str()));
    }

    #[tokio::test]
    async fn client_https() {
        let mut client = Client::builder()
            .get(SECURE_URL)
            .build()
            .await
            .unwrap();
        let response = client.send().await.unwrap();
        assert!(response.status_code().is_success());
        let body = response.text().unwrap();
        assert!(&body.contains(crate::tests::IP.as_str()));
    }

    #[tokio::test]
    async fn client_http_proxy() {
        dotenv::dotenv().ok();
        if let Ok(http_proxy) = dotenv::var("HTTP_PROXY") {
            let mut client = Client::builder()
                .get(SIMPLE_URL)
                .proxy(&http_proxy)
                .build()
                .await
                .unwrap();
            let request = client.request();
            assert_eq!(&request.request_uri(), "GET http://api.ipify.org/ HTTP/1.0\r\n");
            let response = client.send().await.unwrap();
            assert!(response.status_code().is_success());
            let body = response.text().unwrap();
            assert!(&body.contains(crate::tests::IP.as_str()));
        }
    }

    #[tokio::test]
    async fn client_http_proxy_auth() {
        dotenv::dotenv().ok();
        if let Ok(http_auth_proxy) = dotenv::var("HTTP_AUTH_PROXY") {
            let mut client = Client::builder()
                .get(SIMPLE_URL)
                .proxy(&http_auth_proxy)
                .build()
                .await
                .unwrap();
            let response = client.send().await.unwrap();
            assert!(response.status_code().is_success());
            let body = response.text().unwrap();
            assert!(&body.contains(crate::tests::IP.as_str()));
        }
    }

    #[tokio::test]
    async fn client_http_proxy_auth_err() {
        dotenv::dotenv().ok();
        if let Ok(http_auth_proxy) = dotenv::var("HTTP_AUTH_PROXY") {
            let mut client = Client::builder()
                .get(SIMPLE_URL)
                .proxy(&http_auth_proxy)
                .build()
                .await
                .unwrap();
            let response = client.send().await.unwrap();
            assert!(!response.status_code().is_success());
        }
    }

    #[tokio::test]
    async fn client_socks_proxy() {
        dotenv::dotenv().ok();
        if let Ok(socks5_proxy) = dotenv::var("SOCKS5_PROXY") {
            let mut client = Client::builder()
                .get(SIMPLE_URL)
                .proxy(&socks5_proxy)
                .build()
                .await
                .unwrap();
            let response = client.send().await.unwrap();
            assert!(response.status_code().is_success());
            let body = response.text().unwrap();
            assert!(&body.contains(crate::tests::IP.as_str()));
        }
    }

    #[tokio::test]
    async fn client_socks_proxy_auth() {
        dotenv::dotenv().ok();
        if let Ok(socks5_auth_proxy) = dotenv::var("SOCKS5_AUTH_PROXY") {
            let mut client = Client::builder()
                .get(SIMPLE_URL)
                .proxy(&socks5_auth_proxy)
                .build()
                .await
                .unwrap();
            let response = client.send().await.unwrap();
            assert!(response.status_code().is_success());
            let body = response.text().unwrap();
            assert!(&body.contains(crate::tests::IP.as_str()));
        }
    }

    #[tokio::test]
    async fn client_socks_proxy_auth_err() {
        dotenv::dotenv().ok();
        if let Ok(socks5_auth_proxy) = dotenv::var("SOCKS5_AUTH_PROXY") {
            let client = Client::builder()
                .get(SIMPLE_URL)
                .proxy(&socks5_auth_proxy)
                .build()
                .await;
            assert!(client.is_err());
        }
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
