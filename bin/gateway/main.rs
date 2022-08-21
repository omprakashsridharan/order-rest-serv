use axum::{
    extract::Extension,
    http::{uri::Uri, HeaderValue, Request, Response},
    middleware::from_extractor,
    routing::get,
    Router,
};
use axum_casbin_auth::{
    casbin::{CoreApi, Enforcer},
    CasbinAuthLayer,
};
use hyper::{client::HttpConnector, Body};
use lib::settings;
use lib::utils::jwt::Claims;
use std::net::SocketAddr;
use std::sync::Arc;
use tokio::sync::RwLock;
use tower_http::trace::TraceLayer;
use tracing::info;

type Client = hyper::client::Client<HttpConnector, Body>;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv::dotenv().ok();
    tracing_subscriber::fmt::init();
    let client = Client::new();

    let proxy = |service: String| {
        return move |client, mut req: Request<Body>| {
            req.headers_mut().append(
                "service",
                HeaderValue::from_str(service.clone().as_str()).unwrap(),
            );
            handler(client, req)
        };
    };
    let auth_handler = get(proxy(String::from("authserv"))).post(proxy(String::from("authserv")));
    let inventory_handler =
        get(proxy(String::from("inventoryserv"))).post(proxy(String::from("inventoryserv")));

    let e = Enforcer::new("casbin/model.conf", "casbin/policy.csv").await?;
    let casbin_auth_enforcer = Arc::new(RwLock::new(e));

    let app = Router::new()
        .route("/user/*path", auth_handler.clone())
        .route("/auth/*path", auth_handler.clone())
        .route("/inventory", inventory_handler.clone())
        .layer(Extension(client))
        .layer(CasbinAuthLayer::new(casbin_auth_enforcer))
        .layer(from_extractor::<Claims>())
        .layer(TraceLayer::new_for_http());

    let addr = SocketAddr::from(([0, 0, 0, 0], settings::CONFIG.clone().gateway.port));
    info!("reverse proxy listening on {}", addr);
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
    Ok(())
}

async fn handler(Extension(client): Extension<Client>, mut req: Request<Body>) -> Response<Body> {
    let path = req.uri().path();
    let service_header = req.headers().get("service").unwrap();
    let path_query = req
        .uri()
        .path_and_query()
        .map(|v| v.as_str())
        .unwrap_or(path);

    let uri = format!("http://{}{}", service_header.to_str().unwrap(), path_query);
    *req.uri_mut() = Uri::try_from(uri).unwrap();

    client.request(req).await.unwrap()
}
