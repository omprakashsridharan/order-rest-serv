use crate::dto::GetProductDetailsResponse;
use mockall::mock;
use serde_derive::Deserialize;
use serde_derive::Serialize;

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Product {
    pub id: i64,
    pub name: String,
}

#[axum::async_trait]
pub trait TClient: Clone + Send + Sized + 'static {
    async fn get_product_details(&self, product_id: i32) -> Option<GetProductDetailsResponse>;
}

#[derive(Clone)]
pub struct ApiClient {}

pub fn get_clients() -> impl TClient {
    ApiClient {}
}

#[axum::async_trait]
impl TClient for ApiClient {
    async fn get_product_details(&self, product_id: i32) -> Option<GetProductDetailsResponse> {
        Some(GetProductDetailsResponse {
            product_id,
            name: String::from("x"),
        })
    }
}

mock! {
    pub Client {}

    impl Clone for Client {
        fn clone(&self) -> Self;
    }

    #[axum::async_trait]
    impl TClient for Client {
        async fn get_product_details(&self, product_id: i32) -> Option<GetProductDetailsResponse>;
    }
}
