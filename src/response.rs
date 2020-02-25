use std::io::Write;
use std::str;

use crate::error::{Error, Result};
use crate::headers::Headers;
use crate::status::{Status, StatusCode};

#[derive(Debug, PartialEq, Clone)]
pub struct Response {
    status: Status,
    headers: Headers,
}

impl Response {
    pub fn from_head(head: &[u8]) -> Result<Response> {
        let mut head = str::from_utf8(head)?.splitn(2, '\n');

        let status = head.next().ok_or(Error::StatusErr)?.parse()?;
        let headers = head.next().ok_or(Error::HeadersErr)?.parse()?;

        Ok(Response { status, headers })
    }

    pub fn try_from<T: Write>(res: &[u8], writer: &mut T) -> Result<Response> {
        if res.is_empty() {
            Err(Error::EmptyResponse)
        } else {
            let mut pos = res.len();
            if let Some(v) = find_slice(res, &[13, 10, 13, 10]) {
                pos = v;
            }

            let response = Self::from_head(&res[..pos])?;
            writer.write_all(&res[pos..])?;

            Ok(response)
        }
    }

    pub fn status_code(&self) -> StatusCode {
        self.status.status_code()
    }

    pub fn version(&self) -> &str {
        &self.status.version()
    }

    pub fn reason(&self) -> &str {
        &self.status.reason()
    }

    pub fn headers(&self) -> &Headers {
        &self.headers
    }

    pub fn content_len(&self) -> Result<usize> {
        match self.headers().get("Content-Length") {
            Some(p) => Ok(p.parse()?),
            None => Ok(0),
        }
    }
}

pub fn find_slice<T>(data: &[T], e: &[T]) -> Option<usize>
where
    [T]: PartialEq,
{
    for i in 0..=data.len() - e.len() {
        if data[i..(i + e.len())] == *e {
            return Some(i + e.len());
        }
    }

    None
}

// #[cfg(test)]
// mod tests {
//     use super::*;

//     const RESPONSE: &[u8; 129] = b"HTTP/1.1 200 OK\r\n\
//                                          Date: Sat, 11 Jan 2003 02:44:04 GMT\r\n\
//                                          Content-Type: text/html\r\n\
//                                          Content-Length: 100\r\n\r\n\
//                                          <html>hello</html>\r\n\r\nhello";
//     const RESPONSE_H: &[u8; 102] = b"HTTP/1.1 200 OK\r\n\
//                                            Date: Sat, 11 Jan 2003 02:44:04 GMT\r\n\
//                                            Content-Type: text/html\r\n\
//                                            Content-Length: 100\r\n\r\n";
//     const BODY: &[u8; 27] = b"<html>hello</html>\r\n\r\nhello";

//     const STATUS_LINE: &str = "HTTP/1.1 200 OK";
//     const VERSION: &str = "HTTP/1.1";
//     const CODE: u16 = 200;
//     const REASON: &str = "OK";

//     const HEADERS: &str = "Date: Sat, 11 Jan 2003 02:44:04 GMT\r\n\
//                            Content-Type: text/html\r\n\
//                            Content-Length: 100\r\n";
//     const CODE_S: StatusCode = StatusCode(200);

//     #[test]
//     fn status_code_new() {
//         assert_eq!(StatusCode::new(200), StatusCode::new(200));
//         assert_ne!(StatusCode::new(400), StatusCode(404));
//     }

//     #[test]
//     fn status_code_info() {
//         for i in 100..200 {
//             assert!(StatusCode::new(i).is_info())
//         }

//         for i in (0..1000).filter(|&i| i < 100 || i >= 200) {
//             assert!(!StatusCode::new(i).is_info())
//         }
//     }

//     #[test]
//     fn status_code_success() {
//         for i in 200..300 {
//             assert!(StatusCode::new(i).is_success())
//         }

//         for i in (0..1000).filter(|&i| i < 200 || i >= 300) {
//             assert!(!StatusCode::new(i).is_success())
//         }
//     }

//     #[test]
//     fn status_code_redirect() {
//         for i in 300..400 {
//             assert!(StatusCode::new(i).is_redirect())
//         }

//         for i in (0..1000).filter(|&i| i < 300 || i >= 400) {
//             assert!(!StatusCode::new(i).is_redirect())
//         }
//     }

