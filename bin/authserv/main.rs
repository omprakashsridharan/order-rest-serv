use axum::routing::post;
use axum::{Extension, Router};
use handler::{login, signup};
use lib::settings;
use migration::{AuthMigrator as Migrator, MigratorTrait};
use repository::auth::AuthRepository;
use tower_http::trace::TraceLayer;
use std::net::SocketAddr;
use tracing::info;

mod entity;
mod handler;
mod repository;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv::dotenv().ok();
    tracing_subscriber::fmt::init();
    info!("DB url {}", settings::CONFIG.clone().auth.db_url.clone());
    let connection = sea_orm::Database::connect(&settings::CONFIG.clone().auth.db_url).await?;
    Migrator::up(&connection, None).await?;
    let auth_repository = AuthRepository::new(connection.clone());

    let app = Router::new()
        .route("/auth/login", post(login::handle))
        .route("/auth/signup", post(signup::handle))
        .layer(TraceLayer::new_for_http())
        .layer(Extension(auth_repository));

    let addr = SocketAddr::from(([0, 0, 0, 0], settings::CONFIG.clone().auth.port));
    info!("auth serv listening on {}", addr);
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
    Ok(())
}
