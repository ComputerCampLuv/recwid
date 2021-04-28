use super::RequestBuilder;
use crate::Client;

impl Client {
    pub fn create_order(&self) -> RequestBuilder {
        RequestBuilder::default(self)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_create_order() {
        let client = Client::new("id", "token");

        assert_eq!(
            client.create_order(),
            RequestBuilder::default(&client),
        );
    }
}