//     #[test]
//     fn status_code_client_err() {
//         for i in 400..500 {
//             assert!(StatusCode::new(i).is_client_err())
//         }

//         for i in (0..1000).filter(|&i| i < 400 || i >= 500) {
//             assert!(!StatusCode::new(i).is_client_err())
//         }
//     }

//     #[test]
//     fn status_code_server_err() {
//         for i in 500..600 {
//             assert!(StatusCode::new(i).is_server_err())
//         }

//         for i in (0..1000).filter(|&i| i < 500 || i >= 600) {
//             assert!(!StatusCode::new(i).is_server_err())
//         }
//     }

//     #[test]
//     fn status_code_is() {
//         let check = |i| i % 3 == 0;

//         let code_1 = StatusCode::new(200);
//         let code_2 = StatusCode::new(300);

//         assert!(!code_1.is(check));
//         assert!(code_2.is(check));
//     }

//     #[test]
//     fn status_code_reason() {
//         assert_eq!(StatusCode(200).reason(), Some("OK"));
//         assert_eq!(StatusCode(400).reason(), Some("Bad Request"));
//     }

//     #[test]
//     fn status_code_from() {
//         assert_eq!(StatusCode::from(200), StatusCode(200));
//     }

//     #[test]
//     fn u16_from_status_code() {
//         assert_eq!(u16::from(CODE_S), 200);
//     }

//     #[test]
//     fn status_code_display() {
//         let code = format!("Status Code: {}", StatusCode::new(200));
//         const CODE_EXPECT: &str = "Status Code: 200";

//         assert_eq!(code, CODE_EXPECT);
//     }

//     #[test]
//     fn status_code_from_str() {
//         assert_eq!("200".parse::<StatusCode>(), Ok(StatusCode(200)));
//         assert_ne!("400".parse::<StatusCode>(), Ok(StatusCode(404)));
//     }

//     #[test]
//     fn status_from() {
//         let status = Status::from((VERSION, CODE, REASON));

//         assert_eq!(status.version, VERSION);
//         assert_eq!(status.code, CODE_S);
//         assert_eq!(status.reason, REASON);
//     }

//     #[test]
//     fn status_from_str() {
//         let status = STATUS_LINE.parse::<Status>().unwrap();

//         assert_eq!(status.version, VERSION);
//         assert_eq!(status.code, CODE_S);
//         assert_eq!(status.reason, REASON);
//     }

//     #[test]
//     fn headers_new() {
//         assert_eq!(Headers::new(), Headers(HashMap::new()));
//     }

//     #[test]
//     fn headers_get() {
//         let mut headers = Headers::with_capacity(2);
//         headers.insert("Date", "Sat, 11 Jan 2003 02:44:04 GMT");

//         assert_eq!(
//             headers.get("Date"),
//             Some(&"Sat, 11 Jan 2003 02:44:04 GMT".to_string())
//         );
//     }

//     #[test]
//     fn headers_insert() {
//         let mut headers_expect = HashMap::new();
//         headers_expect.insert(Ascii::new("Connection".to_string()), "Close".to_string());
//         let headers_expect = Headers(headers_expect);

//         let mut headers = Headers::new();
//         headers.insert("Connection", "Close");

//         assert_eq!(headers_expect, headers);
//     }

//     #[test]
//     fn headers_default_http() {
//         let uri = "http://doc.rust-lang.org/std/string/index.html"
//             .parse()
//             .unwrap();

//         let mut headers = Headers::with_capacity(4);
//         headers.insert("Host", "doc.rust-lang.org");
//         headers.insert("Referer", "http://doc.rust-lang.org/std/string/index.html");

//         assert_eq!(Headers::default_http(&uri), headers);
//     }

//     #[test]
//     fn headers_from_str() {
//         let mut headers_expect = HashMap::with_capacity(2);
//         headers_expect.insert(
//             Ascii::new("Date".to_string()),
//             "Sat, 11 Jan 2003 02:44:04 GMT".to_string(),
//         );
//         headers_expect.insert(
//             Ascii::new("Content-Type".to_string()),
//             "text/html".to_string(),
//         );
//         headers_expect.insert(Ascii::new("Content-Length".to_string()), "100".to_string());

//         let headers = HEADERS.parse::<Headers>().unwrap();
//         assert_eq!(headers, Headers::from(headers_expect));
//     }

