use std::fmt;
use std::io;
use std::pin::Pin;
use std::task::{Context, Poll};

use bytes::{Buf, BufMut};
use native_tls::TlsConnector;
use rsl::socks5;
use tokio::io::{AsyncRead, AsyncReadExt, AsyncWrite, AsyncWriteExt};
use tokio::net::TcpStream;
use tokio_tls::TlsStream;
use uri::Uri;

use crate::error::Error;
use crate::response::Response;

pub enum MaybeHttpsStream {
    /// A stream over plain text.
    Http(TcpStream),
    /// A stream protected with TLS.
    Https(TlsStream<TcpStream>),
}

impl MaybeHttpsStream {
    pub async fn new(uri: &Uri) -> Result<Self, Error> {
        let addr = uri.socket_addr()?;
        let stream = TcpStream::connect(addr).await?;
        MaybeHttpsStream::maybe_ssl(uri, stream).await
    }

    pub async fn socks(proxy: &Uri, target: &Uri) -> Result<Self, Error> {
        let stream = socks5::connect(proxy.as_str(), target.as_str()).await?;
        MaybeHttpsStream::maybe_ssl(target, stream).await
    }

    async fn maybe_ssl(uri: &Uri, stream: TcpStream) -> Result<Self, Error> {
        if uri.is_ssl() {
            let cx = TlsConnector::builder().build()?;
            let cx = tokio_tls::TlsConnector::from(cx);
            let stream = cx.connect(&uri.host(), stream).await?;
            Ok(MaybeHttpsStream::from(stream))
        } else {
            Ok(MaybeHttpsStream::from(stream))
        }
    }

    pub async fn get_body(&mut self, content_len: usize) -> Result<Vec<u8>, Error> {
        let mut body = vec![0u8; content_len];
        self.read_exact(&mut body).await?;
        Ok(body)
    }

    pub async fn get_response(&mut self) -> Result<Response, Error> {
        let mut header = Vec::with_capacity(512);
        while !(header.len() > 4 && header[header.len() - 4..] == b"\r\n\r\n"[..]) {
            header.push(self.read_u8().await.or(Err(Error::HeaderIncomplete))?);
            if header.len() > 1024 {
                return Err(Error::HeaderToBig);
            }
        }
        Response::from_header(&header)
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
        MaybeHttpsStream::Https(inner)
    }
}

impl AsyncRead for MaybeHttpsStream {
    #[inline]
    unsafe fn prepare_uninitialized_buffer(&self, buf: &mut [std::mem::MaybeUninit<u8>]) -> bool {
        match self {
            MaybeHttpsStream::Http(s) => s.prepare_uninitialized_buffer(buf),
            MaybeHttpsStream::Https(s) => s.prepare_uninitialized_buffer(buf),
        }
    }

    #[inline]
    fn poll_read(
        self: Pin<&mut Self>,
        cx: &mut Context,
        buf: &mut [u8],
    ) -> Poll<Result<usize, io::Error>> {
        match Pin::get_mut(self) {
            MaybeHttpsStream::Http(s) => Pin::new(s).poll_read(cx, buf),
            MaybeHttpsStream::Https(s) => Pin::new(s).poll_read(cx, buf),
        }
    }

    #[inline]
    fn poll_read_buf<B: BufMut>(
        self: Pin<&mut Self>,
        cx: &mut Context<'_>,
        buf: &mut B,
    ) -> Poll<Result<usize, io::Error>> {
        match Pin::get_mut(self) {
            MaybeHttpsStream::Http(s) => Pin::new(s).poll_read_buf(cx, buf),
            MaybeHttpsStream::Https(s) => Pin::new(s).poll_read_buf(cx, buf),
        }
    }
}

impl AsyncWrite for MaybeHttpsStream {
    #[inline]
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

