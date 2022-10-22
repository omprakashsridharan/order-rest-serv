use axum::{routing::post, Router};

pub mod login;
pub mod signup;

pub fn routes() -> Router {
    Router::new()
        .route("/login", post(login::handle))
        .route("/signup", post(signup::handle))
}
