#[allow(dead_code)]
pub const MAIN_RS: &str = r#"use rustwork::{AppConfig, AppState, connect_db};
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

mod app;
mod routes;
mod errors;
mod controllers;
mod services;
mod models;
mod middlewares;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // Initialize tracing
    tracing_subscriber::registry()
        .with(tracing_subscriber::EnvFilter::try_from_default_env()
            .unwrap_or_else(|_| "{{ project_name }}=debug,rustwork=debug,tower_http=debug".into()))
        .with(tracing_subscriber::fmt::layer())
        .init();

    tracing::info!("Starting {{ project_name }}...");

    // Load configuration
    let config = AppConfig::load()?;
    tracing::info!("Configuration loaded");

    // Initialize database
    let db = connect_db(&config.database).await?;
    tracing::info!("Database connected");

    // Create application state
    let state = AppState::new(db, config.clone());

    // Build router with custom routes
    let app = app::build_app_router(state);

    // Start server
    let addr = format!("{}:{}", config.server.host, config.server.port);
    tracing::info!("Server listening on {}", addr);

    let listener = tokio::net::TcpListener::bind(&addr).await?;
    axum::serve(listener, app).await?;

    Ok(())
}
"#;
