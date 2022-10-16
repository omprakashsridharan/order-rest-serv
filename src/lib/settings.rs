use config::{Config, ConfigError, Environment};
use serde::Deserialize;

#[derive(Debug, Deserialize, Clone)]
pub struct Jwt {
    pub secret: String,
}

#[derive(Debug, Deserialize, Clone)]
pub struct Db {
    pub url: String,
}

#[derive(Debug, Deserialize, Clone)]
pub struct Service {
    pub port: u16,
}
#[derive(Debug, Deserialize, Clone)]
pub struct RabbitMq {
    pub url: String,
}

#[derive(Debug, Deserialize, Clone)]
pub struct Settings {
    pub jwt: Jwt,
    pub db: Db,
    pub service: Service,
    pub rabbitmq: RabbitMq,
}

impl Settings {
    pub fn new() -> Result<Self, ConfigError> {
        dotenv::dotenv().ok();

        let s = Config::builder()
            // .add_source(file)
            .add_source(Environment::default())
            .build()?;
        s.try_deserialize()
    }
}

lazy_static! {
    pub static ref CONFIG: Settings = Settings::new().expect("config can be loaded");
}
