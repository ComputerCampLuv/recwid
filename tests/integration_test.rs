use httpmock::MockServer;
// use reqwest::header::ACCEPT;
use serde_json::json;

#[test]
fn test_client() {
    recwid::Client::new("this", "that");
}

#[test]
fn test_search_products() {
    use recwid::api::products::search_products::response::Response;

    let server = MockServer::start();
    let mock = server.mock(|when, then| {
        when.method("GET")
            .path("/mock-id/products")
            .query_param("token", "mock-token")
            .query_param("limit", "1");
        // .header("Accept", "application/json")
        then.status(200).json_body(json!({
            "total": 1
        }));
    });
    let client = recwid::Client::new_with_base_url("mock-id", "mock-token", &server.base_url());
    let response = client.search_products().limit(1).send();

    mock.assert();
    assert_eq!(response, Response { total: 1 });
}
