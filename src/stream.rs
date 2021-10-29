use std::{
    fmt, io,
    io::Write,
    pin::Pin,
    sync::Arc,
    task::{Context, Poll},
};

use bytes::Bytes;
use rsl::socks5;
use tokio::{
    io::{AsyncRead, AsyncReadExt, AsyncWrite, AsyncWriteExt, ReadBuf},
    net::TcpStream,
};
use tokio_rustls::{client::TlsStream, rustls::{ClientConfig, client::ServerName, RootCertStore, OwnedTrustAnchor}, TlsConnector};
use uri::{into_uri::IntoUri, Uri};

use crate::{Error, Response, Version};

const CHUNK_MAX_SIZE: usize = 0x4000; // Maximum size of a TLS fragment
const CHUNK_HEADER_MAX_SIZE: usize = 6; // four hex digits plus "\r\n"
const CHUNK_FOOTER_SIZE: usize = 2; // "\r\n"
const CHUNK_MAX_PAYLOAD_SIZE: usize = CHUNK_MAX_SIZE - CHUNK_HEADER_MAX_SIZE - CHUNK_FOOTER_SIZE;

pub enum HttpStream {
    Http(TcpStream),
    Https(Box<TlsStream<TcpStream>>),
}

impl HttpStream {
    pub async fn new<U: IntoUri>(value: U) -> Result<Self, Error> {
        let uri = value.into_uri()?;
        let socket_addr = uri.socket_addr()?;
        let stream = TcpStream::connect(socket_addr).await?;
        HttpStream::maybe_ssl(&uri, stream).await
    }

    pub async fn socks(proxy: &Uri, target: &Uri) -> Result<Self, Error> {
        let stream = socks5::connect_uri(proxy, target).await?;
        HttpStream::maybe_ssl(target, stream).await
    }

    async fn maybe_ssl(uri: &Uri, stream: TcpStream) -> Result<Self, Error> {
        if uri.scheme() == "https" {
            let mut root_store = RootCertStore::empty();
            root_store.add_server_trust_anchors(webpki_roots::TLS_SERVER_ROOTS.0
                .iter()
                .map(|ta| {
                    OwnedTrustAnchor::from_subject_spki_name_constraints(
                        ta.subject,
                        ta.spki,
                        ta.name_constraints,
                    )
                }),);

            let config = ClientConfig::builder().with_safe_defaults().with_root_certificates(root_store).with_no_client_auth();
            
            let connector = TlsConnector::from(Arc::new(config));
            let server_name = ServerName::try_from(uri.host_str()).map_err(|_| Error::InvalidDnsNameError(uri.host_str().to_string()))?;
            let stream = connector.connect(server_name, stream).await?;
            Ok(HttpStream::from(stream))
        } else {
            Ok(HttpStream::from(stream))
        }
    }

    pub async fn get_body(&mut self, content_len: usize) -> Result<Bytes, Error> {
        let mut body = vec![0u8; content_len];
        self.read_exact(&mut body).await?;
        Ok(body.into())
    }

    pub async fn get_chunked_body(&mut self) -> Result<Bytes, Error> {
        let mut body = Vec::new();
        let mut chunk = Vec::with_capacity(CHUNK_MAX_SIZE);

        // self.read_to_end(&mut body).await?;
        // dbg!("get_chunked_body", String::from_utf8_lossy(&body));

        loop {
            chunk.resize(CHUNK_HEADER_MAX_SIZE, 0);
            let payload_size = self
                .take(CHUNK_MAX_PAYLOAD_SIZE as u64)
                .read_to_end(&mut chunk)
                .await?;

            // Then write the header
            let header_str = format!("{:x}\r\n", payload_size);
            let header = header_str.as_bytes();
            assert!(header.len() <= CHUNK_HEADER_MAX_SIZE);
            let start_index = CHUNK_HEADER_MAX_SIZE - header.len();
            (&mut chunk[start_index..]).write_all(header).unwrap();

            // And add the footer
            chunk.extend_from_slice(b"\r\n");

            // Finally Write the chunk
            std::io::Write::write_all(&mut body, &chunk[start_index..]).unwrap();

            // On EOF, we wrote a 0 sized chunk. This is what the chunked encoding protocol requires.
            if payload_size == 0 {
                return Ok(body.into());
            }
        }
    }

    pub async fn get_response(&mut self) -> Result<Response, Error> {
        let mut header = Vec::with_capacity(512);
        while !(header.len() > 4 && header[header.len() - 4..] == b"\r\n\r\n"[..]) {
            header.push(self.read_u8().await.or(Err(Error::HeaderIncomplete))?);
            if header.len() > 1024 {
                return Err(Error::HeaderToBig);
            }
        }
        let mut response = Response::from_header(&header)?;
        let content_len = response.content_len()?;
        dbg!(content_len);
        let body = match (content_len > 0, response.status.version()) {
            (true, _) => self.get_body(content_len).await?,
            (false, Version::Http11) => self.get_chunked_body().await?,
            _ => Bytes::new(),
        };
        response.body = body;
        Ok(response)
    }

