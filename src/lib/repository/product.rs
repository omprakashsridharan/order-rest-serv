use crate::db::connection::{DatabaseConnection, DbErr};
use crate::db::prelude::*;
use crate::entity::product;
use mockall::mock;
use std::sync::Arc;
use tracing::info;

#[axum::async_trait]
pub trait TProductRepository: Clone + Send + Sized + 'static {
    async fn add_product(&self, name: String, description: String, price: f32)
        -> Result<(), DbErr>;
    async fn get_details(&self, product_id: i32) -> Result<product::Model, DbErr>;
}

#[derive(Clone)]
pub struct ProductRepository {
    pub db_pool: Arc<DatabaseConnection>,
}

#[axum::async_trait]
impl TProductRepository for ProductRepository {
    async fn add_product(
        &self,
        name: String,
        description: String,
        price: f32,
    ) -> Result<(), DbErr> {
        product::ActiveModel {
            name: Set(name),
            description: Set(description),
            price: Set(price),
            ..Default::default()
        }
        .save(self.db_pool.as_ref())
        .await?;
        info!("Product added successfully");
        Ok(())
    }

    async fn get_details(&self, product_id: i32) -> Result<product::Model, DbErr> {
        info!("Get product details request received");
        product::Entity::find_by_id(product_id)
            .one(self.db_pool.as_ref())
            .await?
            .ok_or(DbErr::Query("Product not found".to_owned()))
    }
}

mock! {
    pub ProductRepository {}

    impl Clone for ProductRepository {
        fn clone(&self) -> Self;
    }

    #[axum::async_trait]
    impl TProductRepository for ProductRepository {
        async fn add_product(&self, name: String, description: String, price: f32)
        -> Result<(), DbErr>;
        async fn get_details(&self, product_id: i32) -> Result<product::Model, DbErr>;
    }
}
