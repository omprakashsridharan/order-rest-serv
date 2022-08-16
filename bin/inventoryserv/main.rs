use crate::handler::add_product;
use axum::{middleware::from_extractor, routing::post, Extension, Router};
use axum_casbin_auth::{
    casbin::{CoreApi, Enforcer},
    CasbinAuthLayer,
};
use lib::{
    settings::{self},
    utils::jwt::Claims,
};
use migration::{InventoryhMigrator as Migrator, MigratorTrait};
use repository::product::ProductRepository;
use std::net::SocketAddr;
use std::sync::Arc;
use tokio::sync::RwLock;
use tower_http::trace::TraceLayer;
use tracing::info;

mod entity;
mod handler;
mod repository;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv::dotenv().ok();
    tracing_subscriber::fmt::init();
    info!(
        "DB url {}",
        settings::CONFIG.clone().inventory.db_url.clone()
    );
    let connection = sea_orm::Database::connect(&settings::CONFIG.clone().inventory.db_url).await?;
    Migrator::up(&connection, None).await?;
    let product_repository = ProductRepository::new(connection.clone());

    let e = Enforcer::new("casbin/model.conf", "casbin/policy.csv").await?;
    let casbin_auth_enforcer = Arc::new(RwLock::new(e));

    let app = Router::new()
        .route("/inventory", post(add_product::handle))
        .layer(TraceLayer::new_for_http())
        .layer(Extension(product_repository))
        .layer(CasbinAuthLayer::new(casbin_auth_enforcer))
        .layer(from_extractor::<Claims>());

    let addr = SocketAddr::from(([0, 0, 0, 0], settings::CONFIG.clone().inventory.port));
    info!("inventory serv listening on {}", addr);
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
    Ok(())
}
