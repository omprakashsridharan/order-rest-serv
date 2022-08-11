use axum::routing::post;
use axum::{Extension, Router};
use lib::handler::{login, signup};
use lib::migration::{Migrator, MigratorTrait};
use lib::repository::auth::AuthRepository;
use lib::settings;
use std::net::SocketAddr;
use tracing::info;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    info!("DB url {}", settings::CONFIG.clone().auth.db_url.clone());
    let connection = sea_orm::Database::connect(&settings::CONFIG.clone().auth.db_url).await?;
    Migrator::up(&connection, None).await?;
    let auth_repository = AuthRepository::new(connection.clone());

    let app = Router::new()
        .route("/auth/login", post(login::handle))
        .route("/auth/signup", post(signup::handle))
        .layer(Extension(auth_repository));

    let addr = SocketAddr::from(([0, 0, 0, 0], 80));
    println!("auth serv listening on {}", addr);
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
    Ok(())
}