    #[inline]
    fn poll_flush(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Result<(), io::Error>> {
        match Pin::get_mut(self) {
            MaybeHttpsStream::Http(s) => Pin::new(s).poll_flush(cx),
            MaybeHttpsStream::Https(s) => Pin::new(s).poll_flush(cx),
        }
    }

    #[inline]
    fn poll_shutdown(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Result<(), io::Error>> {
        match Pin::get_mut(self) {
            MaybeHttpsStream::Http(s) => Pin::new(s).poll_shutdown(cx),
            MaybeHttpsStream::Https(s) => Pin::new(s).poll_shutdown(cx),
        }
    }

    #[inline]
    fn poll_write_buf<B: Buf>(
        self: Pin<&mut Self>,
        cx: &mut Context<'_>,
        buf: &mut B,
    ) -> Poll<Result<usize, io::Error>> {
        match Pin::get_mut(self) {
            MaybeHttpsStream::Http(s) => Pin::new(s).poll_write_buf(cx, buf),
            MaybeHttpsStream::Https(s) => Pin::new(s).poll_write_buf(cx, buf),
        }
    }
}

// impl<T: AsyncRead + AsyncWrite + Connection + Unpin> Connection for MaybeHttpsStream<T> {
//     fn connected(&self) -> Connected {
//         match self {
//             MaybeHttpsStream::Http(s) => s.connected(),
//             MaybeHttpsStream::Https(s) => s.get_ref().connected(),
//         }
//     }
// }

// #[cfg(test)]
// mod tests {
//     use super::*;
//     use tokio::io::{AsyncReadExt, AsyncWriteExt};

//     #[tokio::test]
//     async fn http_stream() {
//         let mut client = MaybeHttpsStream::new(&"http://api.ipify.org".parse::<Uri>().unwrap())
//             .await
//             .unwrap();
//         client
//             .send_msg(b"GET / HTTP/1.0\r\nHost: api.ipify.org\r\n\r\n")
//             .await
//             .unwrap();
//         let mut buf = Vec::new();
//         client.read_to_end(&mut buf).await.unwrap();
//         let body = String::from_utf8(buf).unwrap();
//         assert!(&body.contains(crate::tests::IP.as_str()));
//     }

//     #[tokio::test]
//     async fn https_stream() {
//         let mut client = MaybeHttpsStream::new(&"https://api.ipify.org".parse::<Uri>().unwrap())
//             .await
//             .unwrap();
//         client
//             .write_all(b"GET / HTTP/1.0\r\nHost: api.ipify.org\r\n\r\n")
//             .await
//             .unwrap();
//         client.flush().await.unwrap();
//         let mut buf = Vec::new();
//         client.read_to_end(&mut buf).await.unwrap();
//         let body = String::from_utf8(buf).unwrap();
//         assert!(&body.contains(crate::tests::IP.as_str()));
//     }

//     // #[test]
//     // fn http_stream_http_proxy() {
//     //     let mut client =
//     //         HttpStream::connect_proxy(&"http://127.0.0.1:5858".parse::<Uri>().unwrap()).unwrap();
//     //     client
//     //         .send_request(b"GET http://api.ipify.org/ HTTP/1.0\r\nHost: api.ipify.org\r\n\r\n")
//     //         .unwrap();
//     //     let response = client.get_response().unwrap();
//     //     let body = client.get_body(response.content_len().unwrap()).unwrap();
//     //     let body = String::from_utf8(body).unwrap();
//     //     assert!(&body.contains(crate::tests::IP.as_str()));
//     // }

//     #[tokio::test]
//     async fn http_stream_auth_http_proxy() {
//         let mut client = MaybeHttpsStream::new(&"http://127.0.0.1:5656".parse::<Uri>().unwrap())
//             .await
//             .unwrap();
//         client
//             .write_all(b"GET http://api.ipify.org/ HTTP/1.0\r\nHost: api.ipify.org\r\nProxy-Authorization: Basic dGVzdDp0c2V0\r\n\r\n")
//             .await
//             .unwrap();
//         client.flush().await.unwrap();
//         let mut buf = Vec::new();
//         client.read_to_end(&mut buf).await.unwrap();
//         let body = String::from_utf8(buf).unwrap();
//         assert!(&body.contains(crate::tests::IP.as_str()));
//     }
// }
