use axum::{routing::post, Router};

use crate::{
    bus::RabbitBus,
    repository::{cart::CartRepository, product::ProductRepository},
};

pub mod add_product;
pub mod checkout;

pub fn routes() -> Router {
    Router::new()
        .route(
            "/checkout",
            post(checkout::handle::<CartRepository, RabbitBus>),
        )
        .route(
            "/",
            post(add_product::handle::<CartRepository, ProductRepository>),
        )
}
