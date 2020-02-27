use std::{io, num, result, str};

use thiserror::Error;

pub type Result<T> = result::Result<T, Error>;

#[derive(Error, Debug)]
pub enum Error {
    #[error("empty Uri")]
    EmptyUri,
    #[error("wrong http")]
    WrongHttp,
    #[error("empty response")]
    EmptyResponse,
    #[error("parse headers")]
    ParseHeaders,
    #[error("unknown method {0}")]
    UnknownMethod(String),
    #[error("unsupported scheme {0}")]
    UnsupportedScheme(String),
    #[error("unsupported version {0}")]
    UnsupportedVersion(String),
    #[error("bad status")]
    StatusErr,
    #[error("bad headers")]
    HeadersErr,
    #[error("io error")]
    IO(#[from] io::Error),
    #[error("parse int")]
    ParseInt(#[from] num::ParseIntError),
    #[error("utf8")]
    FromUtf8(#[from] str::Utf8Error),
    #[error("uri")]
    UriError(#[from] uri::Error),
    #[error("NativeTls")]
    NativeTls(#[from] native_tls::Error),
    #[error("Socks5")]
    Socks5(#[from] rsl::error::Error),
    #[error("header incomplete")]
    HeaderIncomplete,
    #[error("header more when 1024")]
    HeaderToBig,
}
