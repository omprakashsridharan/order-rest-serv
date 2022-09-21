use crate::repository::cart::CartRepository;
use axum::{Extension, Json};

use hyper::StatusCode;
use lib::{
    clients::Clients,
    dto::AddCartProductData,
    error::{ApiError, ApiResult, Error},
    utils::jwt::{validate_payload, Claims},
};
use tracing::{error, info};

pub async fn handle(
    Json(input): Json<AddCartProductData>,
    claims: Claims,
    Extension(cart_repository): Extension<CartRepository>,
    Extension(clients): Extension<Clients>,
) -> ApiResult<(StatusCode, String)> {
    validate_payload(&input)?;
    info!("Add product to cart request received");
    let product_id = input.product_id;

    let user_id = claims.user_id;
    let is_product_already_in_cart = cart_repository
        .is_product_already_in_cart(user_id, product_id)
        .await
        .map_err(|e| {
            error!("Error while adding product to cart: {e}");
            Error::AddProductToCartError
        })?;

    if !is_product_already_in_cart {
        let product_details = clients
            .inventory_client
            .get_product_details(product_id)
            .await
            .map_err(|e| {
                error!("Error while getting product details: {e}");
                Error::AddProductToCartError
            })?
            .ok_or(Error::AddProductToCartError)
            .unwrap();
        info!("{} added to cart", product_details.name);
        cart_repository
            .add_product(user_id, product_id)
            .await
            .map_err(|e| {
                error!("Error while adding product to cart: {e}");
                Error::AddProductToCartError
            })
            .unwrap();
        Ok((
            StatusCode::CREATED,
            String::from("product added successfully"),
        ))
    } else {
        error!("Product already in cart");
        Err(ApiError::from(Error::ProductAlreadyInCartError))
    }
}
