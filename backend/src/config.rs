use serde::{Deserialize, Serialize};
use config::{Config, ConfigError, Environment, File};
use std::env;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Settings {
    pub server: ServerSettings,
    pub database: DatabaseSettings,
    pub openai: OpenAiSettings,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ServerSettings {
    pub host: String,
    pub port: u16,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DatabaseSettings {
    pub username: String,
    pub password: String,
    pub host: String,
    pub port: u16,
    pub database_name: String,
    pub require_ssl: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OpenAiSettings {
    pub api_key: String,
    pub model: String,
    pub max_tokens: u32,
}

impl Settings {
    pub fn new() -> Result<Self, ConfigError> {
        let run_mode = env::var("RUN_MODE").unwrap_or_else(|_| "development".into());

        let s = Config::builder()
            // Start with default settings
            .add_source(File::with_name("config/default"))
            // Add environment-specific settings
            .add_source(File::with_name(&format!("config/{}", run_mode)).required(false))
            // Add local settings
            .add_source(File::with_name("config/local").required(false))
            // Add environment variables with prefix
            .add_source(Environment::with_prefix("LAUNE").separator("__"))
            .build()?;

        s.try_deserialize()
    }
}

impl DatabaseSettings {
    pub fn connection_string(&self) -> String {
        format!(
            "postgres://{}:{}@{}:{}/{}{}",
            self.username,
            self.password,
            self.host,
            self.port,
            self.database_name,
            if self.require_ssl { "?sslmode=require" } else { "" }
        )
    }
}
