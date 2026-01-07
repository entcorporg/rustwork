use anyhow::Result;
use config::{Config, Environment, File};
use serde::{Deserialize, Serialize};
use std::env;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AppConfig {
    pub server: ServerConfig,
    pub database: DatabaseConfig,
    pub auth: AuthConfig,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ServerConfig {
    pub host: String,
    pub port: u16,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DatabaseConfig {
    pub url: String,
    pub max_connections: u32,
    pub min_connections: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AuthConfig {
    pub jwt_secret: String,
    pub jwt_expiration: i64,
}

impl AppConfig {
    /// Charge la configuration depuis .env puis config/{profile}.toml
    /// Fallback sur config/default.toml
    pub fn load() -> Result<Self> {
        // Charge .env si présent
        dotenvy::dotenv().ok();

        // Détermine le profil (dev/test/prod)
        let profile = env::var("APP_ENV").unwrap_or_else(|_| "dev".to_string());

        // Build config
        let config = Config::builder()
            // Commence par le fichier par défaut
            .add_source(File::with_name("config/default").required(false))
            // Overlay avec le fichier profil
            .add_source(File::with_name(&format!("config/{}", profile)).required(false))
            // Override avec variables d'environnement (préfixe APP__)
            .add_source(Environment::with_prefix("APP").separator("__"))
            .build()?;

        let app_config: AppConfig = config.try_deserialize()?;

        // Validation basique
        if app_config.database.url.is_empty() {
            anyhow::bail!("Database URL is required");
        }

        Ok(app_config)
    }
}

impl Default for ServerConfig {
    fn default() -> Self {
        Self {
            host: "0.0.0.0".to_string(),
            port: 3000,
        }
    }
}

impl Default for DatabaseConfig {
    fn default() -> Self {
        Self {
            url: "postgres://postgres:postgres@localhost/rustwork".to_string(),
            max_connections: 10,
            min_connections: 2,
        }
    }
}

impl Default for AuthConfig {
    fn default() -> Self {
        Self {
            jwt_secret: "change-me-in-production".to_string(),
            jwt_expiration: 86400, // 24h
        }
    }
}

impl Default for AppConfig {
    fn default() -> Self {
        Self {
            server: ServerConfig::default(),
            database: DatabaseConfig::default(),
            auth: AuthConfig::default(),
        }
    }
}
