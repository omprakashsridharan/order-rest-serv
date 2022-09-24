use axum::{routing::post, Extension, Router};

use lib::clients::ApiClient;
use lib::{clients::get_clients, settings, utils::init::initialise};
use migration::CartMigrator as Migrator;
use std::net::SocketAddr;
use tower_http::trace::TraceLayer;
use tracing::info;

use crate::handler::{add_product, checkout};
use crate::repository::cart::CartRepository;

mod entity;
mod handler;
mod repository;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let db_pool = initialise::<Migrator>(settings::CONFIG.clone().cart.db_url.clone()).await?;
    let cart_repository = CartRepository {
        db_pool: db_pool.clone(),
    };
    let clients = get_clients();

    let app = Router::new()
        .route("/cart/checkout", post(checkout::handle::<CartRepository>))
        .route(
            "/cart",
            post(add_product::handle::<ApiClient, CartRepository>),
        )
        .layer(TraceLayer::new_for_http())
        .layer(Extension(cart_repository))
        .layer(Extension(clients));

    let addr = SocketAddr::from(([0, 0, 0, 0], settings::CONFIG.clone().cart.port));
    info!("cart serv listening on {}", addr);
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
    Ok(())
}