//     #[test]
//     fn headers_from() {
//         let mut headers_expect = HashMap::with_capacity(4);
//         headers_expect.insert(
//             Ascii::new("Date".to_string()),
//             "Sat, 11 Jan 2003 02:44:04 GMT".to_string(),
//         );
//         headers_expect.insert(
//             Ascii::new("Content-Type".to_string()),
//             "text/html".to_string(),
//         );
//         headers_expect.insert(Ascii::new("Content-Length".to_string()), "100".to_string());

//         assert_eq!(
//             Headers(headers_expect.clone()),
//             Headers::from(headers_expect)
//         );
//     }

//     #[test]
//     fn headers_case_insensitive() {
//         let header_names = ["Host", "host", "HOST", "HoSt"];
//         let mut headers = Headers::with_capacity(1);
//         headers.insert("Host", "doc.rust-lang.org");

//         for name in header_names.iter() {
//             assert_eq!(headers.get(name), Some(&"doc.rust-lang.org".to_string()));
//         }
//     }

//     #[test]
//     fn hash_map_from_headers() {
//         let mut headers = Headers::with_capacity(4);
//         headers.insert("Date", "Sat, 11 Jan 2003 02:44:04 GMT");
//         headers.insert("Content-Type", "text/html");
//         headers.insert("Content-Length", "100");

//         let mut headers_expect = HashMap::with_capacity(4);
//         headers_expect.insert(
//             Ascii::new("Date".to_string()),
//             "Sat, 11 Jan 2003 02:44:04 GMT".to_string(),
//         );
//         headers_expect.insert(
//             Ascii::new("Content-Type".to_string()),
//             "text/html".to_string(),
//         );
//         headers_expect.insert(Ascii::new("Content-Length".to_string()), "100".to_string());

//         assert_eq!(HashMap::from(headers), headers_expect);
//     }

//     #[test]
//     fn find_slice_e() {
//         const WORDS: [&str; 8] = ["Good", "job", "Great", "work", "Have", "fun", "See", "you"];
//         const SEARCH: [&str; 3] = ["Great", "work", "Have"];

//         assert_eq!(find_slice(&WORDS, &SEARCH), Some(5));
//     }

//     #[test]
//     fn res_from_head() {
//         Response::from_head(RESPONSE_H).unwrap();
//     }

//     #[test]
//     fn res_try_from() {
//         let mut writer = Vec::new();

//         Response::try_from(RESPONSE, &mut writer).unwrap();
//         Response::try_from(RESPONSE_H, &mut writer).unwrap();
//     }

//     #[test]
//     #[should_panic]
//     fn res_from_empty() {
//         let mut writer = Vec::new();
//         Response::try_from(&[], &mut writer).unwrap();
//     }

//     #[test]
//     fn res_status_code() {
//         let mut writer = Vec::new();
//         let res = Response::try_from(RESPONSE, &mut writer).unwrap();

//         assert_eq!(res.status_code(), CODE_S);
//     }

//     #[test]
//     fn res_version() {
//         let mut writer = Vec::new();
//         let res = Response::try_from(RESPONSE, &mut writer).unwrap();

//         assert_eq!(res.version(), "HTTP/1.1");
//     }

//     #[test]
//     fn res_reason() {
//         let mut writer = Vec::new();
//         let res = Response::try_from(RESPONSE, &mut writer).unwrap();

//         assert_eq!(res.reason(), "OK");
//     }

//     #[test]
//     fn res_headers() {
//         let mut writer = Vec::new();
//         let res = Response::try_from(RESPONSE, &mut writer).unwrap();

//         let mut headers = Headers::with_capacity(2);
//         headers.insert("Date", "Sat, 11 Jan 2003 02:44:04 GMT");
//         headers.insert("Content-Type", "text/html");
//         headers.insert("Content-Length", "100");

//         assert_eq!(res.headers(), &Headers::from(headers));
//     }

//     #[test]
//     fn res_content_len() {
//         let mut writer = Vec::with_capacity(101);
//         let res = Response::try_from(RESPONSE, &mut writer).unwrap();

//         assert_eq!(res.content_len(), Ok(100));
//     }

//     #[test]
//     fn res_body() {
//         let mut writer = Vec::new();
//         Response::try_from(RESPONSE, &mut writer).unwrap();

//         assert_eq!(writer, BODY);
//     }
// }
