use crate::dispatchable::{Dispatchable, Method};
use std::collections::HashMap;

pub struct Request {
    query_string: String,
    body: HashMap<String, String>,
}

impl Request {
    pub fn new(query: &Vec<String>) -> Self {
        let query_string = query.join("&");

        Request {
            query_string,
            body: HashMap::new(),
        }
    }
}

impl Dispatchable for Request {
    fn path(&self) -> String {
        format!("products?{}", self.query_string)
    }

    fn method(&self) -> Method {
        Method::Get
    }

    fn body(&self) -> &HashMap<String, String> {
        &self.body
    }
}
