use crate::constants::BEARER;
use crate::{
    dto::{RegisterInput, TokenPayload},
    error::{ApiResult, Error},
    repository::auth::AuthRepository,
    utils::jwt::{sign, validate_payload},
};
use axum::{http::StatusCode, Extension, Json};
use tracing::error;

pub async fn handle(
    Json(input): Json<RegisterInput>,
    Extension(auth_repository): Extension<AuthRepository>,
) -> ApiResult<(StatusCode, Json<TokenPayload>)> {
    validate_payload(&input)?;
    let user_model = auth_repository
        .signup(input.email, input.password, input.address, input.phone)
        .await
        .map_err(|e| {
            error!("Error while signing up {}", e);
            Error::SignupError
        })?;
    let token = sign(
        user_model.email.unwrap(),
        user_model.id.unwrap(),
        user_model.role.unwrap(),
    )?;
    Ok((
        StatusCode::CREATED,
        Json(TokenPayload {
            access_token: token,
            token_type: BEARER.to_string(),
        }),
    ))
}
