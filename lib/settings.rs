use std::{env, fmt};

use config::{Config, ConfigError, Environment, File};
use serde::Deserialize;

#[derive(Debug, Deserialize, Clone)]
pub struct Log {
    pub level: String,
}

#[derive(Debug, Deserialize, Clone)]
pub struct Jwt {
    pub secret: String,
}

#[derive(Debug, Deserialize, Clone)]
pub struct Gateway {
    pub port: i32,
}

#[derive(Debug, Deserialize, Clone)]
pub struct Auth {
    pub db_url: String,
}

#[derive(Debug, Deserialize, Clone)]
pub struct Settings {
    pub jwt: Jwt,
    pub gateway: Gateway,
    pub auth: Auth,
    pub log: Log,
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
        let run_mode = env::var("RUN_MODE").unwrap_or_else(|_| "Development".into());

        let s = Config::builder()
            .add_source(File::with_name("config/Default"))
            .add_source(File::with_name(&format!("config/{}", run_mode)).required(false))
            .add_source(File::with_name("config/Local").required(false))
            .add_source(Environment::with_prefix("order"))
            .set_override("env", run_mode.clone())?
            .build()?;
        s.try_deserialize()
    }
}

lazy_static! {
    pub static ref CONFIG: Settings = Settings::new().expect("config can be loaded");
}
