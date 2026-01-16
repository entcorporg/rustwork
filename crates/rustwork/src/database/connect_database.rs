use crate::config::types::DatabaseConfig;
use crate::errors::AppResult;
use crate::state::DatabaseConnection;
use sqlx::{
    mysql::MySqlConnectOptions, mysql::MySqlPoolOptions, postgres::PgConnectOptions,
    postgres::PgPoolOptions, sqlite::SqliteConnectOptions, sqlite::SqlitePoolOptions,
    ConnectOptions,
};
use std::str::FromStr;
use std::time::Duration;
use tracing::info;

/// Connecte à la base de données selon la config multi-driver
pub async fn connect_database(config: &DatabaseConfig) -> AppResult<DatabaseConnection> {
    let url = config.resolved_url()?;
    let sanitized = config
        .sanitized_url()
        .unwrap_or_else(|_| "[error]".to_string());

    info!("Connecting to database: {}", sanitized);

    // Déterminer le type de base de données depuis l'URL
    if url.starts_with("sqlite:") {
        let opts =
            SqliteConnectOptions::from_str(&url)?.log_statements(tracing::log::LevelFilter::Debug);

        let pool = SqlitePoolOptions::new()
            .max_connections(config.pool.max_connections)
            .min_connections(config.pool.min_connections)
            .acquire_timeout(Duration::from_millis(config.pool.connect_timeout_ms))
            .idle_timeout(Some(Duration::from_secs(300)))
            .connect_with(opts)
            .await?;

        info!("SQLite database connected successfully");
        Ok(DatabaseConnection::Sqlite(pool))
    } else if url.starts_with("postgres:") || url.starts_with("postgresql:") {
        let opts =
            PgConnectOptions::from_str(&url)?.log_statements(tracing::log::LevelFilter::Debug);

        let pool = PgPoolOptions::new()
            .max_connections(config.pool.max_connections)
            .min_connections(config.pool.min_connections)
            .acquire_timeout(Duration::from_millis(config.pool.connect_timeout_ms))
            .idle_timeout(Some(Duration::from_secs(300)))
            .connect_with(opts)
            .await?;

        info!("PostgreSQL database connected successfully");
        Ok(DatabaseConnection::Postgres(pool))
    } else if url.starts_with("mysql:") {
        let opts =
            MySqlConnectOptions::from_str(&url)?.log_statements(tracing::log::LevelFilter::Debug);

        let pool = MySqlPoolOptions::new()
            .max_connections(config.pool.max_connections)
            .min_connections(config.pool.min_connections)
            .acquire_timeout(Duration::from_millis(config.pool.connect_timeout_ms))
            .idle_timeout(Some(Duration::from_secs(300)))
            .connect_with(opts)
            .await?;

        info!("MySQL database connected successfully");
        Ok(DatabaseConnection::Mysql(pool))
    } else {
        Err(crate::errors::AppError::Database(format!(
            "Unsupported database URL: {}",
            sanitized
        )))
    }
}
