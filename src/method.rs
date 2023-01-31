use std::fmt;

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Method {
    Get,
    Head,
    Post,
    Put,
    Delete,
    Connect,
    Options,
    Trace,
    Patch,
    Other(String),
}

impl Method {
    pub fn as_str(&self) -> &str {
        match self {
            Method::Get => "GET",
            Method::Head => "HEAD",
            Method::Post => "POST",
            Method::Put => "PUT",
            Method::Delete => "DELETE",
            Method::Connect => "CONNECT",
            Method::Options => "OPTIONS",
            Method::Trace => "TRACE",
            Method::Patch => "PATCH",
            Method::Other(s) => s,
        }
    }
}

impl Default for Method {
    fn default() -> Self {
        Method::Get
    }
}

impl From<&str> for Method {
    fn from(s: &str) -> Self {
        match s.to_ascii_uppercase().as_str() {
            "GET" => Method::Get,
            "HEAD" => Method::Head,
            "POST" => Method::Post,
            "PUT" => Method::Put,
            "DELETE" => Method::Delete,
            "CONNECT" => Method::Connect,
            "OPTIONS" => Method::Options,
            "TRACE" => Method::Trace,
            "PATCH" => Method::Patch,
            _ => Method::Other(s.to_string()),
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
        let method_get = Method::Get;
        let method_get_expect: Method = "GET".into();
        let method_head = Method::Head;
        let method_head_expect: Method = "HEAD".into();
        let method_post = Method::Post;
        let method_post_expect: Method = "POST".into();
        let method_put = Method::Put;
        let method_put_expect: Method = "PUT".into();
        let method_delete = Method::Delete;
        let method_delete_expect: Method = "DELETE".into();
        let method_connect = Method::Connect;
        let method_connect_expect: Method = "CONNECT".into();
        let method_options = Method::Options;
        let method_options_expect: Method = "OPTIONS".into();
        let method_trace = Method::Trace;
        let method_trace_expect: Method = "TRACE".into();
        let method_patch = Method::Patch;
        let method_patch_expect: Method = "PATCH".into();

        assert_eq!(method_get_expect, method_get);
        assert_eq!(method_head_expect, method_head);
        assert_eq!(method_post_expect, method_post);
        assert_eq!(method_put_expect, method_put);
        assert_eq!(method_delete_expect, method_delete);
        assert_eq!(method_connect_expect, method_connect);
        assert_eq!(method_options_expect, method_options);
        assert_eq!(method_trace_expect, method_trace);
        assert_eq!(method_patch_expect, method_patch);
    }

    #[test]
    fn method_to_string() {
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
        let method_options = Method::Options;
        let method_options_expect = "OPTIONS";
        let method_trace = Method::Trace;
        let method_trace_expect = "TRACE";
        let method_patch = Method::Patch;
        let method_patch_expect = "PATCH";

        assert_eq!(method_get_expect, method_get.as_str());
        assert_eq!(method_head_expect, method_head.as_str());
        assert_eq!(method_post_expect, method_post.as_str());
        assert_eq!(method_put_expect, method_put.as_str());
        assert_eq!(method_delete_expect, method_delete.as_str());
        assert_eq!(method_connect_expect, method_connect.as_str());
        assert_eq!(method_options_expect, method_options.as_str());
        assert_eq!(method_trace_expect, method_trace.as_str());
        assert_eq!(method_patch_expect, method_patch.as_str());
    }
}
