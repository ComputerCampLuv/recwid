pub enum Method {
    Get,
    Post,
}

pub trait Dispatchable {
    fn path(&self) -> String;

    fn method(&self) -> Method;

    fn body(&self) -> &serde_json::Value;

    fn query(&self) -> &Vec<(String, String)>;
}
