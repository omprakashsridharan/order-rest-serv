use crate::repository::cart::TCartRepository;
use axum::Extension;
use hyper::StatusCode;
use lib::{
    bus::TBus,
    error::{ApiResult, Error},
    events::CreateOrderEvent,
    utils::jwt::Claims,
};
use std::sync::Arc;
use tracing::{error, info};

pub async fn handle<CR: TCartRepository, B: TBus>(
    claims: Claims,
    Extension(cart_repository): Extension<CR>,
    Extension(bus): Extension<Arc<B>>,
) -> ApiResult<(StatusCode, String)> {
    info!("Checkout cart request received");

    let user_id = claims.user_id;
    let order_request_id = cart_repository
        .lock_cart_items(user_id)
        .await
        .map_err(|e| {
            error!("Error while locking cart items: {e}");
            Error::LockProductsInCartError
        })?;
    bus.clone()
        .publish_event(CreateOrderEvent {
            order_request_id: order_request_id.to_string(),
            user_id,
        })
        .await
        .map_err(|e| {
            error!("Error while publish CreateOrderEvent {}", e);
            Error::PublishError
        })?;
    info!("Order placed successfully {}", order_request_id);
    Ok((StatusCode::OK, String::from("Checkout successfull")))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::repository::cart::MockCartRepository;
    use lib::{bus::MockRabbitBus, enums::ROLES};
    use migration::sea_orm::DbErr;
    use serde_json::Value;
    use std::sync::Arc;

    #[tokio::test]
    async fn test_checkout_failure_locking_cart_items() {
        let user_id = 1;

        let mut cart_repository = MockCartRepository::default();
        cart_repository
            .expect_lock_cart_items()
            .return_const(Err(DbErr::Exec(format!("Error while locking cart items"))));

        let claims = Claims::new(
            String::from("test@test.com"),
            user_id,
            ROLES::ADMIN.to_string(),
        );
        let mut mock_bus = MockRabbitBus::default();
        mock_bus
            .expect_publish_event()
            .return_once_st(|_: CreateOrderEvent| Ok(()));
        let bus = Extension(Arc::new(mock_bus));
        let cart_repository_extension = Extension(cart_repository);
        let result = handle(claims, cart_repository_extension, bus).await;
        assert_eq!(result.is_err(), true);
        let err = result.err().unwrap();
        assert_eq!(err.0, StatusCode::INTERNAL_SERVER_ERROR);
        let message = err.1.get("message").unwrap();
        assert_eq!(
            message,
            &Value::String(Error::LockProductsInCartError.to_string())
        );
    }

    #[tokio::test]
    async fn test_checkout_failure_publishing_event() {
        let user_id = 1;

        let mut cart_repository = MockCartRepository::default();
        cart_repository
            .expect_lock_cart_items()
            .return_const(Ok(uuid::Uuid::new_v4()));

        let claims = Claims::new(
            String::from("test@test.com"),
            user_id,
            ROLES::ADMIN.to_string(),
        );
        let mut mock_bus = MockRabbitBus::default();
        mock_bus
            .expect_publish_event()
            .return_once_st(|_: CreateOrderEvent| Err(Box::new(Error::PublishError)));
        let bus = Extension(Arc::new(mock_bus));
        let cart_repository_extension = Extension(cart_repository);
        let result = handle(claims, cart_repository_extension, bus).await;
        assert_eq!(result.is_err(), true);
        let err = result.err().unwrap();
        assert_eq!(err.0, StatusCode::INTERNAL_SERVER_ERROR);
        let message = err.1.get("message").unwrap();
        assert_eq!(message, &Value::String(Error::PublishError.to_string()));
    }

    #[tokio::test]
    async fn test_checkout_successful() {
        let user_id = 1;

        let mut cart_repository = MockCartRepository::default();
        cart_repository
            .expect_lock_cart_items()
            .return_const(Ok(uuid::Uuid::new_v4()));

        let claims = Claims::new(
            String::from("test@test.com"),
            user_id,
            ROLES::ADMIN.to_string(),
        );
        let mut mock_bus = MockRabbitBus::default();
        mock_bus
            .expect_publish_event()
            .return_once_st(|_: CreateOrderEvent| Ok(()));
        let bus = Extension(Arc::new(mock_bus));
        let cart_repository_extension = Extension(cart_repository);
        let result = handle(claims, cart_repository_extension, bus).await;
        assert_eq!(result.is_ok(), true);
        let err = result.ok().unwrap();
        assert_eq!(err.0, StatusCode::OK);
    }
}
