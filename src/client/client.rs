use crate::dispatchable::{Dispatchable, Method};
use reqwest::blocking::Client as Connection;
use reqwest::blocking::Response;
use serde::de::DeserializeOwned;
use serde::Serialize;

#[derive(Debug)]
pub struct Client {
    id: String,
    token: String,
    base_url: String,
    conn: Connection,
}

impl Client {
    pub fn new<S: Into<String>>(id: S, token: S) -> Self {
        Client {
            id: id.into(),
            token: token.into(),
            base_url: "https://app.ecwid.com/api/v3".to_owned(),
            conn: Connection::new(),
        }
    }

    pub fn new_with_base_url<S: Into<String>>(id: S, token: S, base_url: S) -> Self {
        Client {
            id: id.into(),
            token: token.into(),
            base_url: base_url.into(),
            conn: Connection::new(),
        }
    }

    pub fn dispatch<T: Serialize + DeserializeOwned>(&self, request: impl Dispatchable) -> T {
        let response = match request.method() {
            Method::Get => self.get(request),
            _ => panic!("Method not implemented!"),
        };
        response.json::<T>().unwrap()
    }

    fn get(&self, request: impl Dispatchable) -> Response {
        let url = format!(
            "{}/{}/{}&token={}",
            self.base_url,
            self.id,
            request.path(),
            self.token
        );
        self.conn.get(url).send().unwrap()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn default_with_str() {
        Client::new("mock-id", "mock-token");
    }

    #[test]
    fn default_with_string() {
        Client::new("mock-id".to_string(), "mock-token".to_string());
    }
}
