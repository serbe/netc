use std::fmt;
use std::io;
use std::pin::Pin;
use std::task::{Context, Poll};

use bytes::{Buf, BufMut};
use tokio::io::{AsyncRead, AsyncWrite, AsyncReadExt, AsyncWriteExt};
use tokio::net::TcpStream;
use tokio_tls::TlsStream;
use uri::Uri;
use rsl::socks5;
use native_tls::TlsConnector;

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
            // Ok(String::from_utf8(header).or(Err(HttpError::HeaderNotUtf8))?)
        Response::from_header(&header)
    }

    // async fn copy_until(&mut self, writer: &mut Vec<u8>, val: &[u8]) -> Result<usize> {
    //     let mut buf = Vec::with_capacity(200);

    //     let mut pre_buf = [0; 10];
    //     let mut read = self.read(&mut pre_buf).await?;
    //     buf.extend(&pre_buf[..read]);

    //     for byte in self.bytes() {
    //         buf.push(byte?);
    //         read += 1;

    //         if &buf[(buf.len() - val.len())..] == val {
    //             break;
    //         }
    //     }

    //     writer.write_all(&buf)?;
    //     writer.flush()?;

    //     Ok(read)
    // }

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
}

// impl<T: AsyncRead + AsyncWrite + Connection + Unpin> Connection for MaybeHttpsStream<T> {
//     fn connected(&self) -> Connected {
//         match self {
//             MaybeHttpsStream::Http(s) => s.connected(),
//             MaybeHttpsStream::Https(s) => s.get_ref().connected(),
//         }
//     }
// }
