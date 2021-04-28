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
    use recwid::Product;

    let server = MockServer::start();
    let mock = server.mock(|when, then| {
        when.method("GET")
            .path("/mock-id/products")
            .header("Accept", "application/json")
            .query_param("token", "mock-token")
            .query_param("limit", "1");
        then.status(200).json_body(json!({
            "total": 5,
            "count": 1,
            "offset": 0,
            "limit": 1,
            "items": [
                {
                    "id": 101,
                    "sku": "SKU101",
                    "name": "product-name",
                    "price": 10.99,
                    "inStock": true,
                    "enabled": true,
                    "isGiftCard": false
                }
            ]
        }));
    });

    let client = recwid::Client::new_with_base_url("mock-id", "mock-token", &server.base_url());
    let actual = client.search_products().limit(1).send();
    let expected = Response {
        total: 5,
        count: 1,
        offset: 0,
        limit: 1,
        items: vec![Product {
            id: 101,
            sku: "SKU101".to_owned(),
            name: "product-name".to_owned(),
            price: 10.99,
            in_stock: true,
            enabled: true,
            is_gift_card: false,
        }],
    };

    mock.assert();
    assert_eq!(actual, expected);
}

#[test]
fn test_create_order() {
    use recwid::api::orders::create_order::response::Response;
    use recwid::Product;

    let server = MockServer::start();
    let mock = server.mock(|when, then| {
        when.method("POST")
            .path("/mock-id/orders")
            .header("Accept", "application/json")
            .header("Content-type", "application/json")
            .query_param("token", "mock-token")
            .json_body(json!({}));
        then.status(200).json_body(json!({
            "id": 101
        }));
    });

    let client = recwid::Client::new_with_base_url("mock-id", "mock-token", &server.base_url());
    let product = Product {
        id: 101,
        sku: "SKU101".to_owned(),
        name: "product-name".to_owned(),
        price: 10.99,
        in_stock: true,
        enabled: true,
        is_gift_card: false,
    };
    let actual = client.create_order().add_item(product).send();
    let expected = Response { id: 101 };

    mock.assert();
    assert_eq!(actual, expected);
}
