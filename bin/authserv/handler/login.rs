use axum::{Extension, Json};
use lib::constants::BEARER;
use tracing::error;

use crate::{
    dto::{LoginInput, TokenPayload},
    error::{ApiResult, Error},
    repository::auth::AuthRepository,
    utils::jwt::{sign, validate_payload},
};

pub async fn handle(
    Json(input): Json<LoginInput>,
    Extension(auth_repository): Extension<AuthRepository>,
) -> ApiResult<Json<TokenPayload>> {
    validate_payload(&input)?;
    let user_model = auth_repository
        .signin(input.email, input.password)
        .await
        .map_err(|e| {
            error!("Error while logging in {}", e);
            Error::WrongCredentials
        })?;
    let token = sign(user_model.email, user_model.id, user_model.role)?;
    Ok(Json(TokenPayload {
        access_token: token,
        token_type: BEARER.to_string(),
    }))
}
