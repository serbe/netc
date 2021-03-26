use std::{
    fmt, io,
    pin::Pin,
    sync::Arc,
    task::{Context, Poll},
};

use bytes::Bytes;
use rsl::socks5;
use tokio::io::{AsyncRead, AsyncReadExt, AsyncWrite, AsyncWriteExt, ReadBuf};
use tokio::net::TcpStream;
use tokio_rustls::{client::TlsStream, rustls::ClientConfig, webpki::DNSNameRef, TlsConnector};
use url::Url;

use crate::error::Error;
use crate::response::Response;
use crate::utils::IntoUrl;

pub enum MaybeHttpsStream {
    Http(TcpStream),
    Https(Box<TlsStream<TcpStream>>),
}

impl MaybeHttpsStream {
    pub async fn new<U: IntoUrl>(value: U) -> Result<Self, Error> {
        let url = value.into_url()?;
        let socket_address = url.socket_addrs(|| None)?;
        let stream =
            TcpStream::connect(socket_address.get(0).map_or(Err(Error::SocketAddr), Ok)?).await?;
        MaybeHttpsStream::maybe_ssl(&url, stream).await
    }

    pub async fn socks(proxy: &Url, target: &Url) -> Result<Self, Error> {
        let stream = socks5::connect_url(proxy, target).await?;
        MaybeHttpsStream::maybe_ssl(target, stream).await
    }

    async fn maybe_ssl(url: &Url, stream: TcpStream) -> Result<Self, Error> {
        if url.scheme() == "https" {
            let mut config = ClientConfig::new();
            config
                .root_store
                .add_server_trust_anchors(&webpki_roots::TLS_SERVER_ROOTS);
            let connector = TlsConnector::from(Arc::new(config));
            let dns_name = DNSNameRef::try_from_ascii_str(url.host_str().ok_or(Error::EmptyHost)?)?;
            let stream = connector.connect(dns_name, stream).await?;
            Ok(MaybeHttpsStream::from(stream))
        } else {
            Ok(MaybeHttpsStream::from(stream))
        }
    }

    pub async fn get_body(&mut self, content_len: usize) -> Result<Bytes, Error> {
        let mut body = vec![0u8; content_len];
        self.read_exact(&mut body).await?;
        Ok(body.into())
    }

    pub async fn get_response(&mut self) -> Result<Response, Error> {
        let mut header = Vec::with_capacity(512);
        while !(header.len() > 4 && header[header.len() - 4..] == b"\r\n\r\n"[..]) {
            header.push(self.read_u8().await.or(Err(Error::HeaderIncomplete))?);
            if header.len() > 1024 {
                return Err(Error::HeaderToBig);
            }
        }
        let response = Response::from_header(&header)?;
        let content_len = response.content_len()?;
        let body = self.get_body(content_len).await?;
        Ok(Response {
            status: response.status,
            headers: response.headers,
            body,
        })
    }

    pub async fn send_msg(&mut self, msg: &[u8]) -> Result<(), Error> {
        self.write_all(msg).await?;
        self.flush().await?;
        Ok(())
    }
}

impl fmt::Debug for MaybeHttpsStream {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            MaybeHttpsStream::Http(s) => f.debug_tuple("Http").field(s).finish(),
            MaybeHttpsStream::Https(s) => f.debug_tuple("Https").field(s).finish(),
        }
    }
}

impl From<TcpStream> for MaybeHttpsStream {
    fn from(inner: TcpStream) -> Self {
        MaybeHttpsStream::Http(inner)
    }
}

impl From<TlsStream<TcpStream>> for MaybeHttpsStream {
    fn from(inner: TlsStream<TcpStream>) -> Self {
        MaybeHttpsStream::Https(Box::new(inner))
    }
}

impl AsyncRead for MaybeHttpsStream {
    // // #[inline]
    // unsafe fn prepare_uninitialized_buffer(&self, buf: &mut [std::mem::MaybeUninit<u8>]) -> bool {
    //     match self {
    //         MaybeHttpsStream::Http(s) => s.prepare_uninitialized_buffer(buf),
    //         MaybeHttpsStream::Https(s) => s.prepare_uninitialized_buffer(buf),
    //     }
    // }

    // #[inline]
    fn poll_read(
        self: Pin<&mut Self>,
        cx: &mut Context,
        buf: &mut ReadBuf,
    ) -> Poll<Result<(), io::Error>> {
        match Pin::get_mut(self) {
            MaybeHttpsStream::Http(s) => Pin::new(s).poll_read(cx, buf),
            MaybeHttpsStream::Https(s) => Pin::new(s).poll_read(cx, buf),
        }
    }

