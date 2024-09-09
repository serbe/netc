/*!

The `netc` crate is HTTP client for the [Rust](http://rust-lang.org/) programming language.

*/

pub mod client;
pub mod client_builder;
pub mod error;
pub mod header;
pub mod headers;
pub mod method;
pub mod request;
pub mod response;
pub mod status;
pub mod stream;
mod utils;
pub mod version;

use utils::IntoUrl;

pub use crate::client::Client;
pub use crate::client_builder::ClientBuilder;
pub use crate::error::Error;
pub use crate::headers::Headers;
pub use crate::method::Method;
pub use crate::request::Request;
pub use crate::response::Response;
pub use crate::status::{Status, StatusCode};
pub use crate::stream::HttpStream;
pub use crate::version::Version;

#[cfg(test)]
pub(crate) fn my_ip() -> String {
    use std::io::{Read, Write};
    use std::net::TcpStream;

    let mut stream = TcpStream::connect("api.ipify.org:80").unwrap();
    stream
        .write_all(b"GET / HTTP/1.0\r\nHost: api.ipify.org\r\n\r\n")
        .unwrap();
    stream.flush().unwrap();
    let mut buf = Vec::new();
    stream.read_to_end(&mut buf).unwrap();
    let body = String::from_utf8(buf).unwrap();
    let split: Vec<&str> = body.splitn(2, "\r\n\r\n").collect();
    split[1].to_string()
}

pub async fn delete<U: IntoUrl>(url: U) -> Result<Response, Error> {
    ClientBuilder::new().delete(url).build().await?.send().await
}

pub async fn get<U: IntoUrl>(url: U) -> Result<Response, Error> {
    ClientBuilder::new().get(url).build().await?.send().await
}

pub async fn post<U: IntoUrl>(url: U) -> Result<Response, Error> {
    ClientBuilder::new().post(url).build().await?.send().await
}

#[cfg(test)]
mod tests {
    use std::sync::OnceLock;

    static IP: OnceLock<String> = OnceLock::new();

    pub fn ip_str() -> &'static str {
        IP.get_or_init(|| crate::my_ip())
    }
}

// OCTET          = <any 8-bit sequence of data>
// CHAR           = <any US-ASCII character (octets 0 - 127)>
// UPALPHA        = <any US-ASCII uppercase letter "A".."Z">
// LOALPHA        = <any US-ASCII lowercase letter "a".."z">
// ALPHA          = UPALPHA | LOALPHA
// DIGIT          = <any US-ASCII digit "0".."9">
// CTL            = <any US-ASCII control character
//                  (octets 0 - 31) and DEL (127)>
// CR             = <US-ASCII CR, carriage return (13)>
// LF             = <US-ASCII LF, linefeed (10)>
// SP             = <US-ASCII SP, space (32)>
// HT             = <US-ASCII HT, horizontal-tab (9)>
// <">            = <US-ASCII double-quote mark (34)>
// CRLF           = CR LF
// LWS            = [CRLF] 1*( SP | HT )
// TEXT           = <any OCTET except CTLs,
//                  but including LWS>
// HEX            = "A" | "B" | "C" | "D" | "E" | "F"
//                | "a" | "b" | "c" | "d" | "e" | "f" | DIGIT
// token          = 1*<any CHAR except CTLs or separators>
// separators     = "(" | ")" | "<" | ">" | "@"
//                | "," | ";" | ":" | "\" | <">
//                | "/" | "[" | "]" | "?" | "="
//                | "{" | "}" | SP | HT
// comment        = "(" *( ctext | quoted-pair | comment ) ")"
// ctext          = <any TEXT excluding "(" and ")">
// quoted-string  = ( <"> *(qdtext | quoted-pair ) <"> )
// qdtext         = <any TEXT except <">>
// quoted-pair    = "\" CHAR
