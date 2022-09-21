use migration::DbErr;
use migration::Expr;
use sea_orm::prelude::*;
use sea_orm::DatabaseConnection;
use sea_orm::Set;
use uuid::Uuid;

use crate::entity::cart;

#[derive(Clone)]
pub struct CartRepository {
    pub db_pool: DatabaseConnection,
}

impl CartRepository {
    pub async fn is_product_already_in_cart(
        &self,
        user_id: i32,
        product_id: i32,
    ) -> Result<bool, DbErr> {
        if let Some(_) = cart::Entity::find()
            .filter(cart::Column::UserId.eq(user_id))
            .filter(cart::Column::ProductId.eq(product_id))
            .one(&self.db_pool)
            .await?
        {
            return Ok(true);
        }
        return Ok(false);
    }

    pub async fn add_product(&self, user_id: i32, product_id: i32) -> Result<(), DbErr> {
        cart::ActiveModel {
            user_id: Set(user_id),
            product_id: Set(product_id),
            ..Default::default()
        }
        .save(&self.db_pool)
        .await?;
        Ok(())
    }

    pub async fn get_cart_items(&self, user_id: i32) -> Result<Vec<i32>, DbErr> {
        let cart_items = cart::Entity::find()
            .filter(cart::Column::UserId.eq(user_id))
            .filter(cart::Column::OrderRequestId.is_null())
            .all(&self.db_pool)
            .await?;
        Ok(cart_items.iter().map(|c| c.product_id).collect())
    }

    pub async fn lock_cart_items(&self, user_id: i32) -> Result<Uuid, DbErr> {
        let order_request_id = Uuid::new_v4();
        cart::Entity::update_many()
            .col_expr(cart::Column::OrderRequestId, Expr::value(order_request_id))
            .filter(cart::Column::UserId.eq(user_id))
            .exec(&self.db_pool)
            .await?;
        Ok(order_request_id)
    }
}
