use axum::Router;

use lib::settings;
use migration::{MigratorTrait, OrderhMigrator as Migrator};
use std::net::SocketAddr;
use tower_http::trace::TraceLayer;
use tracing::info;

mod handler;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv::dotenv().ok();
    tracing_subscriber::fmt::init();
    info!("DB url {}", settings::CONFIG.clone().order.db_url.clone());
    let connection = sea_orm::Database::connect(&settings::CONFIG.clone().order.db_url).await?;
    Migrator::up(&connection, None).await?;

    let app = Router::new().layer(TraceLayer::new_for_http());

    let addr = SocketAddr::from(([0, 0, 0, 0], settings::CONFIG.clone().order.port));
    info!("order serv listening on {}", addr);
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
    Ok(())
}
