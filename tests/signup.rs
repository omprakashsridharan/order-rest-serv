use axum::{body::Body, http::Request};
use lib::{
    get_app,
    settings::{Db, Jwt, RabbitMq, Service, Settings},
};
use std::net::{SocketAddr, TcpListener};
use testcontainers::{clients, images::rabbitmq};

fn initialise_settings() -> Settings {
    let docker = clients::Cli::default();
    let rabbit_node = docker.run(rabbitmq::RabbitMq::default());
    let amqp_url = format!("amqp://127.0.0.1:{}", rabbit_node.get_host_port_ipv4(5672));
    Settings {
        db: Db {
            url: String::from("sqlite::memory:"),
        },
        jwt: Jwt {
            secret: "secret".to_string(),
        },
        rabbitmq: RabbitMq {
            url: "amqp://127.0.0.1:15671".to_string(),
        },
        service: Service { port: 8080 },
    }
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
                .uri(format!("http://{}", addr))
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    let body = hyper::body::to_bytes(response.into_body()).await.unwrap();
    assert_eq!(&body[..], b"Hello, World!");
}
