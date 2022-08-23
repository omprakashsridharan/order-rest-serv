use axum::Json;

use hyper::StatusCode;
use lib::{dto::AddProductData, error::ApiResult, utils::jwt::validate_payload};

pub async fn handle(Json(input): Json<AddProductData>) -> ApiResult<(StatusCode, String)> {
    validate_payload(&input)?;
    Ok((
        StatusCode::CREATED,
        String::from("product added successfully"),
    ))
}
