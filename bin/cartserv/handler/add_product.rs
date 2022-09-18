use axum::{Extension, Json};

use hyper::StatusCode;
use lib::{dto::AddProductData, error::ApiResult, utils::jwt::validate_payload};

use crate::repository::cart::CartRepository;

pub async fn handle(
    Json(input): Json<AddProductData>,
    Extension(cart_repository): Extension<CartRepository>,
) -> ApiResult<(StatusCode, String)> {
    validate_payload(&input)?;
    Ok((
        StatusCode::CREATED,
        String::from("product added successfully"),
    ))
}