    pub async fn send_msg(&mut self, msg: &[u8]) -> Result<(), Error> {
        self.write_all(msg).await?;
        self.flush().await?;
        Ok(())
    }
}

impl fmt::Debug for HttpStream {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            HttpStream::Http(s) => f.debug_tuple("Http").field(s).finish(),
            HttpStream::Https(s) => f.debug_tuple("Https").field(s).finish(),
        }
    }
}

impl From<TcpStream> for HttpStream {
    fn from(inner: TcpStream) -> Self {
        HttpStream::Http(inner)
    }
}

impl From<TlsStream<TcpStream>> for HttpStream {
    fn from(inner: TlsStream<TcpStream>) -> Self {
        HttpStream::Https(Box::new(inner))
    }
}

impl AsyncRead for HttpStream {
    // // #[inline]
    // unsafe fn prepare_uninitialized_buffer(&self, buf: &mut [std::mem::MaybeUninit<u8>]) -> bool {
    //     match self {
    //         HttpStream::Http(s) => s.prepare_uninitialized_buffer(buf),
    //         HttpStream::Https(s) => s.prepare_uninitialized_buffer(buf),
    //     }
    // }

    // #[inline]
    fn poll_read(
        self: Pin<&mut Self>,
        cx: &mut Context,
        buf: &mut ReadBuf,
    ) -> Poll<Result<(), io::Error>> {
        match Pin::get_mut(self) {
            HttpStream::Http(s) => Pin::new(s).poll_read(cx, buf),
            HttpStream::Https(s) => Pin::new(s).poll_read(cx, buf),
        }
    }

    // // #[inline]
    // fn poll_read_buf<B: BufMut>(
    //     self: Pin<&mut Self>,
    //     cx: &mut Context<'_>,
    //     buf: &mut B,
    // ) -> Poll<Result<usize, io::Error>> {
    //     match Pin::get_mut(self) {
    //         HttpStream::Http(s) => Pin::new(s).poll_read_buf(cx, buf),
    //         HttpStream::Https(s) => Pin::new(s).poll_read_buf(cx, buf),
    //     }
    // }
}

impl AsyncWrite for HttpStream {
    // #[inline]
    fn poll_write(
        self: Pin<&mut Self>,
        cx: &mut Context<'_>,
        buf: &[u8],
    ) -> Poll<Result<usize, io::Error>> {
        match Pin::get_mut(self) {
            HttpStream::Http(s) => Pin::new(s).poll_write(cx, buf),
            HttpStream::Https(s) => Pin::new(s).poll_write(cx, buf),
        }
    }

    // #[inline]
    fn poll_flush(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Result<(), io::Error>> {
        match Pin::get_mut(self) {
            HttpStream::Http(s) => Pin::new(s).poll_flush(cx),
            HttpStream::Https(s) => Pin::new(s).poll_flush(cx),
        }
    }

    // #[inline]
    fn poll_shutdown(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Result<(), io::Error>> {
        match Pin::get_mut(self) {
            HttpStream::Http(s) => Pin::new(s).poll_shutdown(cx),
            HttpStream::Https(s) => Pin::new(s).poll_shutdown(cx),
        }
    }

    // // #[inline]
    // fn poll_write_buf<B: Buf>(
    //     self: Pin<&mut Self>,
    //     cx: &mut Context<'_>,
    //     buf: &mut B,
    // ) -> Poll<Result<usize, io::Error>> {
    //     match Pin::get_mut(self) {
    //         HttpStream::Http(s) => Pin::new(s).poll_write_buf(cx, buf),
    //         HttpStream::Https(s) => Pin::new(s).poll_write_buf(cx, buf),
    //     }
    // }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tokio::io::{AsyncReadExt, AsyncWriteExt};

    #[tokio::test]
    async fn http_stream() {
        let mut client = HttpStream::new("http://api.ipify.org").await.unwrap();
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
        let mut client = HttpStream::new("https://api.ipify.org").await.unwrap();
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
        let mut client = HttpStream::new(&http_proxy).await.unwrap();
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
        let uri = http_proxy.parse::<Uri>().unwrap();
        let mut client = HttpStream::new(&http_proxy).await.unwrap();
        let auth = uri.base64_auth().unwrap();

        let body = format!("GET http://api.ipify.org/ HTTP/1.0\r\nHost: api.ipify.org\r\nProxy-Authorization: Basic {}\r\n\r\n", auth);
        client.write_all(body.as_bytes()).await.unwrap();
        client.flush().await.unwrap();
        let mut buf = Vec::new();
        client.read_to_end(&mut buf).await.unwrap();
        let body = String::from_utf8(buf).unwrap();
        assert!(&body.contains(crate::tests::IP.as_str()));
    }
}
