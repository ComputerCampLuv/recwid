use crate::dispatchable::{Dispatchable, Method};
use serde_json::Value;


pub struct Request {
    body: Value,
    query: Vec<(String, String)>,
}

impl Request {
    pub fn new(body: Value) -> Self {
        Request {
            body,
            query: Vec::new(),
        }
    }
}

impl Dispatchable for Request {
    fn path(&self) -> String {
        String::from("orders?")
    }

    fn method(&self) -> Method {
        Method::Post
    }

    fn body(&self) -> &serde_json::Value {
        &self.body
    }

    fn query(&self) -> &Vec<(String, String)> {
        &self.query
    }
}
