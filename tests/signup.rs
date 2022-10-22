use axum::{body::Body, http::Request};
use lib::{
    get_app,
    settings::{Db, Jwt, RabbitMq, Service, Settings},
};
use std::net::{SocketAddr, TcpListener};

pub fn get_settings() -> Settings {
    Settings {
        db: Db {
            url: String::from("sqlite::memory:"),
        },
        jwt: Jwt {
            secret: "secret".to_string(),
        },
        rabbitmq: RabbitMq {
            url: String::from("amqp://guest:guest@localhost:5672"),
        },
        service: Service { port: 8080 },
    }
}

#[tokio::test]
async fn test_signup() {
    let settings = get_settings();
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
                .uri(format!("http://{}", addr))
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    let body = hyper::body::to_bytes(response.into_body()).await.unwrap();
    assert_eq!(&body[..], b"Hello, World!");
}
