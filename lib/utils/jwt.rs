use crate::constants::JWT_SECRET;
use crate::error::Result as ErrorResult;
use async_trait::async_trait;
use axum::extract::{FromRequest, RequestParts, TypedHeader};
use axum::headers::{authorization::Bearer, Authorization};
use axum::response::IntoResponse;
use axum::Json;
use chrono::{Duration, Utc};
use hyper::StatusCode;
use jsonwebtoken::{DecodingKey, EncodingKey, Header, Validation};
use serde::{Deserialize, Serialize};
use serde_json::json;
use validator::Validate;

pub fn validate_payload<T: Validate>(payload: &T) -> ErrorResult<()> {
    Ok(payload.validate()?)
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Claims {
    pub sub: String,
    pub exp: i64,
    pub iat: i64,
    pub user_id: i32,
    pub role: String,
}

impl Claims {
    pub fn new(email: String, user_id: i32, role: String) -> Self {
        let iat = Utc::now();
        let exp = iat + Duration::hours(24);

        Self {
            sub: email,
            iat: iat.timestamp(),
            exp: exp.timestamp(),
            user_id,
            role,
        }
    }
}

pub fn sign(email: String, user_id: i32, role: String) -> ErrorResult<String> {
    Ok(jsonwebtoken::encode(
        &Header::default(),
        &Claims::new(email, user_id, role),
        &EncodingKey::from_secret(JWT_SECRET.as_bytes()),
    )?)
}

pub fn verify(token: &str) -> ErrorResult<Claims> {
    Ok(jsonwebtoken::decode(
        token,
        &DecodingKey::from_secret(JWT_SECRET.as_bytes()),
        &Validation::default(),
    )
    .map(|data| data.claims)?)
}

#[derive(Debug)]
pub enum AuthError {
    WrongCredentials,
    MissingCredentials,
    TokenCreation,
    InvalidToken,
}

#[async_trait]
impl<B> FromRequest<B> for Claims
where
    B: Send,
{
    type Rejection = AuthError;

    async fn from_request(req: &mut RequestParts<B>) -> Result<Self, Self::Rejection> {
        let TypedHeader(Authorization(bearer)) =
            TypedHeader::<Authorization<Bearer>>::from_request(req)
                .await
                .map_err(|_| AuthError::InvalidToken)?;
        // Decode the user data
        let token_data = verify(bearer.token()).map_err(|_| AuthError::InvalidToken)?;

        Ok(token_data)
    }
}

impl IntoResponse for AuthError {
    fn into_response(self) -> axum::response::Response {
        let (status, error_message) = match self {
            AuthError::WrongCredentials => (StatusCode::UNAUTHORIZED, "Wrong credentials"),
            AuthError::MissingCredentials => (StatusCode::BAD_REQUEST, "Missing credentials"),
            AuthError::TokenCreation => (StatusCode::INTERNAL_SERVER_ERROR, "Token creation error"),
            AuthError::InvalidToken => (StatusCode::BAD_REQUEST, "Invalid token"),
        };
        let body = Json(json!({
            "error": error_message,
        }));
        (status, body).into_response()
    }
}
