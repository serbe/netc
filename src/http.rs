use std::sync::Arc;

use tokio::net::TcpStream;
use tokio_rustls::{rustls::ClientConfig, webpki::DNSNameRef, TlsConnector};
use url::Url;

use crate::error::{Error, Result};
use crate::stream::MaybeHttpsStream;

#[derive(Debug)]
pub struct HttpStream {
    stream: MaybeHttpsStream,
}

impl HttpStream {
    pub async fn connect(url: &Url) -> Result<Self> {
        let socket_address = url.socket_addrs(|| None)?;
        let target = socket_address.get(0).map_or(Err(Error::SocketAddr), Ok)?;
        let stream = TcpStream::connect(target).await?;
        let stream = if url.scheme() == "https" {
            let mut config = ClientConfig::new();
            config
                .root_store
                .add_server_trust_anchors(&webpki_roots::TLS_SERVER_ROOTS);
            let connector = TlsConnector::from(Arc::new(config));
            let dns_name = DNSNameRef::try_from_ascii_str(url.host_str().ok_or(Error::EmptyHost)?)?;
            let stream = connector.connect(dns_name, stream).await?;
            MaybeHttpsStream::from(stream)
        } else {
            MaybeHttpsStream::from(stream)
        };
        Ok(HttpStream { stream })
    }
}
