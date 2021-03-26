use base64::encode;
use url::Url;

use crate::error::{Error, Result};

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

pub trait IntoUrl: IntoUrlSealed {}

impl IntoUrl for Url {}
impl IntoUrl for String {}
impl<'a> IntoUrl for &'a str {}
impl<'a> IntoUrl for &'a String {}

pub trait IntoUrlSealed {
    fn into_url(self) -> Result<Url>;

    fn as_str(&self) -> &str;
}

impl IntoUrlSealed for Url {
    fn into_url(self) -> Result<Url> {
        if self.has_host() {
            Ok(self)
        } else {
            Err(Error::EmptyHost)
        }
    }

    fn as_str(&self) -> &str {
        self.as_ref()
    }
}

impl<'a> IntoUrlSealed for &'a str {
    fn into_url(self) -> Result<Url> {
        Url::parse(self)?.into_url()
    }

    fn as_str(&self) -> &str {
        self
    }
}

impl<'a> IntoUrlSealed for &'a String {
    fn into_url(self) -> Result<Url> {
        (&**self).into_url()
    }

    fn as_str(&self) -> &str {
        self.as_ref()
    }
}

impl<'a> IntoUrlSealed for String {
    fn into_url(self) -> Result<Url> {
        (&*self).into_url()
    }

    fn as_str(&self) -> &str {
        self.as_ref()
    }
}
