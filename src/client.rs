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
    use dotenv;

    // #[tokio::test]
    // async fn client_http() {
    //     let mut client = Client::builder()
    //         .get("http://api.ipify.org")
    //         .build()
    //         .await
    //         .unwrap();
    //     let response = client.send().await.unwrap();
    //     assert!(response.status_code().is_success());
    //     let body = client.text().await.unwrap();
    //     assert!(&body.contains(crate::tests::IP.as_str()));
    // }

    // #[tokio::test]
    // async fn client_https() {
    //     let mut client = Client::builder()
    //         .get("https://api.ipify.org")
    //         .build()
    //         .await
    //         .unwrap();
    //     let response = client.send().await.unwrap();
    //     assert!(response.status_code().is_success());
    //     let body = client.text().await.unwrap();
    //     assert!(&body.contains(crate::tests::IP.as_str()));
    // }

    #[tokio::test]
    async fn client_http_proxy() {
        dotenv::dotenv().ok();
        if let Ok(http_proxy) = dotenv::var("HTTP_PROXY") {
            let mut client = Client::builder()
                .get("http://api.ipify.org")
                .proxy(&http_proxy)
                .build()
                .await
                .unwrap();
            let response = client.send().await.unwrap();
            assert!(response.status_code().is_success());
            let body = client.text().await.unwrap();
            assert!(&body.contains(crate::tests::IP.as_str()));
        }
    }

    #[tokio::test]
    async fn client_http_proxy_auth() {
        dotenv::dotenv().ok();
        if let Ok(http_auth_proxy) = dotenv::var("HTTP_AUTH_PROXY") {
            let mut client = Client::builder()
                .get("http://api.ipify.org")
                .proxy(&http_auth_proxy)
                .build()
                .await
                .unwrap();
            let response = client.send().await.unwrap();
            assert!(response.status_code().is_success());
            let body = client.text().await.unwrap();
            assert!(&body.contains(crate::tests::IP.as_str()));
        }
    }

    #[tokio::test]
    async fn client_http_proxy_auth_err() {
        dotenv::dotenv().ok();
        if let Ok(http_auth_proxy) = dotenv::var("HTTP_AUTH_PROXY") {
            let mut client = Client::builder()
                .get("http://api.ipify.org")
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
                .get("http://api.ipify.org")
                .proxy(&socks5_proxy)
                .build()
                .await
                .unwrap();
            let response = client.send().await.unwrap();
            assert!(response.status_code().is_success());
            let body = client.text().await.unwrap();
            assert!(&body.contains(crate::tests::IP.as_str()));
        }
    }

    #[tokio::test]
    async fn client_socks_proxy_auth() {
        dotenv::dotenv().ok();
        if let Ok(socks5_auth_proxy) = dotenv::var("SOCKS5_AUTH_PROXY") {
            let mut client = Client::builder()
                .get("https://api.ipify.org")
                .proxy(&socks5_auth_proxy)
                .build()
                .await
                .unwrap();
            let response = client.send().await.unwrap();
            assert!(response.status_code().is_success());
            let body = client.text().await.unwrap();
            assert!(&body.contains(crate::tests::IP.as_str()));
        }
    }

    #[tokio::test]
    async fn client_socks_proxy_auth_err() {
        dotenv::dotenv().ok();
        if let Ok(socks5_auth_proxy) = dotenv::var("SOCKS5_AUTH_PROXY") {
            let client = Client::builder()
                .get("http://api.ipify.org")
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
        // let client = Client::builder().get("http://api.ipify.org").build().await;
        // assert!(client.is_ok());
        // let client = client.unwrap();
        // assert_eq!(client.content_len(), Err(Error::EmptyResponse));
        // let client = Client::builder().get("http://api.ipify.org").body(b"2020").build().await.unwrap();
        // assert_eq!(dbg!(client.content_len()), dbg!(Ok(4)));
    }
}
