use super::RequestBuilder;
use crate::Client;

impl Client {
    pub fn search_products(&self) -> RequestBuilder {
        RequestBuilder::default(self)
    }
}
