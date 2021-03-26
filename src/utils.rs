use base64::encode;
use url::Url;

pub fn base64_auth(url: &Url) -> Option<String> {
    match (url.username(), url.password()) {
        (user, Some(pass)) => Some(encode(&format!("{}:{}", user, pass))),
        _ => None,
    }
}

pub fn host_port(url: &Url) -> String {
    match (url.host_str(), url.port_or_known_default()) {
        (Some(host), Some(port)) => format!("{}:{}", host, port),
        (Some(host), None) => host.to_string(),
        _ => String::new(),
    }
}

pub fn host_header(url: &Url) -> String {
    match (url.host_str(), url.port()) {
        (Some(host), Some(port)) if Some(port) == url.port_or_known_default() => host.to_string(),
        (Some(host), Some(port)) => format!("{}, {}", host, port),
        (Some(host), None) => host.to_string(),
        _ => String::new(),
    }
}
