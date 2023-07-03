#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("empty url")]
    EmptyUrl,
    #[error("empty host")]
    EmptyHost,
    #[error("wrong http")]
    WrongHttp,
    #[error("empty response")]
    EmptyResponse,
    #[error("parse headers")]
    ParseHeaders,
    #[error("unsupported scheme {0}")]
    UnsupportedScheme(String),
    #[error("unsupported version {0}")]
    UnsupportedVersion(String),
    #[error("bad status")]
    StatusErr,
    #[error("bad headers")]
    HeadersErr,
    #[error("io error")]
    Io(#[from] std::io::Error),
    #[error("parse int")]
    ParseInt(#[from] std::num::ParseIntError),
    #[error("parse utf8")]
    ParseUtf8(#[from] std::string::FromUtf8Error),
    #[error("utf8")]
    FromUtf8(#[from] std::str::Utf8Error),
    #[error("Socks5")]
    Socks5(#[from] rscl::error::Error),
    #[error("header incomplete")]
    HeaderIncomplete,
    #[error("header more when 1024")]
    HeaderToBig,
    #[error("invalid status code {0}")]
    InvalidStatusCode(u16),
    #[error("unsupported proxy scheme {0}")]
    UnsupportedProxyScheme(String),
    #[error("InvalidDNSNameError {0}")]
    InvalidDnsNameError(String),
    #[error("No get socket address")]
    SocketAddr,
    #[error("UrlParseError")]
    UrlError(#[from] url::ParseError),
    #[error("Empty version")]
    EmptyVersion,
    #[error("Empty status")]
    EmptyStatus,
    #[error("Into url {0}")]
    UntoUri(String),
    #[error("Invalid chunk size format")]
    InvalidChunkSize,
    #[error("Invalid chunk end of line")]
    InvalidChunkEOL,
    #[error("Chunk line length {0} > 4096")]
    ChunkLineTooLong(usize),
    #[error("Header name must start with uppercase char or *")]
    HeaderWrongNameStart,
    #[error("Header name must contain ascii alphanumeric or -")]
    HeaderWrongName,
    #[error("Maximum number of redirects reached")]
    MaxRedirects,
}

impl PartialEq for Error {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Error::EmptyUrl, Error::EmptyUrl) => true,
            (Error::EmptyHost, Error::EmptyHost) => true,
            (Error::WrongHttp, Error::WrongHttp) => true,
            (Error::EmptyResponse, Error::EmptyResponse) => true,
            (Error::ParseHeaders, Error::ParseHeaders) => true,
            (Error::UnsupportedScheme(scheme), Error::UnsupportedScheme(other_scheme)) => {
                scheme == other_scheme
            }
            (Error::UnsupportedVersion(version), Error::UnsupportedVersion(other_version)) => {
                version == other_version
            }
            (Error::StatusErr, Error::StatusErr) => true,
            (Error::HeadersErr, Error::HeadersErr) => true,
            (Error::Io(io), Error::Io(other_io)) => io.to_string() == other_io.to_string(),
            (Error::ParseInt(int), Error::ParseInt(other_int)) => int == other_int,
            (Error::ParseUtf8(utf8), Error::ParseUtf8(other_utf8)) => utf8 == other_utf8,
            (Error::FromUtf8(utf8), Error::FromUtf8(other_utf8)) => utf8 == other_utf8,

            (Error::Socks5(socks), Error::Socks5(other_socks)) => {
                socks.to_string() == other_socks.to_string()
            }
            (Error::HeaderIncomplete, Error::HeaderIncomplete) => true,
            (Error::HeaderToBig, Error::HeaderToBig) => true,
            (Error::InvalidStatusCode(code), Error::InvalidStatusCode(other_code)) => {
                code == other_code
            }
            (
                Error::UnsupportedProxyScheme(scheme),
                Error::UnsupportedProxyScheme(other_scheme),
            ) => scheme == other_scheme,
            (Error::InvalidDnsNameError(dns), Error::InvalidDnsNameError(other_dns)) => {
                dns == other_dns
            }
            (Error::SocketAddr, Error::SocketAddr) => true,
            (Error::UrlError(err), Error::UrlError(other_err)) => err == other_err,
            (Error::EmptyVersion, Error::EmptyVersion) => true,
            (Error::EmptyStatus, Error::EmptyStatus) => true,
            (Error::UntoUri(err), Error::UntoUri(other_err)) => err == other_err,
            (Error::InvalidChunkSize, Error::InvalidChunkSize) => true,
            (Error::InvalidChunkEOL, Error::InvalidChunkEOL) => true,
            (Error::ChunkLineTooLong(length), Error::ChunkLineTooLong(other_length)) => {
                length == other_length
            }
            (Error::HeaderWrongNameStart, Error::HeaderWrongNameStart) => true,
            (Error::HeaderWrongName, Error::HeaderWrongName) => true,
            (Error::MaxRedirects, Error::MaxRedirects) => true,
            _ => false,
        }
    }
}
