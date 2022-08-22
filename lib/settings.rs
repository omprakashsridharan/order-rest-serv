use std::{env, fmt};

use config::{Config, ConfigError, Environment, File, FileFormat};
use serde::Deserialize;
use tracing::info;

#[derive(Debug, Deserialize, Clone)]
pub struct Jwt {
    pub secret: String,
}

#[derive(Debug, Deserialize, Clone)]
pub struct Gateway {
    pub port: u16,
}

#[derive(Debug, Deserialize, Clone)]
pub struct Auth {
    pub port: u16,
    pub db_url: String,
}

#[derive(Debug, Deserialize, Clone)]
pub struct Inventory {
    pub port: u16,
    pub db_url: String,
}

#[derive(Debug, Deserialize, Clone)]
pub struct Order {
    pub port: u16,
    pub db_url: String,
}

#[derive(Debug, Deserialize, Clone)]
pub struct Settings {
    pub jwt: Jwt,
    pub gateway: Gateway,
    pub auth: Auth,
    pub inventory: Inventory,
    pub order: Order,
    pub env: ENV,
}

#[derive(Clone, Debug, Deserialize)]
pub enum ENV {
    Development,
    Production,
}

impl fmt::Display for ENV {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            ENV::Development => write!(f, "Development"),
            ENV::Production => write!(f, "Production"),
        }
    }
}

impl From<&str> for ENV {
    fn from(env: &str) -> Self {
        match env {
            "Production" => ENV::Production,
            _ => ENV::Development,
        }
    }
}

impl Settings {
    pub fn new() -> Result<Self, ConfigError> {
        let run_mode: ENV = env::var("RUN_MODE")
            .unwrap_or_else(|_| "Development".into())
            .as_str()
            .into();
        info!("Run mode {}", run_mode);
        let file = match run_mode.clone() {
            ENV::Development => {
                File::from_str(include_str!("config/Development.toml"), FileFormat::Toml)
            }
            ENV::Production => {
                File::from_str(include_str!("config/Production.toml"), FileFormat::Toml)
            }
        };

        let s = Config::builder()
            .add_source(file)
            .add_source(Environment::with_prefix("order"))
            .set_override("env", run_mode.to_string())?
            .build()?;
        s.try_deserialize()
    }
}

lazy_static! {
    pub static ref CONFIG: Settings = Settings::new().expect("config can be loaded");
}
