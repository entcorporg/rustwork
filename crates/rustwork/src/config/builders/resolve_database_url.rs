use crate::config::types::{DatabaseConfig, DbConnection};
use crate::errors::AppError;
use anyhow::Result;
use std::path::Path;

/// Construit l'URL de connexion résolue selon la priorité:
/// 1. DB_URL (ou url explicite)
/// 2. Sinon construction selon connection + params
pub fn resolve_database_url(config: &DatabaseConfig) -> Result<String, AppError> {
    // Priorité 1: URL explicite
    if let Some(url) = &config.url {
        if !url.is_empty() {
            return Ok(url.clone());
        }
    }

    // Priorité 2: construire selon le type de connexion
    match config.connection {
        DbConnection::Sqlite => {
            // Créer le dossier parent si nécessaire
            if let Some(parent) = Path::new(&config.sqlite_path).parent() {
                if !parent.exists() {
                    std::fs::create_dir_all(parent).map_err(|e| {
                        AppError::InternalError(format!(
                            "Failed to create SQLite directory {}: {}",
                            parent.display(),
                            e
                        ))
                    })?;
                }
            }
            Ok(format!("sqlite://{}?mode=rwc", config.sqlite_path))
        }
        DbConnection::Postgres => {
            let database = config.database.as_ref().ok_or_else(|| {
                AppError::BadRequest(
                    "DB_DATABASE is required when DB_CONNECTION=postgres".to_string(),
                )
            })?;
            let username = config.username.as_deref().unwrap_or("postgres");
            let password = config.password.as_deref().unwrap_or("");
            let host = &config.host;
            let port = config.port.unwrap_or(5432);

            Ok(format!(
                "postgres://{}:{}@{}:{}/{}",
                username, password, host, port, database
            ))
        }
        DbConnection::Mysql => {
            let database = config.database.as_ref().ok_or_else(|| {
                AppError::BadRequest("DB_DATABASE is required when DB_CONNECTION=mysql".to_string())
            })?;
            let username = config.username.as_deref().unwrap_or("root");
            let password = config.password.as_deref().unwrap_or("");
            let host = &config.host;
            let port = config.port.unwrap_or(3306);

            Ok(format!(
                "mysql://{}:{}@{}:{}/{}",
                username, password, host, port, database
            ))
        }
    }
}
