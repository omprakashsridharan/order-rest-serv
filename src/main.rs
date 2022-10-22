use lib::get_app;
use lib::settings;
use std::net::SocketAddr;
use tracing::info;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let app = get_app(settings::CONFIG.clone()).await?;

    let addr = SocketAddr::from(([0, 0, 0, 0], settings::CONFIG.clone().service.port));
    info!("order serv listening on {}", addr);
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
    Ok(())
}
