use uri::Uri;

use crate::error::Result;
use crate::http::HttpStream;
use crate::proxy::ProxyStream;

#[derive(Debug)]
pub enum Transport {
    Proxy(ProxyStream),
    Stream(HttpStream),
    None,
}

impl Default for Transport {
    fn default() -> Self {
        Transport::None
    }
}

impl Transport {
    pub fn new() -> Self {
        Transport::default()
    }

    pub fn proxy(proxy: &Uri, target: &Uri) -> Result<Self> {
        Ok(Transport::Proxy(ProxyStream::from(proxy, target)?))
    }

    pub fn stream(uri: &Uri) -> Result<Self> {
        Ok(Transport::Stream(HttpStream::connect(uri)?))
    }
}
