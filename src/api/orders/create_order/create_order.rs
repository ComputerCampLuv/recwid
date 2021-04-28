use super::RequestBuilder;
use crate::Client;

impl Client {
    pub fn create_order(&self) -> RequestBuilder {
        RequestBuilder::default(self)
    }
}
