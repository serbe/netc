use uri::Uri;

use crate::client_builder::ClientBuilder;
use crate::error::{Error, Result};
use crate::request::Request;
use crate::response::Response;
use crate::transport::Transport;

#[derive(Debug)]
pub struct Client {
    request: Request,
    uri: Uri,
    transport: Transport,
    response: Option<Response>,
}

impl Client {
    pub fn new(uri: &str) -> ClientBuilder {
        ClientBuilder::new().uri(uri)
    }

    pub fn builder() -> ClientBuilder {
        ClientBuilder::new()
    }

    pub fn from(
        request: Request,
        uri: Uri,
        transport: Transport,
        response: Option<Response>,
    ) -> Client {
        Client {
            request,
            uri,
            transport,
            response,
        }
    }

    pub fn request(&self) -> Request {
        self.request.clone()
    }

    pub fn send_request(&mut self) -> Result<()> {
        match self.transport {
            Transport::Proxy(ref mut proxy) => proxy.send_request(&self.request.to_vec()),
            Transport::Stream(ref mut stream) => stream.send_request(&self.request.to_vec()),
            Transport::None => Err(Error::WrongHttp),
        }
    }

    pub fn send(&mut self) -> Result<Response> {
        self.send_request()?;
        let response = match self.transport {
            Transport::Proxy(ref mut proxy) => proxy.get_response(),
            Transport::Stream(ref mut stream) => stream.get_response(),
            Transport::None => return Err(Error::WrongHttp),
        }?;
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

    pub fn get_body(&mut self) -> Result<Vec<u8>> {
        let content_len = self.content_len()?;
        match self.transport {
            Transport::Proxy(ref mut proxy) => proxy.get_body(content_len),
            Transport::Stream(ref mut stream) => stream.get_body(content_len),
            Transport::None => Err(Error::WrongHttp),
        }
    }

    pub fn text(&mut self) -> Result<String> {
        let body = self.get_body()?;
        Ok(String::from_utf8_lossy(&body).to_string())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn client_http() {
        let mut client = Client::new("http://api.ipify.org").build().unwrap();
        let response = client.send().unwrap();
        assert!(response.status_code().is_success());
        let body = client.text().unwrap();
        assert!(&body.contains(crate::tests::IP.as_str()));
    }

    #[test]
    fn client_https() {
        let mut client = Client::new("https://api.ipify.org").build().unwrap();
        let response = client.send().unwrap();
        assert!(response.status_code().is_success());
        let body = client.text().unwrap();
        assert!(&body.contains(crate::tests::IP.as_str()));
    }

    #[test]
    fn client_http_proxy() {
        let mut client = Client::new("http://api.ipify.org")
            .proxy("http://127.0.0.1:5858")
            .build()
            .unwrap();
        let response = client.send().unwrap();
        assert!(response.status_code().is_success());
        let body = client.text().unwrap();
        assert!(&body.contains(crate::tests::IP.as_str()));
    }

    #[test]
    fn client_http_proxy_auth() {
        let mut client = Client::new("http://api.ipify.org")
            .proxy("http://test:tset@127.0.0.1:5656")
            .build()
            .unwrap();
        let response = client.send().unwrap();
        assert!(response.status_code().is_success());
        let body = client.text().unwrap();
        assert!(&body.contains(crate::tests::IP.as_str()));
    }

    #[test]
    fn client_http_proxy_auth_err() {
        let mut client = Client::new("http://api.ipify.org")
            .proxy("http://test:test@127.0.0.1:5656")
            .build()
            .unwrap();
        let response = client.send().unwrap();
        assert!(!response.status_code().is_success());
    }

    #[test]
    fn client_socks_proxy() {
        let mut client = Client::new("http://api.ipify.org")
            .proxy("socks5://127.0.0.1:5959")
            .build()
            .unwrap();
        let response = client.send().unwrap();
        assert!(response.status_code().is_success());
        let body = client.text().unwrap();
        assert!(&body.contains(crate::tests::IP.as_str()));
    }

    #[test]
    fn client_socks_proxy_auth() {
        let mut client = Client::new("http://api.ipify.org")
            .proxy("socks5://test:tset@127.0.0.1:5757")
            .build()
            .unwrap();
        let response = client.send().unwrap();
        assert!(response.status_code().is_success());
        let body = client.text().unwrap();
        assert!(&body.contains(crate::tests::IP.as_str()));
    }

    #[test]
    fn client_socks_proxy_auth_err() {
        let client = Client::new("http://api.ipify.org")
            .proxy("socks5://test:test@127.0.0.1:5757")
            .build();
        assert!(client.is_err());
    }
}
