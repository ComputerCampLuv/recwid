use super::Request;
use super::Response;
use crate::Client;

pub struct RequestBuilder<'c> {
    query: Vec<(String, String)>,
    client: &'c Client,
}

impl<'c> RequestBuilder<'c> {
    pub fn default(client: &'c Client) -> Self {
        RequestBuilder {
            query: Vec::new(),
            client,
        }
    }

    pub fn limit(&mut self, limit: u32) -> &mut Self {
        self.query.push(("limit".to_owned(), limit.to_string()));
        self
    }

    pub fn in_stock(&mut self, in_stock: bool) -> &mut Self {
        self.query.push(("inStock".to_owned(), in_stock.to_string()));
        self
    }

    pub fn enabled(&mut self, enabled: bool) -> &mut Self {
        self.query.push(("enabled".to_owned(), enabled.to_string()));
        self
    }

    pub fn send(&self) -> Response {
        self.client.dispatch(self.build())
    }

    fn build(&self) -> Request {
        Request::new(self.query.clone())
    }
}
