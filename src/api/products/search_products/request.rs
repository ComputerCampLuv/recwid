use crate::dispatchable::{Dispatchable, Method};

pub struct Request {
    query_string: String,
}

impl Request {
    pub fn new(query: &Vec<String>) -> Self {
        let query_string = query.join("&");

        Request { query_string }
    }
}

impl Dispatchable for Request {
    fn path(&self) -> String {
        format!("products?{}", self.query_string)
    }

    fn method(&self) -> Method {
        Method::Get
    }
}
