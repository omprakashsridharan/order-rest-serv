use axum::Router;

use lib::{settings, utils::init::initialise};
use migration::CarthMigrator as Migrator;
use std::net::SocketAddr;
use tower_http::trace::TraceLayer;
use tracing::info;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let connection = initialise::<Migrator>(settings::CONFIG.clone().cart.db_url.clone()).await?;

    let app = Router::new().layer(TraceLayer::new_for_http());

    let addr = SocketAddr::from(([0, 0, 0, 0], settings::CONFIG.clone().cart.port));
    info!("cart serv listening on {}", addr);
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
    Ok(())
}
