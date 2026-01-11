use crate::config::types::DatabaseConfig;
use crate::errors::AppResult;
use sea_orm::{ConnectOptions, Database, DatabaseConnection};
use std::time::Duration;
use tracing::info;

/// Connecte à la base de données selon la config multi-driver
pub async fn connect_database(config: &DatabaseConfig) -> AppResult<DatabaseConnection> {
    let url = config.resolved_url()?;
    let sanitized = config
        .sanitized_url()
        .unwrap_or_else(|_| "[error]".to_string());

    info!("Connecting to database: {}", sanitized);

    let mut opt = ConnectOptions::new(url);
    opt.max_connections(config.pool.max_connections)
        .min_connections(config.pool.min_connections)
        .connect_timeout(Duration::from_millis(config.pool.connect_timeout_ms))
        .acquire_timeout(Duration::from_millis(config.pool.connect_timeout_ms))
        .idle_timeout(Duration::from_secs(300))
        .sqlx_logging(true)
        .sqlx_logging_level(tracing::log::LevelFilter::Debug);

    let db = Database::connect(opt).await?;

    info!("Database connected successfully");
    Ok(db)
}
