use crate::config::AppConfig;
use sqlx::{MySql, Pool, Postgres, Sqlite};

/// Database connection pool supporting multiple backends
#[derive(Clone)]
pub enum DatabaseConnection {
    Sqlite(Pool<Sqlite>),
    Postgres(Pool<Postgres>),
    Mysql(Pool<MySql>),
}
use std::sync::Arc;

#[derive(Clone)]
pub struct AppState {
    pub db: DatabaseConnection,
    pub config: Arc<AppConfig>,
}

impl AppState {
    pub fn new(db: DatabaseConnection, config: AppConfig) -> Self {
        Self {
            db,
            config: Arc::new(config),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::config::types::{CorsConfig, DatabaseConfig, ServerConfig};
    use sqlx::SqlitePool;

    #[tokio::test]
    async fn test_app_state_new() {
        let pool = SqlitePool::connect("sqlite::memory:").await.unwrap();
        let db = DatabaseConnection::Sqlite(pool);
        let config = AppConfig {
            server: ServerConfig {
                host: "127.0.0.1".to_string(),
                port: 8080,
            },
            database: DatabaseConfig::default(),
            cors: CorsConfig::default(),
        };
        let state = AppState::new(db, config);
        assert_eq!(state.config.server.port, 8080);
    }

    #[tokio::test]
    async fn test_app_state_clone() {
        let pool = SqlitePool::connect("sqlite::memory:").await.unwrap();
        let db = DatabaseConnection::Sqlite(pool);
        let config = AppConfig {
            server: ServerConfig {
                host: "127.0.0.1".to_string(),
                port: 3000,
            },
            database: DatabaseConfig::default(),
            cors: CorsConfig::default(),
        };
        let state1 = AppState::new(db, config);
        let state2 = state1.clone();
        assert_eq!(state2.config.server.port, 3000);
        // Arc permet de partager la config sans duplication
        assert_eq!(Arc::strong_count(&state1.config), 2);
    }

    #[tokio::test]
    async fn test_app_state_config_is_arc() {
        let pool = SqlitePool::connect("sqlite::memory:").await.unwrap();
        let db = DatabaseConnection::Sqlite(pool);
        let config = AppConfig {
            server: ServerConfig {
                host: "0.0.0.0".to_string(),
                port: 8000,
            },
            database: DatabaseConfig::default(),
            cors: CorsConfig::default(),
        };
        let state = AppState::new(db, config);
        // Cloner l'état ne duplique pas la config grâce à Arc
        let cloned = state.clone();
        assert_eq!(state.config.server.host, cloned.config.server.host);
    }
}
