use axum::{Extension, Json};

use hyper::StatusCode;
use lib::{
    dto::AddProductData,
    error::{ApiResult, Error},
    utils::jwt::validate_payload,
};
use tracing::info;

use crate::repository::product::ProductRepository;

pub async fn handle(
    Json(input): Json<AddProductData>,
    Extension(product_repository): Extension<ProductRepository>,
) -> ApiResult<(StatusCode, String)> {
    info!("Add product request received");
    validate_payload(&input)?;
    product_repository
        .add_product(input.name, input.description, input.price)
        .await
        .map_err(|_| Error::AddProductError)?;
    Ok((
        StatusCode::CREATED,
        String::from("product added successfully"),
    ))
}
