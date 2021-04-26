use reqwest::blocking::Client as Connection;

#[derive(Debug)]
pub struct Client {
    id: String,
    token: String,
    conn: Connection,
}

impl Client {
    pub fn new<S: Into<String>>(id: S, token: S) -> Self {
        Client {
            id: id.into(),
            token: token.into(),
            conn: Connection::new(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new_with_str() {
        Client::new("mock-id", "mock-token");
    }

    #[test]
    fn new_with_string() {
        Client::new("mock-id".to_string(), "mock-token".to_string());
    }
}
