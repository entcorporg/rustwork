pub mod builders;
pub mod types;

#[cfg(test)]
mod tests;

// Re-exports publics pour compatibilité avec l'ancienne API
pub use builders::{
    load_app_config::load_app_config, resolve_database_url::resolve_database_url,
    sanitize_database_url::sanitize_database_url,
};
pub use types::{AppConfig, CorsConfig, DatabaseConfig, DbConnection, PoolConfig, ServerConfig};
