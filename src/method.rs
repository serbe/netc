use std::{fmt, str::FromStr};

use crate::Error;

#[derive(Clone, Debug, PartialEq)]
pub enum Method {
    Options,
    Get,
    Head,
    Post,
    Put,
    Delete,
    Trace,
    Connect,
    Custom(String),
}

impl Method {
    pub fn as_str(&self) -> &str {
        match self {
            Method::Options => "OPTIONS",
            Method::Get => "GET",
            Method::Head => "HEAD",
            Method::Post => "POST",
            Method::Put => "PUT",
            Method::Delete => "DELETE",
            Method::Trace => "TRACE",
            Method::Connect => "CONNECT",
            Method::Custom(s) => s.as_str(),
        }
    }
}

impl Default for Method {
    fn default() -> Self {
        Method::Get
    }
}

impl FromStr for Method {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Error> {
        match s.to_ascii_uppercase().as_str() {
            "OPTIONS" => Ok(Method::Options),
            "GET" => Ok(Method::Get),
            "HEAD" => Ok(Method::Head),
            "POST" => Ok(Method::Post),
            "PUT" => Ok(Method::Put),
            "DELETE" => Ok(Method::Delete),
            "CONNECT" => Ok(Method::Connect),
            "TRACE" => Ok(Method::Trace),
            s => Ok(Method::Custom(s.to_string())),
        }
    }
}

impl fmt::Display for Method {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.as_str())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn method_parse() {
        let method_options = Method::Options;
        let method_options_expect: Method = "OPTIONS".parse().unwrap();
        let method_get = Method::Get;
        let method_get_expect: Method = "GET".parse().unwrap();
        let method_head = Method::Head;
        let method_head_expect: Method = "HEAD".parse().unwrap();
        let method_post = Method::Post;
        let method_post_expect: Method = "POST".parse().unwrap();
        let method_put = Method::Put;
        let method_put_expect: Method = "PUT".parse().unwrap();
        let method_delete = Method::Delete;
        let method_delete_expect: Method = "DELETE".parse().unwrap();
        let method_connect = Method::Connect;
        let method_connect_expect: Method = "CONNECT".parse().unwrap();
        let method_trace = Method::Trace;
        let method_trace_expect: Method = "TRACE".parse().unwrap();
        let method_custom = Method::Custom("PATCH".to_string());
        let method_custom_expect: Method = "PATCH".parse().unwrap();

        assert_eq!(method_options_expect, method_options);
        assert_eq!(method_get_expect, method_get);
        assert_eq!(method_head_expect, method_head);
        assert_eq!(method_post_expect, method_post);
        assert_eq!(method_put_expect, method_put);
        assert_eq!(method_delete_expect, method_delete);
        assert_eq!(method_connect_expect, method_connect);
        assert_eq!(method_trace_expect, method_trace);
        assert_eq!(method_custom_expect, method_custom);
    }

    #[test]
    fn method_to_string() {
        let method_options = Method::Options;
        let method_options_expect = "OPTIONS";
        let method_get = Method::Get;
        let method_get_expect = "GET";
        let method_head = Method::Head;
        let method_head_expect = "HEAD";
        let method_post = Method::Post;
        let method_post_expect = "POST";
        let method_put = Method::Put;
        let method_put_expect = "PUT";
        let method_delete = Method::Delete;
        let method_delete_expect = "DELETE";
        let method_connect = Method::Connect;
        let method_connect_expect = "CONNECT";
        let method_trace = Method::Trace;
        let method_trace_expect = "TRACE";
        let method_custom = Method::Custom("PATCH".to_string());
        let method_custom_expect = "PATCH";

        assert_eq!(method_options_expect, method_options.as_str());
        assert_eq!(method_get_expect, method_get.as_str());
        assert_eq!(method_head_expect, method_head.as_str());
        assert_eq!(method_post_expect, method_post.as_str());
        assert_eq!(method_put_expect, method_put.as_str());
        assert_eq!(method_delete_expect, method_delete.as_str());
        assert_eq!(method_connect_expect, method_connect.as_str());
        assert_eq!(method_trace_expect, method_trace.as_str());
        assert_eq!(method_custom_expect, method_custom.as_str());
    }
}
