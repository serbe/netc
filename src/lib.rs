// pub mod chunk;
pub mod client;
pub mod client_builder;
pub mod error;
pub mod headers;
pub mod method;
pub mod request;
pub mod response;
pub mod status;
pub mod stream;
pub mod version;

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

#[cfg(test)]
mod tests {
    use lazy_static::lazy_static;

    lazy_static! {
        pub static ref IP: String = crate::my_ip();
    }
}
