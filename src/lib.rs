pub mod api;
mod client;
mod dispatchable;
pub mod product;

pub use client::Client;

#[cfg(test)]
mod test {
    // use super::*;

    #[test]
    fn test() {
        assert_eq!(2 + 2, 4);
    }
}
