use super::Request;
use super::Response;
use crate::{Client, Product};
use serde_json::json;

pub struct RequestBuilder<'c> {
    client: &'c Client,
    line_items: Vec<Product>,
}

impl<'c> RequestBuilder<'c> {
    pub fn default(client: &'c Client) -> Self {
        RequestBuilder {
            client,
            line_items: Vec::new(),
        }
    }

    pub fn add_item(&mut self, product: Product) -> &mut Self {
        self.line_items.push(product);
        self
    }

    pub fn send(&self) -> Response {
        self.client.dispatch(self.build())
    }

    fn build(&self) -> Request {
        let product = self.line_items.first().unwrap();

        let body = json!({
            "subtotal": product.price,
            "total": product.price,
            "email": "plip@plop.com",
            "paymentMethod": "Pay by Cash",
            "tax": 0.0,
            "items": [
                {
                    "productId": product.id,
                    "sku": product.sku,
                    "name": product.name,
                    "price": product.price,
                    "productPrice": product.price,
                    "discountsAllowed": true,
                    "isGiftCard": product.is_gift_card,
                    "taxable": true,
                    "quantity": 1,
                    "tax": 0.0,
                    "taxes": [],
                }
            ],
            "shippingPerson": {
                "name": "Karie Hirthe",
                "firstName": "Karie",
                "lastName": "Hirthe",
                "street": "84454 Littel Rue",
                "city": "Sungview",
                "countryName": "Poland",
                "postalCode": "77019",
                "stateOrProvinceName": "Idaho",
            },
            "shippingOption": {
                "shippingMethodName": "Pick Up",
                "shippingRate": 0.0,
                "isPickUp": true,
            },
            "couponDiscount": 0.0,
            "volumeDiscount": 0.0,
            "discount": 0.0,
        });

        Request::new(body)
    }
}

#[cfg(test)]
mod test {
    #[test]
    fn test_default() {
        assert!(true);
    }
}