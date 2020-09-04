use std::sync::Arc;

use tokio::net::TcpStream;
use tokio_rustls::{rustls::ClientConfig, webpki::DNSNameRef, TlsConnector};
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
            let mut config = ClientConfig::new();
            config
                .root_store
                .add_server_trust_anchors(&webpki_roots::TLS_SERVER_ROOTS);
            let connector = TlsConnector::from(Arc::new(config));
            let dnsname = DNSNameRef::try_from_ascii_str(uri.host())?;
            let stream = connector.connect(dnsname, stream).await?;
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
