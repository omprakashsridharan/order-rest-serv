use crate::entity::cart;
use migration::DbErr;
use migration::Expr;
use mockall::mock;
use sea_orm::prelude::*;
use sea_orm::DatabaseConnection;
use sea_orm::Set;
use std::sync::Arc;
use uuid::Uuid;

#[axum::async_trait]
pub trait TCartRepository: Clone + Send + Sized + 'static {
    async fn add_product(&self, user_id: i32, product_id: i32) -> Result<(), DbErr>;
    async fn get_cart_items(&self, user_id: i32) -> Result<Vec<i32>, DbErr>;
    async fn lock_cart_items(&self, user_id: i32) -> Result<Uuid, DbErr>;
}
#[derive(Clone)]
pub struct CartRepository {
    pub db_pool: Arc<DatabaseConnection>,
}

#[axum::async_trait]
impl TCartRepository for CartRepository {
    async fn add_product(&self, user_id: i32, product_id: i32) -> Result<(), DbErr> {
        cart::Entity::insert(cart::ActiveModel {
            user_id: Set(user_id),
            product_id: Set(product_id),
            ..Default::default()
        })
        .exec(self.db_pool.as_ref())
        .await?;
        Ok(())
    }

    async fn get_cart_items(&self, user_id: i32) -> Result<Vec<i32>, DbErr> {
        let cart_items = cart::Entity::find()
            .filter(cart::Column::UserId.eq(user_id))
            .filter(cart::Column::OrderRequestId.is_null())
            .all(self.db_pool.as_ref())
            .await?;
        Ok(cart_items.iter().map(|c| c.product_id).collect())
    }

    async fn lock_cart_items(&self, user_id: i32) -> Result<Uuid, DbErr> {
        let order_request_id = Uuid::new_v4();
        let update_result = cart::Entity::update_many()
            .col_expr(cart::Column::OrderRequestId, Expr::value(order_request_id))
            .filter(cart::Column::UserId.eq(user_id))
            .filter(cart::Column::OrderRequestId.is_null())
            .exec(self.db_pool.as_ref())
            .await?;
        if update_result.rows_affected == 0 {
            return Err(DbErr::Exec(String::from("No cart items to update")));
        }
        return Ok(order_request_id);
    }
}

mock! {
    pub CartRepository {}

    impl Clone for CartRepository {
        fn clone(&self) -> Self;
    }

    #[axum::async_trait]
    impl TCartRepository for CartRepository {
        async fn add_product(&self, user_id: i32, product_id: i32) -> Result<(), DbErr>;
        async fn get_cart_items(&self, user_id: i32) -> Result<Vec<i32>, DbErr>;
        async fn lock_cart_items(&self, user_id: i32) -> Result<Uuid, DbErr>;
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use sea_orm::{DatabaseBackend, MockDatabase, MockExecResult};

    #[tokio::test]
    async fn test_add_product_successful() {
        let db_pool = MockDatabase::new(DatabaseBackend::MySql)
            .append_exec_results(vec![MockExecResult {
                rows_affected: 1,
                last_insert_id: 1,
            }])
            .into_connection();
        let cart_repository = CartRepository {
            db_pool: Arc::new(db_pool),
        };
        let result = cart_repository.add_product(1, 1).await;
        assert_eq!(result.is_ok(), true);
    }

    #[tokio::test]
    async fn test_lock_cart_items_empty() {
        let db_pool = MockDatabase::new(DatabaseBackend::MySql)
            .append_exec_results(vec![MockExecResult {
                rows_affected: 0,
                ..Default::default()
            }])
            .into_connection();
        let cart_repository = CartRepository {
            db_pool: Arc::new(db_pool),
        };
        let result = cart_repository.lock_cart_items(1).await;
        assert_eq!(result.is_err(), true);
    }

    #[tokio::test]
    async fn test_lock_cart_items_success() {
        let db_pool = MockDatabase::new(DatabaseBackend::MySql)
            .append_exec_results(vec![MockExecResult {
                rows_affected: 1,
                ..Default::default()
            }])
            .into_connection();
        let cart_repository = CartRepository {
            db_pool: Arc::new(db_pool),
        };
        let result = cart_repository.lock_cart_items(1).await;
        assert_eq!(result.is_ok(), true);
    }
}
