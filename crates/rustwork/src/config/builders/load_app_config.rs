use crate::config::builders::{
    resolve_database_url::resolve_database_url, validate_cors_config::validate_cors_config,
};
use crate::config::types::AppConfig;
use anyhow::Result;
use config::{Config, Environment, File};
use std::env;

/// Charge la configuration depuis .env puis config/{profile}.toml
/// Fallback sur config/default.toml
pub fn load_app_config() -> Result<AppConfig> {
    // Charge .env si présent
    dotenvy::dotenv().ok();

    // Détermine le profil (dev/test/prod)
    let profile = env::var("APP_ENV").unwrap_or_else(|_| "dev".to_string());

    // Build config de base
    let mut builder = Config::builder()
        // Commence par le fichier par défaut
        .add_source(File::with_name("config/default").required(false))
        // Overlay avec le fichier profil
        .add_source(File::with_name(&format!("config/{}", profile)).required(false));

    // Gestion des variables DB à la Laravel
    // DB_CONNECTION
    if let Ok(conn) = env::var("DB_CONNECTION") {
        builder = builder.set_override("database.connection", conn)?;
    }

    // DB_URL (priorité absolue)
    if let Ok(url) = env::var("DB_URL") {
        builder = builder.set_override("database.url", url)?;
    }

    // DB_SQLITE_PATH
    if let Ok(path) = env::var("DB_SQLITE_PATH") {
        builder = builder.set_override("database.sqlite_path", path)?;
    }

    // DB_HOST
    if let Ok(host) = env::var("DB_HOST") {
        builder = builder.set_override("database.host", host)?;
    }

    // DB_PORT
    if let Ok(port) = env::var("DB_PORT") {
        builder = builder.set_override("database.port", port)?;
    }

    // DB_DATABASE
    if let Ok(database) = env::var("DB_DATABASE") {
        builder = builder.set_override("database.database", database)?;
    }

    // DB_USERNAME
    if let Ok(username) = env::var("DB_USERNAME") {
        builder = builder.set_override("database.username", username)?;
    }

    // DB_PASSWORD
    if let Ok(password) = env::var("DB_PASSWORD") {
        builder = builder.set_override("database.password", password)?;
    }

    // Override avec variables d'environnement (préfixe APP__)
    builder = builder.add_source(Environment::with_prefix("APP").separator("__"));

    let config = builder.build()?;
    let app_config: AppConfig = config.try_deserialize()?;

    // Validation au boot
    resolve_database_url(&app_config.database)?;
    validate_cors_config(&app_config.cors)?;

    Ok(app_config)
}
