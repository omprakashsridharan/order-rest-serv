use axum::{Extension, Json};
use common::constants::BEARER;

use crate::{
    dto::{LoginInput, TokenPayload},
    error::{ApiResult, Error},
    repository::auth::AuthRepository,
    utils::jwt::validate_payload,
};

pub async fn handle(
    Json(input): Json<LoginInput>,
    Extension(auth_repository): Extension<AuthRepository>,
) -> ApiResult<Json<TokenPayload>> {
    validate_payload(&input)?;
    let token = auth_repository
        .signin(input.email, input.password)
        .await
        .map_err(|_| Error::WrongCredentials)?;
    Ok(Json(TokenPayload {
        access_token: token,
        token_type: BEARER.to_string(),
    }))
}
