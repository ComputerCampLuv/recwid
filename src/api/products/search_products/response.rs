use crate::product::Product;
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct Response {
    pub total: u32,
    pub count: u32,
    pub offset: u32,
    pub limit: u32,
    pub items: Vec<Product>,
}
