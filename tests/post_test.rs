use httptest::{
    matchers::{all_of, contains, request},
    responders::status_code,
    Expectation, Server,
};
use netc::client_builder::{delete, get};

const ACCEPT: &str = "accept";
const ACCEPT_JSON: &str = "application/json";

#[tokio::test]
async fn test_delete_client() {
    let server = Server::run();
    server.expect(
        Expectation::matching(all_of![
            request::method("DELETE"),
            request::path("/bar"),
            request::headers(contains(("accept", "application/json"))),
        ])
        .respond_with(status_code(200)),
    );
    let url = server.url("/bar").to_string();
    let mut client = delete(&url)
        .unwrap()
        .header(ACCEPT, ACCEPT_JSON)
        .build()
        .await
        .unwrap();
    let response = client.send().await.unwrap();
    let body = response.body();
    assert!(body.is_empty());
    assert_eq!(response.status_code().as_u16(), 200);
}

#[tokio::test]
async fn test_get_client() {
    let server = Server::run();
    server.expect(
        Expectation::matching(all_of![
            request::method("GET"),
            request::path("/foo"),
            request::headers(contains(("content-type", "test/type"))),
        ])
        .respond_with(status_code(200)),
    );
    let url = server.url("/foo").to_string();
    let client_builder = get(&url).unwrap().content_type("test/type");
    let mut client = client_builder.build().await.unwrap();
    let response = client.send().await.unwrap();
    assert_eq!(response.status_code().as_u16(), 200);
}
