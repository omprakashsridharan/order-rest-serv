use axum::{Extension, Router};
use lib::settings::{self};
use migration::{InventoryhMigrator as Migrator, MigratorTrait};
use repository::product::ProductRepository;
use std::net::SocketAddr;

mod entity;
mod repository;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!(
        "DB url {}",
        settings::CONFIG.clone().inventory.db_url.clone()
    );
    let connection = sea_orm::Database::connect(&settings::CONFIG.clone().inventory.db_url).await?;
    Migrator::up(&connection, None).await?;
    let product_repository = ProductRepository::new(connection.clone());

    let app = Router::new().layer(Extension(product_repository));

    let addr = SocketAddr::from(([0, 0, 0, 0], 80));
    println!("auth serv listening on {}", addr);
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
    Ok(())
}
