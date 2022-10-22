use axum::{Extension, Json};

use crate::{
    dto::AddProductData,
    error::{ApiResult, Error},
    repository::product::TProductRepository,
    utils::jwt::validate_payload,
};
use hyper::StatusCode;

pub async fn handle<PR: TProductRepository>(
    Json(input): Json<AddProductData>,
    Extension(product_repository): Extension<PR>,
) -> ApiResult<(StatusCode, String)> {
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
