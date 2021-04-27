pub enum Method {
    Get,
    Post,
}

pub trait Dispatchable {
    fn path(&self) -> String;

    fn method(&self) -> Method;
}
