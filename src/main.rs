#[macro_use]
extern crate lazy_static;

mod lib;

use crate::lib::handler::auth::routes as auth_routes;
use crate::lib::handler::cart::routes as cart_routes;
use crate::lib::handler::inventory::routes as inventory_routes;
use axum::{extract::Extension, middleware::from_extractor, Router};
use axum_extra::routing::SpaRouter;
use lib::bus::get_bus;
use lib::repository::auth::AuthRepository;
use lib::repository::cart::CartRepository;
use lib::repository::product::ProductRepository;
use lib::utils::jwt::Claims;
use lib::{settings, utils::init::initialise};
use migration::Migrator;
use std::net::SocketAddr;
use tower_http::trace::TraceLayer;
use tracing::info;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let db_pool = initialise::<Migrator>(settings::CONFIG.clone().db.url.clone()).await?;
    let auth_repository = AuthRepository::new(db_pool.clone());
    let cart_repository = CartRepository {
        db_pool: db_pool.clone(),
    };
    let product_repository = ProductRepository {
        db_pool: db_pool.clone(),
    };

    let bus = get_bus(settings::CONFIG.clone().rabbitmq.url.clone()).await;

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
    let app = Router::new()
        .merge(SpaRouter::new("/assets", "build"))
        .nest("/api", api_router);

    let addr = SocketAddr::from(([0, 0, 0, 0], settings::CONFIG.clone().service.port));
    info!("order serv listening on {}", addr);
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
    Ok(())
}
