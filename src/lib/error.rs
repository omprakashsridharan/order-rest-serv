use axum::{http::StatusCode, Json};
use serde_json::{json, Value};
use thiserror::Error;
use tracing::log::error;

#[derive(Debug, Error)]
pub enum Error {
    #[error(transparent)]
    BcryptError(#[from] bcrypt::BcryptError),
    #[error(transparent)]
    DbError(#[from] sea_orm::error::DbErr),
    #[error(transparent)]
    JwtError(#[from] jsonwebtoken::errors::Error),
    #[error(transparent)]
    TokioRecvError(#[from] tokio::sync::oneshot::error::RecvError),
    #[error(transparent)]
    AxumExtensionError(#[from] axum::extract::rejection::ExtensionRejection),
    #[error(transparent)]
    ValidationError(#[from] validator::ValidationErrors),
    #[error("wrong credentials")]
    WrongCredentials,
    #[error("password doesn't match")]
    WrongPassword,
    #[error("email is already taken")]
    DuplicateUserEmail,
    #[error("name is already taken")]
    DuplicateUserName,
    #[error("error while signing up")]
    SignupError,
    #[error("error while adding product to the inventory")]
    AddProductError,
    #[error("error while getting product details")]
    GetProductDetailsError,
    #[error("error while adding product to the cart")]
    AddProductToCartError,
    #[error("error while locking products in cart")]
    LockProductsInCartError,
    #[error("error while publishing event")]
    PublishError,
}
pub type Result<T> = std::result::Result<T, Error>;

pub type ApiError = (StatusCode, Json<Value>);
pub type ApiResult<T> = std::result::Result<T, ApiError>;

impl From<Error> for ApiError {
    fn from(err: Error) -> Self {
        let status = match err {
            Error::WrongCredentials => StatusCode::UNAUTHORIZED,
            Error::ValidationError(_) => StatusCode::BAD_REQUEST,
            _ => StatusCode::INTERNAL_SERVER_ERROR,
        };
        let payload = match err {
            Error::ValidationError(ve) => json!(ve.errors()),
            _ => json!({"message": err.to_string()}),
        };
        (status, Json(payload))
    }
}
