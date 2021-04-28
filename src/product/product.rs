use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct Product {
    pub id: u64,
    pub sku: String,
    pub name: String,
    pub price: f64,
    pub in_stock: bool,
    pub enabled: bool,
    pub is_gift_card: bool,
}
