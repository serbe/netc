use std::{io::Write, str};

use bytes::Bytes;

use crate::{utils::find_slice, Error, Headers, Method, Status, StatusCode, Version};

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Response {
    pub status: Status,
    pub headers: Headers,
    pub method: Method,
    pub body: Bytes,
}

impl Response {
    pub fn from_header(header: &[u8]) -> Result<Response, Error> {
        let mut header = str::from_utf8(header)?.splitn(2, '\n');

        let status = header.next().ok_or(Error::EmptyStatus)?.parse()?;
        let headers = header.next().ok_or(Error::HeadersErr)?.parse()?;
        let body = Bytes::new();

        Ok(Response {
            status,
            headers,
            method: Method::Get,
            body,
        })
    }

    pub fn try_from<T: Write>(res: &[u8], writer: &mut T) -> Result<Response, Error> {
        if res.is_empty() {
            Err(Error::EmptyResponse)
        } else {
            let mut pos = res.len();
            if let Some(v) = find_slice(res, &[13, 10, 13, 10]) {
                pos = v;
            }

            let response = Self::from_header(&res[..pos])?;
            writer.write_all(&res[pos..])?;

            Ok(response)
        }
    }

    pub fn status_code(&self) -> StatusCode {
        self.status.status_code()
    }

    pub fn version(&self) -> Version {
        self.status.version()
    }

    pub fn reason(&self) -> &str {
        self.status.reason()
    }

    pub fn headers(&self) -> &Headers {
        &self.headers
    }

    pub fn header(&self, value: &str) -> Option<String> {
        self.headers.get(value)
    }

    pub fn content_len(&self) -> Option<usize> {
        self.headers().content_length()
    }

    pub fn body(&self) -> Bytes {
        self.body.clone()
    }

    pub fn text(&self) -> Result<String, Error> {
        Ok(String::from_utf8_lossy(&self.body).to_string())
    }

    pub fn has_body(&self) -> bool {
        let has_no_body = self.method == Method::Head || self.status_code().is_nobody();
        !has_no_body
    }

    pub fn has_chuncked_body(&self) -> bool {
        let is_http10 = self.status.version() == Version::Http10;
        let is_chunked = self
            .headers
            .get_array("transfer-encoding")
            .contains(&"chunked".to_string());
        !is_http10 && self.has_body() && is_chunked
    }
}

#[cfg(test)]
mod tests {
    use httpmock::{Method::GET, MockServer};

    use super::*;
    use crate::{status::StatusCode, Client};

    const RESPONSE: &[u8; 129] = b"HTTP/1.1 200 OK\r\n\
                                         Date: Sat, 11 Jan 2003 02:44:04 GMT\r\n\
                                         Content-Type: text/html\r\n\
                                         Content-Length: 100\r\n\r\n\
                                         <html>hello</html>\r\n\r\nhello";
    const RESPONSE_H: &[u8; 102] = b"HTTP/1.1 200 OK\r\n\
                                           Date: Sat, 11 Jan 2003 02:44:04 GMT\r\n\
                                           Content-Type: text/html\r\n\
                                           Content-Length: 100\r\n\r\n";
    const BODY: &[u8; 27] = b"<html>hello</html>\r\n\r\nhello";

    #[test]
    fn res_from_head() {
        Response::from_header(RESPONSE_H).unwrap();
    }

    #[test]
    fn res_try_from() {
        let mut writer = Vec::new();

        Response::try_from(RESPONSE, &mut writer).unwrap();
        Response::try_from(RESPONSE_H, &mut writer).unwrap();
    }

    #[test]
    #[should_panic]
    fn res_from_empty() {
        let mut writer = Vec::new();
        Response::try_from(&[], &mut writer).unwrap();
    }

    #[test]
    fn res_status_code() {
        let code: StatusCode = StatusCode::from_u16(200).unwrap();
        let mut writer = Vec::new();
        let res = Response::try_from(RESPONSE, &mut writer).unwrap();

        assert_eq!(res.status_code(), code);
    }

    #[test]
    fn res_version() {
        let mut writer = Vec::new();
        let res = Response::try_from(RESPONSE, &mut writer).unwrap();

        assert_eq!(&res.version().to_string(), "HTTP/1.1");
    }

    #[test]
    fn res_reason() {
        let mut writer = Vec::new();
        let res = Response::try_from(RESPONSE, &mut writer).unwrap();

        assert_eq!(res.reason(), "OK");
    }

    #[test]
    fn res_headers() {
        let mut writer = Vec::new();
        let res = Response::try_from(RESPONSE, &mut writer).unwrap();

        let mut headers = Headers::with_capacity(2);
        headers.insert("Date", "Sat, 11 Jan 2003 02:44:04 GMT");
        headers.insert("Content-Type", "text/html");
        headers.insert("Content-Length", "100");

        assert_eq!(res.headers(), &headers);
    }

    #[test]
    fn res_content_len() {
        let mut writer = Vec::with_capacity(101);
        let res = Response::try_from(RESPONSE, &mut writer).unwrap();

        assert_eq!(res.content_len(), Some(100));
    }

    #[test]
    fn res_body() {
        let mut writer = Vec::new();
        Response::try_from(RESPONSE, &mut writer).unwrap();

        assert_eq!(writer, BODY);
    }

    #[tokio::test]
    async fn res_status_code_200() {
        let path = "/foo";
        let server = MockServer::start_async().await;
        let mock = server
            .mock_async(|when, then| {
                when.method(GET).path(path);
                then.status(200)
                    .header("content-type", "text/html; charset=UTF-8")
                    .body("GET");
            })
            .await;
        let url = server.url(path);
        let mut client = Client::builder().get(&url).build().await.unwrap();
        let response = client.send().await.unwrap();
        assert_eq!(response.status_code().as_u16(), 200);
        mock.assert_async().await;
    }

    #[tokio::test]
    async fn res_status_code_302() {
        let redirect_path = "/redirectPath";
        let final_path = "/finalPath";

        let redirect_server = MockServer::start_async().await;
        let final_server = MockServer::start_async().await;

        let redirect_url = redirect_server.url(redirect_path);
        let final_url = final_server.url(final_path);

        let redirect_mock = redirect_server
            .mock_async(|when, then| {
                when.method(GET).path(redirect_path);
                then.status(302).header("Location", final_url);
            })
            .await;

        let final_mock = final_server
            .mock_async(|when, then| {
                when.method(GET).path(final_path);
                then.status(200)
                    .header("content-type", "text/html; charset=UTF-8")
                    .body("GET");
            })
            .await;

        let mut client = Client::builder().get(&redirect_url).build().await.unwrap();
        let response = client.send().await.unwrap();
        assert_eq!(response.status_code().as_u16(), 200);
        let body = response.text().unwrap();
        assert_eq!(&body, "GET");
        assert!(client.redirects() == 1);

        redirect_mock.assert_async().await;
        final_mock.assert_async().await;
    }
}
