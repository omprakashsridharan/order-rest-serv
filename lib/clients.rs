use serde_derive::Deserialize;
use serde_derive::Serialize;

use feign::{client, ClientResult};

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Product {
    pub id: i64,
    pub name: String,
}

#[client(host = "http://127.0.0.1:3000", path = "/inventory")]
pub trait InventoryClient {
    #[get(path = "/<id>")]
    async fn get_product_details(&self, #[path] id: i32) -> ClientResult<Option<Product>>;
}

#[derive(Clone)]
pub struct Clients {
    pub inventory_client: InventoryClient,
}

pub fn get_clients() -> Clients {
    let inventory_client: InventoryClient = InventoryClient::new();
    Clients { inventory_client }
}
