use tokio::runtime::Runtime;

pub use crate::client::Client;
pub use crate::client_builder::ClientBuilder;
pub use crate::error::Error;
pub use crate::headers::Headers;
pub use crate::method::Method;
pub use crate::request::Request;
pub use crate::response::Response;
pub use crate::status::{Status, StatusCode};
pub use crate::stream::MaybeHttpsStream;
pub use crate::version::Version;

mod client;
mod client_builder;
mod error;
mod headers;
mod http;
mod method;
mod request;
mod response;
mod status;
mod stream;
mod utils;
mod version;

// fn ip() -> String {
//     use std::io::{Read, Write};
//     use std::net::TcpStream;

//     let mut stream = TcpStream::connect("api.ipify.org:80").unwrap();
//     stream
//         .write_all(b"GET / HTTP/1.0\r\nHost: api.ipify.org\r\n\r\n")
//         .unwrap();
//     stream.flush().unwrap();
//     let mut buf = Vec::new();
//     stream.read_to_end(&mut buf).unwrap();
//     let body = String::from_utf8(buf).unwrap();
//     let split: Vec<&str> = body.splitn(2, "\r\n\r\n").collect();
//     split[1].to_string()
// }

async fn run() -> Result<(), Error> {
    // let http_proxy = "http://test:tset1@127.0.0.1:5656";
    // let socks_proxy = "socks5://test:tset1@127.0.0.1:5757";
    // const SECURE_URL: &'static str = "https://www.socks-proxy.net/";
    // let ip = ip();

    let client_builder = Client::builder().get("https://www.socks-proxy.net");
    // .proxy(socks_proxy);
    // .proxy(http_proxy);

    // .build()
    // .await
    // .unwrap();
    println!("{:?}", client_builder);
    let mut client = client_builder.build().await?;
    println!("{:?}", client);
    let request = client.request();
    println!("{:?}", request);
    let response = client.send().await.unwrap();
    println!("{:?}", response);
    // let body = response.text().unwrap();
    // println!("{:?}", body);

    Ok(())
}

fn main() {
    let rt = Runtime::new().unwrap();

    rt.block_on(async { run().await.unwrap() });
}

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
