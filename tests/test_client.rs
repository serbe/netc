use httpmock::prelude::*;
use netc::Client;

#[tokio::test]
async fn test_delete_client() {
    let path = "/bar";
    let server = MockServer::start_async().await;
    let mock = server
        .mock_async(|when, then| {
            when.method(DELETE).path(path);
            then.status(201)
                .header("content-type", "text/html; charset=UTF-8")
                .body("DELETE");
        })
        .await;
    let url = server.url(path);
    let mut client = Client::builder().delete(&url).build().await.unwrap();
    let response = client.send().await.unwrap();
    assert_eq!(response.status_code().as_u16(), 201);
    let body = response.text().unwrap();
    assert_eq!(&body, "DELETE");
    mock.assert();
}

#[tokio::test]
async fn test_get_client() {
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
    let body = response.text().unwrap();
    assert_eq!(&body, "GET");
    mock.assert();
}

#[tokio::test]
async fn test_http_proxy() {
    let test_var = "TEST_HTTP_PROXY";
    let path = format!("/{test_var}");
    dotenv::dotenv().ok();
    let proxy = match dotenv::var(test_var) {
        Ok(it) => it,
        _ => return,
    };
    let server = MockServer::start_async().await;
    let mock = server
        .mock_async(|when, then| {
            when.method(GET).path(&path);
            then.status(200)
                .header("content-type", "text/html; charset=UTF-8")
                .body(test_var);
        })
        .await;
    let mut client = Client::builder()
        .get(&server.url(&path))
        .proxy(&proxy)
        .build()
        .await
        .unwrap();
    let response = client.send().await.unwrap();
    assert!(response.status_code().is_success());
    assert_eq!(&response.text().unwrap(), test_var);
    mock.assert();
}

#[tokio::test]
async fn test_http_proxy_auth() {
    let test_var = "TEST_HTTP_AUTH_PROXY";
    let path = format!("/{test_var}");
    dotenv::dotenv().ok();
    let proxy = match dotenv::var(test_var) {
        Ok(it) => it,
        _ => return,
    };
    let server = MockServer::start_async().await;
    let mock = server
        .mock_async(|when, then| {
            when.method(GET).path(&path);
            then.status(200)
                .header("content-type", "text/html; charset=UTF-8")
                .body(test_var);
        })
        .await;
    let mut client = Client::builder()
        .get(&server.url(&path))
        .proxy(&proxy)
        .build()
        .await
        .unwrap();
    let response = client.send().await.unwrap();
    assert!(response.status_code().is_success());
    assert_eq!(&response.text().unwrap(), test_var);
    mock.assert();
}

#[tokio::test]
async fn test_http_proxy_auth_err() {
    let test_var = "TEST_HTTP_AUTH_ERR_PROXY";
    let path = format!("/{test_var}");
    dotenv::dotenv().ok();
    let proxy = match dotenv::var(test_var) {
        Ok(it) => it,
        _ => return,
    };
    let server = MockServer::start_async().await;
    let mut client = Client::builder()
        .get(&server.url(path))
        .proxy(&proxy)
        .build()
        .await
        .unwrap();
    let response = client.send().await.unwrap();
    assert_eq!(response.status_code().as_u16(), 407);
}

#[tokio::test]
async fn test_socks_proxy() {
    let test_var = "TEST_SOCKS5_PROXY";
    let path = format!("/{test_var}");
    dotenv::dotenv().ok();
    let proxy = match dotenv::var(test_var) {
        Ok(it) => it,
        _ => return,
    };
    let server = MockServer::start_async().await;
    let mock = server
        .mock_async(|when, then| {
            when.method(GET).path(&path);
            then.status(200)
                .header("content-type", "text/html; charset=UTF-8")
                .body(test_var);
        })
        .await;
    let mut client = Client::builder()
        .get(&server.url(&path))
        .proxy(&proxy)
        .build()
        .await
        .unwrap();
    let response = client.send().await.unwrap();
    assert!(response.status_code().is_success());
    assert_eq!(&response.text().unwrap(), test_var);
    mock.assert();
}

#[tokio::test]
async fn test_socks_proxy_100_hits() {
    let test_var = "TEST_SOCKS5_PROXY";
    let path = format!("/{test_var}");
    dotenv::dotenv().ok();
    let proxy = match dotenv::var(test_var) {
        Ok(it) => it,
        _ => return,
    };
    let server = MockServer::start_async().await;
    let mock = server
        .mock_async(|when, then| {
            when.method(GET).path(&path);
            then.status(200)
                .header("content-type", "text/html; charset=UTF-8")
                .body(test_var);
        })
        .await;
    for _ in 0..100 {
        let mut client = Client::builder()
            .get(&server.url(&path))
            .proxy(&proxy)
            .build()
            .await
            .unwrap();
        let response = client.send().await.unwrap();
        assert!(response.status_code().is_success());
        assert_eq!(&response.text().unwrap(), test_var);
    }
    assert_eq!(100, mock.hits_async().await);
}

#[tokio::test]
async fn test_socks_proxy_auth() {
    let test_var = "TEST_SOCKS5_AUTH_PROXY";
    let path = format!("/{test_var}");
    dotenv::dotenv().ok();
    let proxy = match dotenv::var(test_var) {
        Ok(it) => it,
        _ => return,
    };
    let server = MockServer::start_async().await;
    let mock = server
        .mock_async(|when, then| {
            when.method(GET).path(&path);
            then.status(200)
                .header("content-type", "text/html; charset=UTF-8")
                .body(test_var);
        })
        .await;
    let mut client = Client::builder()
        .get(&server.url(&path))
        .proxy(&proxy)
        .build()
        .await
        .unwrap();
    let response = client.send().await.unwrap();
    assert!(response.status_code().is_success());
    assert_eq!(&response.text().unwrap(), test_var);
    mock.assert();
}

#[tokio::test]
async fn test_socks_proxy_auth_err() {
    let test_var = "TEST_SOCKS5_AUTH_ERR_PROXY";
    let path = format!("/{test_var}");
    dotenv::dotenv().ok();
    let proxy = match dotenv::var(test_var) {
        Ok(it) => it,
        _ => return,
    };
    let server = MockServer::start_async().await;
    let client = Client::builder()
        .get(&server.url(&path))
        .proxy(&proxy)
        .build()
        .await;
    assert!(client.is_err());
}

// #[tokio::test]
// async fn client_http() {
//     let mut client = Client::builder().get(SIMPLE_URL).build().await.unwrap();
//     let response = client.send().await.unwrap();
//     assert!(response.status_code().is_success());
//     let body = response.text().unwrap();
//     assert!(&body.contains(crate::tests::IP.as_str()));
// }
