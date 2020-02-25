use std::fmt;
use std::str::FromStr;

use crate::error::{Error, Result};

#[derive(PartialEq, Debug, Clone)]
pub struct Status {
    version: String,
    code: StatusCode,
    reason: String,
}

impl Status {
    pub fn status_code(&self) -> StatusCode {
        self.code
    }

    pub fn version(&self) -> &str {
        &self.version
    }

    pub fn reason(&self) -> &str {
        &self.reason
    }
}

impl<T, U, V> From<(T, U, V)> for Status
where
    T: ToString,
    V: ToString,
    StatusCode: From<U>,
{
    fn from(status: (T, U, V)) -> Status {
        Status {
            version: status.0.to_string(),
            code: StatusCode::from(status.1),
            reason: status.2.to_string(),
        }
    }
}

impl FromStr for Status {
    type Err = Error;

    fn from_str(status_line: &str) -> Result<Status> {
        let mut status_line = status_line.trim().splitn(3, ' ');

        let version = status_line.next().ok_or(Error::StatusErr)?;
        let code: StatusCode = status_line.next().ok_or(Error::StatusErr)?.parse()?;
        let reason = status_line
            .next()
            .unwrap_or_else(|| code.reason().unwrap_or("Unknown"));

        Ok(Status::from((version, u16::from(code), reason)))
    }
}

#[derive(Debug, PartialEq, Clone, Copy)]
pub struct StatusCode(u16);

impl StatusCode {
    pub const fn new(code: u16) -> StatusCode {
        StatusCode(code)
    }

    pub fn is_info(self) -> bool {
        self.0 >= 100 && self.0 < 200
    }

    pub fn is_success(self) -> bool {
        self.0 >= 200 && self.0 < 300
    }

    pub fn is_redirect(self) -> bool {
        self.0 >= 300 && self.0 < 400
    }

    pub fn is_client_err(self) -> bool {
        self.0 >= 400 && self.0 < 500
    }

    pub fn is_server_err(self) -> bool {
        self.0 >= 500 && self.0 < 600
    }

    pub fn is<F: FnOnce(u16) -> bool>(self, f: F) -> bool {
        f(self.0)
    }

    pub fn reason(self) -> Option<&'static str> {
        match self.0 {
            100 => Some("Continue"),
            101 => Some("Switching Protocols"),
            102 => Some("Processing"),
            200 => Some("OK"),
            201 => Some("Created"),
            202 => Some("Accepted"),
            203 => Some("Non Authoritative Information"),
            204 => Some("No Content"),
            205 => Some("Reset Content"),
            206 => Some("Partial Content"),
            207 => Some("Multi-Status"),
            208 => Some("Already Reported"),
            226 => Some("IM Used"),
            300 => Some("Multiple Choices"),
            301 => Some("Moved Permanently"),
            302 => Some("Found"),
            303 => Some("See Other"),
            304 => Some("Not Modified"),
            305 => Some("Use Proxy"),
            307 => Some("Temporary Redirect"),
            308 => Some("Permanent Redirect"),
            400 => Some("Bad Request"),
            401 => Some("Unauthorized"),
            402 => Some("Payment Required"),
            403 => Some("Forbidden"),
            404 => Some("Not Found"),
            405 => Some("Method Not Allowed"),
            406 => Some("Not Acceptable"),
            407 => Some("Proxy Authentication Required"),
            408 => Some("Request Timeout"),
            409 => Some("Conflict"),
            410 => Some("Gone"),
            411 => Some("Length Required"),
            412 => Some("Precondition Failed"),
            413 => Some("Payload Too Large"),
            414 => Some("URI Too Long"),
            415 => Some("Unsupported Media Type"),
            416 => Some("Range Not Satisfiable"),
            417 => Some("Expectation Failed"),
            418 => Some("I'm a teapot"),
            421 => Some("Misdirected Request"),
            422 => Some("Unprocessable Entity"),
            423 => Some("Locked"),
            424 => Some("Failed Dependency"),
            426 => Some("Upgrade Required"),
            428 => Some("Precondition Required"),
            429 => Some("Too Many Requests"),
            431 => Some("Request Header Fields Too Large"),
            451 => Some("Unavailable For Legal Reasons"),
            500 => Some("Internal Server Error"),
            501 => Some("Not Implemented"),
            502 => Some("Bad Gateway"),
            503 => Some("Service Unavailable"),
            504 => Some("Gateway Timeout"),
            505 => Some("HTTP Version Not Supported"),
            506 => Some("Variant Also Negotiates"),
            507 => Some("Insufficient Storage"),
            508 => Some("Loop Detected"),
            510 => Some("Not Extended"),
            511 => Some("Network Authentication Required"),
            _ => None,
        }
    }
}

impl From<StatusCode> for u16 {
    fn from(code: StatusCode) -> Self {
        code.0
    }
}

impl From<u16> for StatusCode {
    fn from(code: u16) -> Self {
        StatusCode(code)
    }
}

impl fmt::Display for StatusCode {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl FromStr for StatusCode {
    type Err = Error;

    fn from_str(s: &str) -> Result<StatusCode> {
        Ok(StatusCode::new(s.parse()?))
    }
}
