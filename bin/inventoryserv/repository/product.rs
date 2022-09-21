use crate::entity::product;
use lib::db::connection::{DatabaseConnection, DbErr};
use lib::db::prelude::*;
use std::sync::Arc;
use tracing::info;

#[derive(Clone)]
pub struct ProductRepository {
    pub db_pool: Arc<DatabaseConnection>,
}

impl ProductRepository {
    pub fn new(db_pool: Arc<DatabaseConnection>) -> Self {
        ProductRepository { db_pool }
    }

    pub async fn add_product(
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
}