    // // #[inline]
    // fn poll_read_buf<B: BufMut>(
    //     self: Pin<&mut Self>,
    //     cx: &mut Context<'_>,
    //     buf: &mut B,
    // ) -> Poll<Result<usize, io::Error>> {
    //     match Pin::get_mut(self) {
    //         MaybeHttpsStream::Http(s) => Pin::new(s).poll_read_buf(cx, buf),
    //         MaybeHttpsStream::Https(s) => Pin::new(s).poll_read_buf(cx, buf),
    //     }
    // }
}

impl AsyncWrite for MaybeHttpsStream {
    // #[inline]
    fn poll_write(
        self: Pin<&mut Self>,
        cx: &mut Context<'_>,
        buf: &[u8],
    ) -> Poll<Result<usize, io::Error>> {
        match Pin::get_mut(self) {
            MaybeHttpsStream::Http(s) => Pin::new(s).poll_write(cx, buf),
            MaybeHttpsStream::Https(s) => Pin::new(s).poll_write(cx, buf),
        }
    }

    // #[inline]
    fn poll_flush(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Result<(), io::Error>> {
        match Pin::get_mut(self) {
            MaybeHttpsStream::Http(s) => Pin::new(s).poll_flush(cx),
            MaybeHttpsStream::Https(s) => Pin::new(s).poll_flush(cx),
        }
    }

    // #[inline]
    fn poll_shutdown(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Result<(), io::Error>> {
        match Pin::get_mut(self) {
            MaybeHttpsStream::Http(s) => Pin::new(s).poll_shutdown(cx),
            MaybeHttpsStream::Https(s) => Pin::new(s).poll_shutdown(cx),
        }
    }

    // // #[inline]
    // fn poll_write_buf<B: Buf>(
    //     self: Pin<&mut Self>,
    //     cx: &mut Context<'_>,
    //     buf: &mut B,
    // ) -> Poll<Result<usize, io::Error>> {
    //     match Pin::get_mut(self) {
    //         MaybeHttpsStream::Http(s) => Pin::new(s).poll_write_buf(cx, buf),
    //         MaybeHttpsStream::Https(s) => Pin::new(s).poll_write_buf(cx, buf),
    //     }
    // }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::utils::base64_auth;
    use tokio::io::{AsyncReadExt, AsyncWriteExt};

    #[tokio::test]
    async fn http_stream() {
        let mut client = MaybeHttpsStream::new("http://api.ipify.org").await.unwrap();
        client
            .send_msg(b"GET / HTTP/1.0\r\nHost: api.ipify.org\r\n\r\n")
            .await
            .unwrap();
        let mut buf = Vec::new();
        client.read_to_end(&mut buf).await.unwrap();
        let body = String::from_utf8(buf).unwrap();
        assert!(&body.contains(crate::tests::IP.as_str()));
    }

    #[tokio::test]
    async fn https_stream() {
        let mut client = MaybeHttpsStream::new("https://api.ipify.org")
            .await
            .unwrap();
        client
            .write_all(b"GET / HTTP/1.0\r\nHost: api.ipify.org\r\n\r\n")
            .await
            .unwrap();
        client.flush().await.unwrap();
        let mut buf = Vec::new();
        client.read_to_end(&mut buf).await.unwrap();
        let body = String::from_utf8(buf).unwrap();
        assert!(&body.contains(crate::tests::IP.as_str()));
    }

    #[tokio::test]
    async fn http_stream_http_proxy() {
        dotenv::dotenv().ok();
        let http_proxy = match dotenv::var("TEST_HTTP_PROXY") {
            Ok(it) => it,
            _ => return,
        };
        let mut client = MaybeHttpsStream::new(&http_proxy).await.unwrap();
        client
            .write_all(b"GET http://api.ipify.org/ HTTP/1.0\r\nHost: api.ipify.org\r\n\r\n")
            .await
            .unwrap();
        client.flush().await.unwrap();
        let mut buf = Vec::new();
        client.read_to_end(&mut buf).await.unwrap();
        let body = String::from_utf8(buf).unwrap();
        assert!(&body.contains(crate::tests::IP.as_str()));
    }

    #[tokio::test]
    async fn http_stream_auth_http_proxy() {
        dotenv::dotenv().ok();

        let http_proxy = match dotenv::var("TEST_HTTP_AUTH_PROXY") {
            Ok(it) => it,
            _ => return,
        };
        let url = http_proxy.parse::<Url>().unwrap();
        let mut client = MaybeHttpsStream::new(&http_proxy).await.unwrap();
        let auth = base64_auth(&url).unwrap();

        let body = format!("GET http://api.ipify.org/ HTTP/1.0\r\nHost: api.ipify.org\r\nProxy-Authorization: Basic {}\r\n\r\n", auth);
        client.write_all(body.as_bytes()).await.unwrap();
        client.flush().await.unwrap();
        let mut buf = Vec::new();
        client.read_to_end(&mut buf).await.unwrap();
        let body = String::from_utf8(buf).unwrap();
        assert!(&body.contains(crate::tests::IP.as_str()));
    }
}
