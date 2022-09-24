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
            Error::AddProductToCartError
        })?;
    bus.clone()
        .publish_event(CreateOrderEvent {
            order_request_id: order_request_id.to_string(),
            user_id,
        })
        .map_err(|e| {
            error!("Error while publish CreateOrderEvent {}", e);
            Error::AddProductError
        })?;
    info!("Order placed successfully {}", order_request_id);
    Ok((StatusCode::OK, String::from("Checkout successfull")))
}
