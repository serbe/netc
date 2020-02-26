use std::{io, result, num, str};

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

//             EmptyScheme => write!(w, "Uri no have scheme"),
//             EmptyAuthority => write!(w, "Uri no have authority"),
//             Io(e) => write!(w, "{}", e),
//             HandshakeError(e) => write!(w, "{}", e),
//             StdParseAddr(e) => write!(w, "{}", e),
//             NoneString => write!(w, "none string"),
//             ParseFragment(e) => write!(w, "parse fragmeng {}", e),
//             ParseHost => write!(w, "parse host"),
//             ParseAddr => write!(w, "parse addr"),
//             ParseIPv6 => write!(w, "parse ip version 6"),
//             ParsePort => write!(w, "parse port"),
//             ParseQuery(e) => write!(w, "parse query {}", e),
//             ParseScheme => write!(w, "parse scheme"),
//             ParseUserInfo(e) => write!(w, "parse user info {}", e),
//             NativeTls(e) => write!(w, "{}", e),
//             UnsupportedProxyScheme => write!(w, "unsupported proxy scheme"),
//             InvalidServerVersion => write!(w, "invalid socks server version"),
//             InvalidAuthVersion => write!(w, "invalid auth version"),
//             AuthFailure => write!(w, "failure, connection must be closed"),
//             InvalidAuthMethod => write!(w, "auth method not supported"),
//             InvalidAddressType => write!(w, "Invalid address type"),
//             InvalidReservedByte => write!(w, "Invalid reserved byte"),
//             UnknownError => write!(w, "unknown error"),
//             InvalidCommandProtocol => write!(w, "command not supported / protocol error"),
//             TtlExpired => write!(w, "TTL expired"),
//             RefusedByHost => write!(w, "connection refused by destination host"),
//             HostUnreachable => write!(w, "host unreachable"),
//             NetworkUnreachable => write!(w, "network unreachable"),
//             InvalidRuleset => write!(w, "connection not allowed by ruleset"),
//             GeneralFailure => write!(w, "general failure"),
//             FromUtf8(e) => write!(w, "{}", e),
//             Utf8(e) => write!(w, "{}", e),
//         }
//     }
// }
