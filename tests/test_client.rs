use mockito::{mock, server_url};
use netc::{
    client_builder::{delete, get},
    Client, StatusCode,
};

// const ACCEPT: &str = "accept";
// const ACCEPT_JSON: &str = "application/json";

#[tokio::test]
async fn test_delete_client() {
    let path = "/bar";
    let server = mock("DELETE", path).with_status(201).create();

    let mut client = delete(&format!("{}{path}", server_url()))
        .unwrap()
        .build()
        .await
        .unwrap();
    let response = client.send().await.unwrap();
    assert_eq!(response.status_code().as_u16(), 201);
    let body = response.body();
    assert!(body.is_empty());
    server.assert();
}

#[tokio::test]
async fn test_get_client() {
    let path = "/foo";
    let server = mock("GET", path).with_status(200).create();

    let mut client = get(&format!("{}{path}", server_url()))
        .unwrap()
        .content_type("test/type")
        .build()
        .await
        .unwrap();
    let response = client.send().await.unwrap();
    assert_eq!(response.status_code().as_u16(), 200);
    let body = response.body();
    assert!(body.is_empty());
    server.assert();
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

    let server = mock("GET", path.as_str())
        .with_body(test_var)
        .with_status(200)
        .create();
    let mut client = Client::builder()
        .get(&format!("{}{path}", server_url()))
        .proxy(&proxy)
        .build()
        .await
        .unwrap();
    let response = client.send().await.unwrap();
    assert!(response.status_code().is_success());
    assert_eq!(&response.text().unwrap(), test_var);
    server.assert();
}

#[tokio::test]
async fn client_http_proxy_auth() {
    let test_var = "TEST_HTTP_AUTH_PROXY";
    let path = format!("/{test_var}");
    dotenv::dotenv().ok();
    let proxy = match dotenv::var(test_var) {
        Ok(it) => it,
        _ => return,
    };

    let server = mock("GET", path.as_str())
        .with_body(test_var)
        .with_status(200)
        .create();
    let mut client = Client::builder()
        .get(&format!("{}{path}", server_url()))
        .proxy(&proxy)
        .build()
        .await
        .unwrap();
    let response = client.send().await.unwrap();
    assert!(response.status_code().is_success());
    assert_eq!(&response.text().unwrap(), test_var);
    server.assert();
}

#[tokio::test]
async fn client_http_proxy_auth_err() {
    let test_var = "TEST_HTTP_AUTH_ERR_PROXY";
    let path = format!("/{test_var}");
    dotenv::dotenv().ok();
    let proxy = match dotenv::var(test_var) {
        Ok(it) => it,
        _ => return,
    };

    let client = Client::builder()
        .get(&format!("{}{path}", server_url()))
        .proxy(&proxy)
        .build()
        .await;
    assert!(client.is_err());
}

#[tokio::test]
async fn test_client_socks_proxy() {
    let test_var = "TEST_SOCKS5_PROXY";
    let path = format!("/{test_var}");
    dotenv::dotenv().ok();
    let proxy = match dotenv::var(test_var) {
        Ok(it) => it,
        _ => return,
    };

    let server = mock("GET", path.as_str())
        .with_body(test_var)
        .with_status(200)
        .create();
    let mut client = Client::builder()
        .get(&format!("{}{path}", server_url()))
        .proxy(&proxy)
        .build()
        .await
        .unwrap();
    let response = client.send().await.unwrap();
    assert!(response.status_code().is_success());
    assert_eq!(&response.text().unwrap(), test_var);
    server.assert();
}

#[tokio::test]
async fn client_socks_proxy() {
    dotenv::dotenv().ok();
    let socks5_proxy = match dotenv::var("TEST_SOCKS5_PROXY") {
        Ok(it) => it,
        _ => return,
    };
    const SIMPLE_URL: &'static str = "http://api.ipify.org";
    let mut client = Client::builder()
        .get(SIMPLE_URL)
        .proxy(&socks5_proxy)
        .build()
        .await
        .unwrap();
    let response = client.send().await.unwrap();
    assert!(response.status_code().is_success());
    let body = response.text().unwrap();
    dbg!(body);
}

#[tokio::test]
async fn client_socks_proxy_auth() {
    let test_var = "TEST_SOCKS5_AUTH_PROXY";
    let path = format!("/{test_var}");
    dotenv::dotenv().ok();
    let proxy = match dotenv::var(test_var) {
        Ok(it) => it,
        _ => return,
    };
    dbg!(&path);
    let server = mock("GET", path.as_str())
        .with_body(test_var)
        .with_status(200)
        .create();
    dbg!(&server);
    let mut client = Client::builder()
        // .get("http://api.ipify.org")
        .get(&format!("{}{path}", server_url()))
        .proxy(&proxy)
        .build()
        .await
        .unwrap();
    dbg!(&server_url());
    dbg!(&client);
    let response = client.send().await.unwrap();
    dbg!(&response);
    assert_eq!(response.status_code(), StatusCode::from(200));
    // assert_eq!(&response.text().unwrap(), test_var);
    // server.assert();
}

#[tokio::test]
async fn client_socks_proxy_auth_err() {
    let test_var = "TEST_SOCKS5_AUTH_ERR_PROXY";
    let path = format!("/{test_var}");
    dotenv::dotenv().ok();
    let proxy = match dotenv::var(test_var) {
        Ok(it) => it,
        _ => return,
    };

    let client = Client::builder()
        .get(&format!("{}{path}", server_url()))
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
