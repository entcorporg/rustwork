use crate::{config::DatabaseConfig, errors::AppResult};
use sea_orm::{ConnectOptions, Database, DatabaseConnection};
use std::time::Duration;
use tracing::info;

pub async fn init_database(config: &DatabaseConfig) -> AppResult<DatabaseConnection> {
    info!("Connecting to database...");

    let mut opt = ConnectOptions::new(&config.url);
    opt.max_connections(config.max_connections)
        .min_connections(config.min_connections)
        .connect_timeout(Duration::from_secs(8))
        .idle_timeout(Duration::from_secs(8))
        .sqlx_logging(true);

    let db = Database::connect(opt).await?;

    info!("Database connected successfully");
    Ok(db)
}

// Helper pour pagination
#[derive(Debug, Clone)]
pub struct Paginator {
    pub page: u64,
    pub per_page: u64,
}

impl Paginator {
    pub fn new(page: u64, per_page: u64) -> Self {
        Self {
            page: page.max(1),
            per_page: per_page.clamp(1, 100),
        }
    }

    pub fn offset(&self) -> u64 {
        (self.page - 1) * self.per_page
    }

    pub fn limit(&self) -> u64 {
        self.per_page
    }
}

impl Default for Paginator {
    fn default() -> Self {
        Self::new(1, 20)
    }
}
