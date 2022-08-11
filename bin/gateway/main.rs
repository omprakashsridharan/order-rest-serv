use axum::{
    extract::Extension,
    http::{uri::Uri, HeaderValue, Request, Response},
    routing::get,
    Router,
};
use hyper::{client::HttpConnector, Body};
use std::net::SocketAddr;

type Client = hyper::client::Client<HttpConnector, Body>;

#[tokio::main]
async fn main() {
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

    let app = Router::new()
        .route("/user/*path", auth_handler.clone())
        .route("/auth/*path", auth_handler.clone())
        .layer(Extension(client));

    let addr = SocketAddr::from(([0, 0, 0, 0], 80));
    println!("reverse proxy listening on {}", addr);
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
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
    println!("Uri {:?}", uri);
    *req.uri_mut() = Uri::try_from(uri).unwrap();

    client.request(req).await.unwrap()
}