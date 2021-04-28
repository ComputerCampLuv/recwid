use crate::dispatchable::{Dispatchable, Method};
use std::collections::HashMap;

pub struct Request {
    body: HashMap<String, String>,
}

impl Request {
    pub fn new() -> Self {
        Request {
            body: HashMap::new(),
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

    fn body(&self) -> &HashMap<String, String> {
        &self.body
    }
}
