use mockito::{mock, server_url};
use netc::client_builder::{delete, get};

// const ACCEPT: &str = "accept";
// const ACCEPT_JSON: &str = "application/json";

#[tokio::test]
async fn test_delete_client() {
    let server = mock("DELETE", "/bar").with_status(201).create();

    let mut client = delete(&format!("{}/bar", server_url()))
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
    let server = mock("GET", "/foo").with_body("body").with_status(200).create();

    let mut client = get(&format!("{}/foo", server_url())).unwrap().content_type("test/type").build().await.unwrap();
    let response = client.send().await.unwrap();
    assert_eq!(response.status_code().as_u16(), 200);
    let body = response.text().unwrap();
    assert_eq!(&body, "body");
    server.assert();
}
