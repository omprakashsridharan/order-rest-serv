use db::connection::{DatabaseConnection, DbErr};
use db::entity::product;
use db::prelude::*;
use tracing::{error, info};

pub struct ProductRepository {
    pub db_pool: DatabaseConnection,
}

impl ProductRepository {
    pub fn new(db_pool: DatabaseConnection) -> Self {
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
        .save(&self.db_pool)
        .await?;
        info!("Product added successfully");
        Ok(())
    }

    pub async fn get_product(&self, product_id: i32) -> Result<product::Model, DbErr> {
        if let Some(model) = product::Entity::find_by_id(product_id)
            .one(&self.db_pool)
            .await?
        {
            Ok(model)
        } else {
            error!("Product not found in DB");
            Err(DbErr::Custom("Product not found in DB".to_string()))
        }
    }
}
