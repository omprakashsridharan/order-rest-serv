use crate::handler::add_product;
use axum::{routing::post, Extension, Router};

use lib::settings;
use lib::utils::init::initialise;
use migration::InventoryhMigrator as Migrator;
use repository::product::ProductRepository;
use std::net::SocketAddr;
use tower_http::trace::TraceLayer;
use tracing::info;
mod entity;
mod handler;
mod repository;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let db_pool = initialise::<Migrator>(settings::CONFIG.clone().inventory.db_url.clone()).await?;
    let product_repository = ProductRepository::new(db_pool.clone());

    let app = Router::new()
        .route("/inventory", post(add_product::handle))
        .layer(TraceLayer::new_for_http())
        .layer(Extension(product_repository));

    let addr = SocketAddr::from(([0, 0, 0, 0], settings::CONFIG.clone().inventory.port));
    info!("inventory serv listening on {}", addr);
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
    Ok(())
}
