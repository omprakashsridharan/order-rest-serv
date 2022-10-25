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

pub fn init(filename_option: Option<&str>) -> Result<Settings, ConfigError> {
    if let Some(filename) = filename_option {
        dotenv::from_filename(filename).ok();
    } else {
        dotenv::dotenv().ok();
    }

    let s = Config::builder()
        .add_source(Environment::default())
        .build()?;
    s.try_deserialize()
}

lazy_static! {
    pub static ref CONFIG: Settings = init(None).expect("config can be loaded");
}
