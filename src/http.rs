use tokio::net::TcpStream;
use native_tls::TlsConnector;
use uri::Uri;

use crate::error::Result;
// use crate::response::Response;
use crate::stream::MaybeHttpsStream;

#[derive(Debug)]
pub struct HttpStream {
    stream: MaybeHttpsStream,
}

impl HttpStream {
    pub async fn connect(uri: &Uri) -> Result<Self> {
        let target = uri.socket_addr()?;
        let stream = TcpStream::connect(target).await?;
        let stream = if uri.is_ssl() {
            let cx = TlsConnector::builder().build()?;
            let cx = tokio_tls::TlsConnector::from(cx);
            let stream = cx.connect(&uri.origin(), stream).await?;
            MaybeHttpsStream::from(stream)
        } else {
            MaybeHttpsStream::from(stream)
        };
        Ok(HttpStream { stream })
    }

    pub async fn connect_proxy(uri: &Uri) -> Result<Self> {
        let target = uri.socket_addr()?;
        let stream = TcpStream::connect(target).await?;
        let stream = MaybeHttpsStream::from(stream);
        Ok(HttpStream { stream })
    }

    // pub fn send_request(&mut self, req: &[u8]) -> Result<()> {
    //     Stream::send_msg(&mut self.stream, req)
    // }

    // pub fn get_response(&mut self) -> Result<Response> {
    //     Stream::read_head(&mut self.stream)
    // }

    // pub fn get_body(&mut self, content_len: usize) -> Result<Vec<u8>> {
    //     Stream::get_body(&mut self.stream, content_len)
    // }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tokio::io::{AsyncWriteExt, AsyncReadExt};


    #[tokio::test]
    async fn http_stream_http() {
        let mut client =
            HttpStream::connect(&"http://api.ipify.org".parse::<Uri>().unwrap()).await.unwrap();
        client.stream.write_all(b"GET / HTTP/1.0\r\nHost: api.ipify.org\r\n\r\n").await.unwrap();
        client.stream.flush().await.unwrap();
        let mut buf = Vec::new();
        client.stream.read_to_end(&mut buf).await.unwrap();
        let body = String::from_utf8(buf).unwrap();
        // let split: Vec<&str> = body.splitn(2, "\r\n\r\n").collect();
        // split[1].to_string()
        // let response = client.get_response().unwrap();
        // let body = client.get_body(response.content_len().unwrap()).unwrap();
        // let body = String::from_utf8(body).unwrap();
        assert!(&body.contains(crate::tests::IP.as_str()));
    }

    // #[test]
    // fn http_stream_https() {
    //     let mut client =
    //         HttpStream::connect(&"https://api.ipify.org".parse::<Uri>().unwrap()).unwrap();
    //     client
    //         .send_request(b"GET / HTTP/1.0\r\nHost: api.ipify.org\r\n\r\n")
    //         .unwrap();
    //     let response = client.get_response().unwrap();
    //     let body = client.get_body(response.content_len().unwrap()).unwrap();
    //     let body = String::from_utf8(body).unwrap();
    //     assert!(&body.contains(crate::tests::IP.as_str()));
    // }

    // #[test]
    // fn http_stream_http_proxy() {
    //     let mut client =
    //         HttpStream::connect_proxy(&"http://127.0.0.1:5858".parse::<Uri>().unwrap()).unwrap();
    //     client
    //         .send_request(b"GET http://api.ipify.org/ HTTP/1.0\r\nHost: api.ipify.org\r\n\r\n")
    //         .unwrap();
    //     let response = client.get_response().unwrap();
    //     let body = client.get_body(response.content_len().unwrap()).unwrap();
    //     let body = String::from_utf8(body).unwrap();
    //     assert!(&body.contains(crate::tests::IP.as_str()));
    // }

    // #[test]
    // fn http_stream_auth_http_proxy() {
    //     let mut client =
    //         HttpStream::connect_proxy(&"http://test:tset@127.0.0.1:5656".parse::<Uri>().unwrap())
    //             .unwrap();
    //     client
    //         .send_request(b"GET http://api.ipify.org/ HTTP/1.0\r\nHost: api.ipify.org\r\nProxy-Authorization: Basic dGVzdDp0c2V0\r\n\r\n")
    //         .unwrap();
    //     let response = client.get_response().unwrap();
    //     let body = client.get_body(response.content_len().unwrap()).unwrap();
    //     let body = String::from_utf8(body).unwrap();
    //     assert!(&body.contains(crate::tests::IP.as_str()));
    // }
}
