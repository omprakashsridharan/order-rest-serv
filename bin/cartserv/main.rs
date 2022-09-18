use axum::{routing::post, Extension, Router};

use lib::{settings, utils::init::initialise};
use migration::CarthMigrator as Migrator;
use std::net::SocketAddr;
use tower_http::trace::TraceLayer;
use tracing::info;

use crate::{handler::add_product, repository::cart::CartRepository};

mod entity;
mod handler;
mod repository;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let db_pool = initialise::<Migrator>(settings::CONFIG.clone().cart.db_url.clone()).await?;
    let cart_repository = CartRepository::new(db_pool.clone());
    let app = Router::new()
        .route("/cart", post(add_product::handle))
        .layer(TraceLayer::new_for_http())
        .layer(Extension(cart_repository));

    let addr = SocketAddr::from(([0, 0, 0, 0], settings::CONFIG.clone().cart.port));
    info!("cart serv listening on {}", addr);
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
    Ok(())
}
