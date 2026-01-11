use crate::config::AppConfig;
use sea_orm::DatabaseConnection;
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
    use sea_orm::Database;

    #[tokio::test]
    async fn test_app_state_new() {
        let db = Database::connect("sqlite::memory:").await.unwrap();
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
        let db = Database::connect("sqlite::memory:").await.unwrap();
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
        let db = Database::connect("sqlite::memory:").await.unwrap();
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
