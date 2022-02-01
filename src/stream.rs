use std::{
    fmt,
    io,
    // io::Write,
    pin::Pin,
    sync::Arc,
    task::{Context, Poll},
};

use bytes::{BufMut, Bytes};
use rscl::socks5;
use tokio::{
    io::{AsyncRead, AsyncReadExt, AsyncWrite, AsyncWriteExt, ReadBuf},
    net::TcpStream,
};
use tokio_rustls::{
    client::TlsStream,
    rustls::{client::ServerName, ClientConfig, OwnedTrustAnchor, RootCertStore},
    TlsConnector,
};
use url::Url;

use crate::{utils::IntoUrl, Error, Request, Response};

const CHUNK_MAX_LINE_LENGTH: usize = 4096;
const HEADERS_MAX_LENGTH: usize = 4096;

pub enum HttpStream {
    Http(TcpStream),
    Https(Box<TlsStream<TcpStream>>),
}

impl HttpStream {
    pub async fn new<U: IntoUrl>(value: U) -> Result<Self, Error> {
        let url = value.into_url()?;
        let socket_addr = url.socket_addrs(|| None)?.pop().ok_or(Error::SocketAddr)?;
        let stream = TcpStream::connect(socket_addr).await?;
        HttpStream::maybe_ssl(&url, stream).await
    }

    pub async fn from_request(request: &Request) -> Result<Self, Error> {
        match &request.proxy {
            Some(proxy) => match proxy.scheme() {
                "socks5" | "socks5h" => Ok(HttpStream::socks(proxy, &request.url).await?),
                "http" | "https" => Ok(HttpStream::new(proxy).await?),
                scheme => Err(Error::UnsupportedProxyScheme(scheme.to_owned())),
            },
            None => Ok(HttpStream::new(&request.url).await?),
        }
    }

    pub async fn socks(proxy: &Url, target: &Url) -> Result<Self, Error> {
        let stream = socks5::connect_uri(proxy, target).await?;
        HttpStream::maybe_ssl(target, stream).await
    }

    async fn maybe_ssl(url: &Url, stream: TcpStream) -> Result<Self, Error> {
        if url.scheme() == "https" {
            let mut root_store = RootCertStore::empty();
            root_store.add_server_trust_anchors(webpki_roots::TLS_SERVER_ROOTS.0.iter().map(
                |ta| {
                    OwnedTrustAnchor::from_subject_spki_name_constraints(
                        ta.subject,
                        ta.spki,
                        ta.name_constraints,
                    )
                },
            ));

            let config = ClientConfig::builder()
                .with_safe_defaults()
                .with_root_certificates(root_store)
                .with_no_client_auth();

            let connector = TlsConnector::from(Arc::new(config));
            let host = url.host_str().unwrap_or("");
            let server_name = ServerName::try_from(host)
                .map_err(|_| Error::InvalidDnsNameError(host.to_string()))?;
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

    pub async fn get_response(&mut self) -> Result<Response, Error> {
        let mut header = Vec::with_capacity(512);
        while !(header.len() > 4 && header[header.len() - 4..] == b"\r\n\r\n"[..]) {
            header.push(self.read_u8().await.or(Err(Error::HeaderIncomplete))?);
            if header.len() > HEADERS_MAX_LENGTH {
                return Err(Error::HeaderToBig);
            }
        }
        let mut response = Response::from_header(&header)?;
        let body = match (
            response.has_body(),
            response.has_chuncked_body(),
            response.content_len(),
        ) {
            (true, false, Some(size)) => self.get_body(size).await?,
            (true, true, _) => self.get_chunked_body().await?,
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

    pub async fn read_chunk_line(&mut self) -> Result<usize, Error> {
        let mut buf = vec![];
        while !(buf.len() > 1 && buf[buf.len() - 2..] == b"\r\n"[..]) {
            buf.put_u8(self.read_u8().await?);
            if buf.len() >= CHUNK_MAX_LINE_LENGTH {
                return Err(Error::ChunkLineTooLong(buf.len()));
            }
        }
        let without_ext = buf
            .split(|b| *b == b';')
            .next()
            .ok_or(Error::InvalidChunkSize)?;
        let str_line = String::from_utf8(without_ext.to_vec())?;
        let size = usize::from_str_radix(str_line.trim(), 16)?;
        Ok(size)
    }

    pub async fn get_chunked_body(&mut self) -> Result<Bytes, Error> {
        let mut body = Vec::new();
        loop {
            match self.read_chunk_line().await? {
                size if size == 0 => break,
                size => {
                    let mut buf = vec![0u8; size];
                    self.read_exact(&mut buf).await?;
                    body.append(&mut buf);
                }
            }
            let mut buf = [0u8; 2];
            self.read(&mut buf).await?;
        }
        let mut buf = [0u8; 2];
        self.read(&mut buf).await?;
        if buf != [b'\r', b'\n'] {
            return Err(Error::InvalidChunkEOL);
        }
        Ok(body.into())
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
    // use crate::utils::base64_auth;

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

    // #[tokio::test]
    // async fn http_stream_auth_http_proxy() {
    //     dotenv::dotenv().ok();

    //     let http_proxy = match dotenv::var("TEST_HTTP_AUTH_PROXY") {
    //         Ok(it) => it,
    //         _ => return,
    //     };
    //     let url = http_proxy.parse::<Url>().unwrap();
    //     let mut client = HttpStream::new(&http_proxy).await.unwrap();
    //     let auth = base64_auth(&url).unwrap();

    //     let body = format!("GET http://api.ipify.org/ HTTP/1.0\r\nHost: api.ipify.org\r\nProxy-Authorization: Basic {}\r\n\r\n", auth);
    //     client.write_all(body.as_bytes()).await.unwrap();
    //     client.flush().await.unwrap();
    //     let mut buf = Vec::new();
    //     client.read_to_end(&mut buf).await.unwrap();
    //     let body = String::from_utf8(buf).unwrap();
    //     assert!(&body.contains(crate::tests::IP.as_str()));
    // }

    #[tokio::test]
    async fn chunked_body() {
        let mut client = HttpStream::new("https://www.socks-proxy.net/")
            .await
            .unwrap();
        client
            .send_msg(b"GET / HTTP/1.1\r\nHost: www.socks-proxy.net\r\n\r\n")
            .await
            .unwrap();
        let response = client.get_response().await.unwrap();
        assert!(!response.body().is_empty());
    }
}
