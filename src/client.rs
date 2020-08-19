use uri::Uri;

use crate::client_builder::ClientBuilder;
use crate::error::{Error, Result};
use crate::request::Request;
use crate::response::Response;
use crate::stream::MaybeHttpsStream;

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

    pub fn request(&self) -> Request {
        self.request.clone()
    }

    pub async fn send_request(&mut self) -> Result<()> {
        self.stream.send_msg(&self.request.to_vec()).await
    }

    pub async fn send(&mut self) -> Result<Response> {
        self.send_request().await?;
        let response = self.stream.get_response().await?;
        self.response = Some(response.clone());
        Ok(response)
    }

    fn content_len(&self) -> Result<usize> {
        if let Some(response) = &self.response {
            response.content_len()
        } else {
            Err(Error::EmptyResponse)
        }
    }

    pub async fn get_body(&mut self) -> Result<Vec<u8>> {
        let content_len = self.content_len()?;
        self.stream.get_body(content_len).await
    }

    pub async fn text(&mut self) -> Result<String> {
        let body = self.get_body().await?;
        Ok(String::from_utf8_lossy(&body).to_string())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn client_http() {
        let mut client = ClientBuilder::new().get("http://api.ipify.org").build().await.unwrap();
        let response = client.send().await.unwrap();
        assert!(response.status_code().is_success());
        let body = client.text().await.unwrap();
        assert!(&body.contains(crate::tests::IP.as_str()));
    }

    #[tokio::test]
    async fn client_https() {
        let mut client = ClientBuilder::new().get("https://api.ipify.org").build().await.unwrap();
        let response = client.send().await.unwrap();
        assert!(response.status_code().is_success());
        let body = client.text().await.unwrap();
        assert!(&body.contains(crate::tests::IP.as_str()));
    }

    #[tokio::test]
    async fn client_http_proxy() {
        let mut client = ClientBuilder::new().get("http://api.ipify.org")
            .proxy("http://127.0.0.1:5858")
            .build()
            .await
            .unwrap();
        let response = client.send().await.unwrap();
        assert!(response.status_code().is_success());
        let body = client.text().await.unwrap();
        assert!(&body.contains(crate::tests::IP.as_str()));
    }

    #[tokio::test]
    async fn client_http_proxy_auth() {
        let mut client = ClientBuilder::new().get("http://api.ipify.org")
            .proxy("http://test:tset@127.0.0.1:5656")
            .build()
            .await
            .unwrap();
        let response = client.send().await.unwrap();
        assert!(response.status_code().is_success());
        let body = client.text().await.unwrap();
        assert!(&body.contains(crate::tests::IP.as_str()));
    }

    #[tokio::test]
    async fn client_http_proxy_auth_err() {
        let mut client = ClientBuilder::new().get("http://api.ipify.org")
            .proxy("http://127.0.0.1:5656")
            .build()
            .await
            .unwrap();
        let response = client.send().await.unwrap();
        assert!(!response.status_code().is_success());
    }

    #[tokio::test]
    async fn client_socks_proxy() {
        let mut client = ClientBuilder::new().get("http://api.ipify.org")
            .proxy("socks5://127.0.0.1:5959")
            .build()
            .await
            .unwrap();
        let response = client.send().await.unwrap();
        assert!(response.status_code().is_success());
        let body = client.text().await.unwrap();
        assert!(&body.contains(crate::tests::IP.as_str()));
    }

    #[tokio::test]
    async fn client_socks_proxy_auth() {
        let mut client = ClientBuilder::new().get("https://api.ipify.org")
            .proxy("socks5://test:tset@127.0.0.1:5757")
            .build()
            .await
            .unwrap();
        let response = client.send().await.unwrap();
        assert!(response.status_code().is_success());
        let body = client.text().await.unwrap();
        assert!(&body.contains(crate::tests::IP.as_str()));
    }

    #[tokio::test]
    async fn client_socks_proxy_auth_err() {
        let client = ClientBuilder::new().get("http://api.ipify.org")
            .proxy("socks5://t:t@127.0.0.1:5757")
            .build()
            .await;
        assert!(client.is_err());
    }
}
