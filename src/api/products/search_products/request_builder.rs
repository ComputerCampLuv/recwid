use super::Request;
use super::Response;
use crate::Client;

pub struct RequestBuilder<'c> {
    query: Vec<String>,
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
        self.query.push(format!("limit={}", limit));
        self
    }

    pub fn in_stock(&mut self, in_stock: bool) -> &mut Self {
        self.query.push(format!("inStock={}", in_stock));
        self
    }

    pub fn enabled(&mut self, enabled: bool) -> &mut Self {
        self.query.push(format!("enabled={}", enabled));
        self
    }

    pub fn send(&self) -> Response {
        self.client.dispatch(self.build())
    }

    fn build(&self) -> Request {
        Request::new(&self.query)
    }
}
