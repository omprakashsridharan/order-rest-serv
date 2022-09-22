use serde_derive::Deserialize;
use serde_derive::Serialize;

use crate::dto::GetProductDetailsResponse;

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Product {
    pub id: i64,
    pub name: String,
}

#[derive(Clone)]
pub struct InventoryClient {}

impl InventoryClient {
    pub async fn get_product_details(&self, product_id: i32) -> Option<GetProductDetailsResponse> {
        Some(GetProductDetailsResponse {
            product_id,
            name: String::from("x"),
        })
    }
}

#[derive(Clone)]
pub struct Clients {
    pub inventory_client: InventoryClient,
}

pub fn get_clients() -> Clients {
    let inventory_client: InventoryClient = InventoryClient {};
    Clients { inventory_client }
}
