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

    pub fn send(&self) -> Response {
        self.client.dispatch(self.build())
    }

    fn build(&self) -> Request {
        Request::new(&self.query)
    }
}
