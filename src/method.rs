use std::fmt;
use std::str::FromStr;

use crate::error::{Error, Result};

#[derive(Clone, Debug, PartialEq)]
pub enum Method {
    CONNECT,
    DELETE,
    GET,
    HEAD,
    OPTIONS,
    PATCH,
    POST,
    PUT,
    TRACE,
}

impl Method {
    pub fn as_str(&self) -> &str {
        match self {
            Method::CONNECT => "CONNECT",
            Method::DELETE => "DELETE",
            Method::GET => "GET",
            Method::HEAD => "HEAD",
            Method::OPTIONS => "OPTIONS",
            Method::PATCH => "PATCH",
            Method::POST => "POST",
            Method::PUT => "PUT",
            Method::TRACE => "TRACE",
        }
    }
}

impl Default for Method {
    fn default() -> Self {
        Method::GET
    }
}

impl FromStr for Method {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self> {
        match s.to_ascii_uppercase().as_str() {
            "CONNECT" => Ok(Method::CONNECT),
            "DELETE" => Ok(Method::DELETE),
            "GET" => Ok(Method::GET),
            "HEAD" => Ok(Method::HEAD),
            "OPTIONS" => Ok(Method::OPTIONS),
            "PATCH" => Ok(Method::PATCH),
            "POST" => Ok(Method::POST),
            "PUT" => Ok(Method::PUT),
            "TRACE" => Ok(Method::TRACE),
            _ => Err(Error::UnknownMethod(s.to_owned())),
        }
    }
}

impl fmt::Display for Method {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let method = self.as_str();
        write!(f, "{}", method)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn method_parse() {
        let method_connect = Method::CONNECT;
        let method_connect_expect: Method = "CONNECT".parse().unwrap();
        let method_get = Method::GET;
        let method_get_expect: Method = "GET".parse().unwrap();
        let method_head = Method::HEAD;
        let method_head_expect: Method = "HEAD".parse().unwrap();
        let method_options = Method::OPTIONS;
        let method_options_expect: Method = "OPTIONS".parse().unwrap();
        let method_patch = Method::PATCH;
        let method_patch_expect: Method = "PATCH".parse().unwrap();
        let method_post = Method::POST;
        let method_post_expect: Method = "POST".parse().unwrap();
        let method_put = Method::PUT;
        let method_put_expect: Method = "PUT".parse().unwrap();
        let method_trace = Method::TRACE;
        let method_trace_expect: Method = "TRACE".parse().unwrap();

        assert_eq!(method_connect_expect, method_connect);
        assert_eq!(method_get_expect, method_get);
        assert_eq!(method_head_expect, method_head);
        assert_eq!(method_options_expect, method_options);
        assert_eq!(method_patch_expect, method_patch);
        assert_eq!(method_post_expect, method_post);
        assert_eq!(method_put_expect, method_put);
        assert_eq!(method_trace_expect, method_trace);
    }

    #[test]
    fn method_to_string() {
        let method_connect = Method::CONNECT;
        let method_connect_expect = "CONNECT";
        let method_get = Method::GET;
        let method_get_expect = "GET";
        let method_head = Method::HEAD;
        let method_head_expect = "HEAD";
        let method_options = Method::OPTIONS;
        let method_options_expect = "OPTIONS";
        let method_patch = Method::PATCH;
        let method_patch_expect = "PATCH";
        let method_post = Method::POST;
        let method_post_expect = "POST";
        let method_put = Method::PUT;
        let method_put_expect = "PUT";
        let method_trace = Method::TRACE;
        let method_trace_expect = "TRACE";

        assert_eq!(method_connect_expect, method_connect.as_str());
        assert_eq!(method_get_expect, method_get.as_str());
        assert_eq!(method_head_expect, method_head.as_str());
        assert_eq!(method_options_expect, method_options.as_str());
        assert_eq!(method_patch_expect, method_patch.as_str());
        assert_eq!(method_post_expect, method_post.as_str());
        assert_eq!(method_put_expect, method_put.as_str());
        assert_eq!(method_trace_expect, method_trace.as_str());
    }
}
