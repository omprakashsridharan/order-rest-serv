use axum::{routing::post, Router};

use crate::lib::repository::product::ProductRepository;

pub mod add_product;

pub fn routes() -> Router {
    Router::new().route("/", post(add_product::handle::<ProductRepository>))
}
