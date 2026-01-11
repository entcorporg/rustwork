use super::{
    cors_config::CorsConfig, database_config::DatabaseConfig, server_config::ServerConfig,
};
use anyhow::Result;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct AppConfig {
    pub server: ServerConfig,
    #[serde(alias = "database", alias = "db")]
    pub database: DatabaseConfig,
    #[serde(default)]
    pub cors: CorsConfig,
}

impl AppConfig {
    /// Charge la configuration depuis .env puis config/{profile}.toml
    pub fn load() -> Result<Self> {
        crate::config::builders::load_app_config::load_app_config()
    }
}
