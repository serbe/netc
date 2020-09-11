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
            let dnsname = DNSNameRef::try_from_ascii_str(uri.host_str())?;
            let stream = connector.connect(dnsname, stream).await?;
            MaybeHttpsStream::from(stream)
        } else {
            MaybeHttpsStream::from(stream)
        };
        Ok(HttpStream { stream })
    }
}
