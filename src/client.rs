use bytes::Bytes;
use futures::{future::BoxFuture, FutureExt};
use url::Url;

use crate::{client_builder::Config, ClientBuilder, Error, Headers, HttpStream, Request, Response};

#[derive(Debug)]
pub struct Client {
    pub(crate) request: Request,
    pub(crate) stream: HttpStream,
    pub(crate) response: Option<Response>,
    pub(crate) config: Config,
}

impl Client {
    pub fn builder() -> ClientBuilder {
        ClientBuilder::default()
    }

    pub fn new(
        request: Request,
        stream: HttpStream,
        response: Option<Response>,
        config: Config,
    ) -> Client {
        Client {
            request,
            stream,
            response,
            config,
        }
    }

    pub fn send(&mut self) -> BoxFuture<'_, Result<Response, Error>> {
        async {
            self.stream.send_msg(&self.request.to_vec()).await?;
            let mut response = self.stream.get_response().await?;
            response.method = self.request.method.clone();
            if response.status_code().is_redirect() {
                if let Some(location) = response.headers().get("Location") {
                    let redirect_url = if let Ok(new_url) = Url::parse(&location) {
                        new_url
                    } else {
                        let mut current_url = self.request().url();
                        current_url.set_path(&location);
                        current_url
                    };
                    self.redirect()?;
                    return ClientBuilder::from_client(self)
                        .url(&redirect_url)
                        .build()
                        .await?
                        .send()
                        .await;
                }
            };
            self.response = Some(response.clone());
            Ok(response)
        }
        .boxed()
    }

    pub fn body(&self) -> Option<Bytes> {
        self.request.get_body()
    }

    pub fn headers(&self) -> &Headers {
        self.request.get_headers()
    }

    pub fn request(&self) -> Request {
        self.request.clone()
    }

    pub fn redirects(&self) -> usize {
        self.config.redirects
    }

    pub(crate) fn redirect(&mut self) -> Result<(), Error> {
        self.config.redirects += 1;
        if self.config.redirects >= self.config.max_redirects {
            Err(Error::MaxRedirects)
        } else {
            Ok(())
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const SIMPLE_URL: &'static str = "http://api.ipify.org";
    const SECURE_URL: &'static str = "https://api.ipify.org";

    #[tokio::test]
    async fn client_https() {
        let mut client = Client::builder().get(SECURE_URL).build().await.unwrap();
        let response = client.send().await.unwrap();
        assert!(response.status_code().is_success());
        let body = response.text().unwrap();
        assert!(&body.contains(crate::tests::IP.as_str()));
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
