use axum::{body::Body, http::Request};
use hyper::{Method, StatusCode};
use lib::{
    get_app,
    settings::{self, Settings},
};
use serde_json::json;
use std::net::{SocketAddr, TcpListener};

fn initialise_settings() -> Settings {
    dotenv::from_filename(".env.test").ok();
    return settings::init(Some(".env.test")).unwrap();
}

#[tokio::test]
async fn test_signup() {
    let settings = initialise_settings();
    let listener = TcpListener::bind(
        format!("127.0.0.1:{}", settings.clone().service.port)
            .parse::<SocketAddr>()
            .unwrap(),
    )
    .unwrap();
    let addr = listener.local_addr().unwrap();

    let app = get_app(settings.clone()).await.unwrap();

    tokio::spawn(async move {
        axum::Server::from_tcp(listener)
            .unwrap()
            .serve(app.into_make_service())
            .await
            .unwrap();
    });

    let client = hyper::Client::new();
    let response = client
        .request(
            Request::builder()
                .method(Method::POST)
                .uri(format!("http://{}/api/auth/signup", addr))
                .header("content-type", "application/json")
                .body(Body::from(
                    json!({
                        "email": "test@test.com",
                        "password": "123456",
                        "address":"abc",
                        "phone":"123456789"
                    })
                    .to_string(),
                ))
                .unwrap(),
        )
        .await
        .unwrap();
    assert_eq!(response.status(), StatusCode::CREATED);
}
