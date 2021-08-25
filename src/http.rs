use std::sync::Arc;

use tokio::net::TcpStream;
use tokio_rustls::{rustls::ClientConfig, webpki::DNSNameRef, TlsConnector};
use uri::Uri;

use crate::{Error, MaybeHttpsStream};

#[derive(Debug)]
pub struct HttpStream {
    pub(crate) stream: MaybeHttpsStream,
}

impl HttpStream {
    pub async fn connect(uri: &Uri) -> Result<Self, Error> {
        let socket_addr = uri.socket_addr()?;
        let stream = TcpStream::connect(socket_addr).await?;
        let stream = if uri.scheme() == "https" {
            let mut config = ClientConfig::new();
            config
                .root_store
                .add_server_trust_anchors(&webpki_roots::TLS_SERVER_ROOTS);
            let connector = TlsConnector::from(Arc::new(config));
            let dns_name = DNSNameRef::try_from_ascii_str(uri.host_str())?;
            let stream = connector.connect(dns_name, stream).await?;
            MaybeHttpsStream::from(stream)
        } else {
            MaybeHttpsStream::from(stream)
        };
        Ok(HttpStream { stream })
    }
}
