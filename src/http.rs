use native_tls::TlsConnector;
use tokio::net::TcpStream;
use uri::Uri;

use crate::error::Result;
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

    // pub async fn connect_proxy(uri: &Uri) -> Result<Self> {
    //     let target = uri.socket_addr()?;
    //     let stream = TcpStream::connect(target).await?;
    //     let stream = MaybeHttpsStream::from(stream);
    //     Ok(HttpStream { stream })
    // }

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
