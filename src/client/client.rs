use crate::dispatchable::{Dispatchable, Method};
use reqwest::blocking::{Client as Connection, Response};
use reqwest::header::{ACCEPT, CONTENT_TYPE};
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
            Method::Post => self.post(request),
        };
        println!("RESPONSE: {:?}", response);
        response.json::<T>().unwrap()
    }

    fn post(&self, request: impl Dispatchable) -> Response {
        let url = format!("{}/{}/{}", self.base_url, self.id, request.path());

        println!("URL: {}", url);
        println!("BODY: {}", request.body());

        self.conn
            .post(url)
            .header(ACCEPT, "application/json")
            .header(CONTENT_TYPE, "application/json")
            .query(&[("token", &self.token)])
            .json(&request.body())
            .send()
            .unwrap()
    }

    fn get(&self, request: impl Dispatchable) -> Response {
        let url = format!("{}/{}/{}", self.base_url, self.id, request.path());

        self.conn
            .get(url)
            .header(ACCEPT, "application/json")
            .query(&[("token", &self.token)])
            .query(&request.query())
            .send()
            .unwrap()
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
