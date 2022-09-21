use axum::routing::post;
use axum::{Extension, Router};
use handler::{login, signup};
use lib::{settings, utils::init::initialise};
use migration::{AuthMigrator as Migrator, MigratorTrait};
use repository::auth::AuthRepository;
use std::net::SocketAddr;
use tower_http::trace::TraceLayer;
use tracing::info;

mod entity;
mod handler;
mod repository;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let db_pool = initialise::<Migrator>(settings::CONFIG.clone().auth.db_url.clone()).await?;
    let auth_repository = AuthRepository::new(db_pool.clone());

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
