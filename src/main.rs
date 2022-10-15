#[macro_use]
extern crate lazy_static;

mod lib;

use axum::routing::post;
use axum::{extract::Extension, middleware::from_extractor, Router};
use lib::bus::{get_bus, RabbitBus};
use lib::handler::auth::{login, signup};
use lib::handler::cart::{add_product as add_product_cart, checkout};
use lib::handler::inventory::add_product as add_product_inventory;
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

    let app = Router::new()
        .route(
            "/cart/checkout",
            post(checkout::handle::<CartRepository, RabbitBus>),
        )
        .route(
            "/cart",
            post(add_product_cart::handle::<CartRepository, ProductRepository>),
        )
        .route(
            "/inventory",
            post(add_product_inventory::handle::<ProductRepository>),
        )
        .layer(from_extractor::<Claims>())
        .route("/auth/login", post(login::handle))
        .route("/auth/signup", post(signup::handle))
        .layer(Extension(cart_repository))
        .layer(Extension(auth_repository))
        .layer(Extension(product_repository))
        .layer(Extension(bus))
        .layer(TraceLayer::new_for_http());

    let addr = SocketAddr::from(([0, 0, 0, 0], settings::CONFIG.clone().service.port));
    info!("auth serv listening on {}", addr);
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
    Ok(())
}
