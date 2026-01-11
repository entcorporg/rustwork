use super::db_connection::DbConnection;
use super::pool_config::PoolConfig;
use serde::{Deserialize, Serialize};

/// Configuration de la base de données avec support multi-driver
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DatabaseConfig {
    /// Type de connexion (sqlite, postgres, mysql)
    #[serde(default = "default_connection")]
    pub connection: DbConnection,

    /// URL complète (priorité la plus haute si définie)
    #[serde(default)]
    pub url: Option<String>,

    /// Chemin pour SQLite (par défaut: ./data/app.db)
    #[serde(default = "default_sqlite_path")]
    pub sqlite_path: String,

    /// Host DB (postgres/mysql)
    #[serde(default = "default_host")]
    pub host: String,

    /// Port DB (postgres/mysql)
    #[serde(default)]
    pub port: Option<u16>,

    /// Nom de la base de données
    #[serde(default)]
    pub database: Option<String>,

    /// Username
    #[serde(default)]
    pub username: Option<String>,

    /// Password
    #[serde(default)]
    pub password: Option<String>,

    /// Pool config
    #[serde(default)]
    pub pool: PoolConfig,
}

fn default_connection() -> DbConnection {
    DbConnection::Sqlite
}

fn default_sqlite_path() -> String {
    "./data/app.db".to_string()
}

fn default_host() -> String {
    "127.0.0.1".to_string()
}

impl Default for DatabaseConfig {
    fn default() -> Self {
        Self {
            connection: DbConnection::Sqlite,
            url: None,
            sqlite_path: default_sqlite_path(),
            host: default_host(),
            port: None,
            database: None,
            username: None,
            password: None,
            pool: PoolConfig::default(),
        }
    }
}

impl DatabaseConfig {
    /// Construit l'URL de connexion résolue
    pub fn resolved_url(&self) -> Result<String, crate::errors::AppError> {
        crate::config::builders::resolve_database_url::resolve_database_url(self)
    }

    /// Retourne une version "sanitisée" de l'URL (password masqué)
    pub fn sanitized_url(&self) -> Result<String, crate::errors::AppError> {
        crate::config::builders::sanitize_database_url::sanitize_database_url(self)
    }
}

use anyhow::Result;
