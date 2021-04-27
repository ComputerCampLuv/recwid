use serde::{Deserialize, Serialize};
use crate::product::Product;

#[derive(Debug, Deserialize, Serialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct Response {
    pub total: u32,
    pub items: Vec<Product>
}
