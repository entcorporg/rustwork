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
