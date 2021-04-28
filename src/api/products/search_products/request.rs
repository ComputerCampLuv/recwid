use crate::dispatchable::{Dispatchable, Method};
// use std::collections::HashMap;
use serde_json::json;
use serde_json::Value;

pub struct Request {
    body: Value,
    query: Vec<(String, String)>,
}

impl Request {
    pub fn new(query: Vec<(String, String)>) -> Self {
        Request {
            body: json!({}),
            query,
        }
    }
}

impl Dispatchable for Request {
    fn path(&self) -> String {
        String::from("products")
    }

    fn method(&self) -> Method {
        Method::Get
    }

    fn body(&self) -> &Value {
        &self.body
    }

    // fn body(&self) -> &HashMap<String, String> {
    //     &self.body
    // }

    fn query(&self) -> &Vec<(String, String)> {
        &self.query
    }
}
