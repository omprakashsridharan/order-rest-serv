#[macro_use]
extern crate lazy_static;

pub mod bus;
pub mod constants;
pub mod db;
pub mod dto;
pub mod entity;
pub mod enums;
pub mod error;
pub mod events;
pub mod handler;
pub mod repository;
pub mod settings;
pub mod utils;

use axum::{extract::Extension, middleware::from_extractor, Router};
use axum_extra::routing::SpaRouter;
use bus::get_bus;
use handler::auth::routes as auth_routes;
use handler::cart::routes as cart_routes;
use handler::inventory::routes as inventory_routes;
use migration::Migrator;
use repository::auth::AuthRepository;
use repository::cart::CartRepository;
use repository::product::ProductRepository;
use settings::Settings;
use tower_http::trace::TraceLayer;
use utils::init::initialise;
use utils::jwt::Claims;

pub async fn get_app(config: Settings) -> Result<Router, Box<dyn std::error::Error>> {
    let db_pool = initialise::<Migrator>(config.clone().db.url.clone()).await?;
    let auth_repository = AuthRepository::new(db_pool.clone());
    let cart_repository = CartRepository {
        db_pool: db_pool.clone(),
    };
    let product_repository = ProductRepository {
        db_pool: db_pool.clone(),
    };

    let bus = get_bus(config.clone().rabbitmq.url.clone()).await;

    let api_router = Router::new()
        .nest("/cart", cart_routes())
        .nest("/inventory", inventory_routes())
        .layer(from_extractor::<Claims>())
        .nest("/auth", auth_routes())
        .layer(Extension(cart_repository))
        .layer(Extension(auth_repository))
        .layer(Extension(product_repository))
        .layer(Extension(bus))
        .layer(TraceLayer::new_for_http());
    Ok(Router::new()
        .merge(SpaRouter::new("/assets", "build"))
        .nest("/api", api_router))
}
