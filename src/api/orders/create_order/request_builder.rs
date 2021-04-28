use super::Request;
use super::Response;
use crate::{Client, Product};

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
        Request::new()
    }
}
