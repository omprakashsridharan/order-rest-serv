use crate::constants::BEARER;
use axum::{Extension, Json};
use tracing::error;

use crate::{
    dto::{LoginInput, TokenPayload},
    error::{ApiResult, Error},
    utils::jwt::{sign, validate_payload},
};

use crate::repository::auth::AuthRepository;

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
