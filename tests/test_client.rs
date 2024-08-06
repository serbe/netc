// use httpmock::prelude::*;
use netc::Client;
use wiremock::{matchers, Mock, MockServer, ResponseTemplate};

#[tokio::test]
async fn test_delete_client() {
    let mock_server = MockServer::start().await;
    Mock::given(matchers::method("DELETE"))
        .respond_with(ResponseTemplate::new(201).set_body_string("DELETE"))
        .mount(&mock_server)
        .await;
    let mut client = Client::builder()
        .delete(&mock_server.uri())
        .build()
        .await
        .unwrap();
    let response = client.send().await.unwrap();
    assert_eq!(response.status_code().as_u16(), 201);
    let body = response.text().unwrap();
    assert_eq!(&body, "DELETE");
}

#[tokio::test]
async fn test_get_client() {
    let mock_server = MockServer::start().await;
    Mock::given(matchers::method("GET"))
        .respond_with(ResponseTemplate::new(200).set_body_string("GET"))
        .mount(&mock_server)
        .await;
    let mut client = Client::builder()
        .get(&mock_server.uri())
        .build()
        .await
        .unwrap();
    let response = client.send().await.unwrap();
    assert_eq!(response.status_code().as_u16(), 200);
    let body = response.text().unwrap();
    assert_eq!(&body, "GET");
}

#[tokio::test]
async fn test_http_proxy() {
    let test_var = "TEST_HTTP_PROXY";
    dotenv::dotenv().ok();
    let proxy = match dotenv::var(test_var) {
        Ok(it) => it,
        _ => return,
    };
    let mock_server = MockServer::start().await;
    Mock::given(matchers::method("GET"))
        .respond_with(ResponseTemplate::new(200).set_body_string(test_var))
        .mount(&mock_server)
        .await;
    let mut client = Client::builder()
        .get(&mock_server.uri())
        .proxy(&proxy)
        .build()
        .await
        .unwrap();
    let response = client.send().await.unwrap();
    assert!(response.status_code().is_success());
    assert_eq!(&response.text().unwrap(), test_var);
}

#[tokio::test]
async fn test_http_proxy_auth() {
    let test_var = "TEST_HTTP_AUTH_PROXY";
    dotenv::dotenv().ok();
    let proxy = match dotenv::var(test_var) {
        Ok(it) => it,
        _ => return,
    };
    let mock_server = MockServer::start().await;
    Mock::given(matchers::method("GET"))
        .respond_with(ResponseTemplate::new(200).set_body_string(test_var))
        .mount(&mock_server)
        .await;
    let mut client = Client::builder()
        .get(&mock_server.uri())
        .proxy(&proxy)
        .build()
        .await
        .unwrap();
    let response = client.send().await.unwrap();
    assert!(response.status_code().is_success());
    assert_eq!(&response.text().unwrap(), test_var);
}

#[tokio::test]
async fn test_http_proxy_auth_err() {
    let test_var = "TEST_HTTP_AUTH_ERR_PROXY";
    dotenv::dotenv().ok();
    let proxy = match dotenv::var(test_var) {
        Ok(it) => it,
        _ => return,
    };
    let mock_server = MockServer::start().await;
    Mock::given(matchers::method("GET"))
        .respond_with(ResponseTemplate::new(200).set_body_string(test_var))
        .mount(&mock_server)
        .await;
    let mut client = Client::builder()
        .get(&mock_server.uri())
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
    dotenv::dotenv().ok();
    let proxy = match dotenv::var(test_var) {
        Ok(it) => it,
        _ => return,
    };
    let mock_server = MockServer::start().await;
    Mock::given(matchers::method("GET"))
        .respond_with(ResponseTemplate::new(200).set_body_string(test_var))
        .mount(&mock_server)
        .await;
    let mut client = Client::builder()
        .get(&mock_server.uri())
        .proxy(&proxy)
        .build()
        .await
        .unwrap();
    let response = client.send().await.unwrap();
    assert!(response.status_code().is_success());
    assert_eq!(&response.text().unwrap(), test_var);
}

#[tokio::test]
async fn test_socks_proxy_100_hits() {
    let test_var = "TEST_SOCKS5_PROXY";
    dotenv::dotenv().ok();
    let proxy = match dotenv::var(test_var) {
        Ok(it) => it,
        _ => return,
    };
    let mock_server = MockServer::start().await;
    Mock::given(matchers::method("GET"))
        .respond_with(ResponseTemplate::new(200).set_body_string(test_var))
        .up_to_n_times(100)
        .mount(&mock_server)
        .await;
    for _ in 0..100 {
        let mut client = Client::builder()
            .get(&mock_server.uri())
            .proxy(&proxy)
            .build()
            .await
            .unwrap();
        let response = client.send().await.unwrap();
        assert!(response.status_code().is_success());
        assert_eq!(&response.text().unwrap(), test_var);
    }
    let mut client = Client::builder()
        .get(&mock_server.uri())
        .proxy(&proxy)
        .build()
        .await
        .unwrap();
    let response = client.send().await.unwrap();
    assert_eq!(response.status_code().as_u16(), 404);
}

#[tokio::test]
async fn test_socks_proxy_auth() {
    let test_var = "TEST_SOCKS5_AUTH_PROXY";
    dotenv::dotenv().ok();
    let proxy = match dotenv::var(test_var) {
        Ok(it) => it,
        _ => return,
    };
    let mock_server = MockServer::start().await;
    Mock::given(matchers::method("GET"))
        .respond_with(ResponseTemplate::new(200).set_body_string(test_var))
        .mount(&mock_server)
        .await;
    let mut client = Client::builder()
        .get(&mock_server.uri())
        .proxy(&proxy)
        .build()
        .await
        .unwrap();
    let response = client.send().await.unwrap();
    assert!(response.status_code().is_success());
    assert_eq!(&response.text().unwrap(), test_var);
}

#[tokio::test]
async fn test_socks_proxy_auth_err() {
    let test_var = "TEST_SOCKS5_AUTH_ERR_PROXY";
    dotenv::dotenv().ok();
    let proxy = match dotenv::var(test_var) {
        Ok(it) => it,
        _ => return,
    };
    let mock_server = MockServer::start().await;
    Mock::given(matchers::method("GET"))
        .respond_with(ResponseTemplate::new(200).set_body_string(test_var))
        .mount(&mock_server)
        .await;
    let client = Client::builder()
        .get(&mock_server.uri())
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
