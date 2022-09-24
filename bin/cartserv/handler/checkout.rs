use crate::repository::cart::TCartRepository;
use axum::Extension;
use hyper::StatusCode;
use lib::{
    error::{ApiResult, Error},
    utils::jwt::Claims,
};
use tracing::{error, info};

pub async fn handle<CR: TCartRepository>(
    claims: Claims,
    Extension(cart_repository): Extension<CR>,
) -> ApiResult<(StatusCode, String)> {
    info!("Checkout cart request received");

    let user_id = claims.user_id;
    cart_repository
        .lock_cart_items(user_id)
        .await
        .map_err(|e| {
            error!("Error while locking cart items: {e}");
            Error::AddProductToCartError
        })?;
    Ok((StatusCode::OK, String::from("Checkout successfull")))
}
