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

    tracing::info!("Starting {{ project_name }} service...");

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
    tracing::info!("{{ project_name }} service listening on {}", addr);

    let listener = tokio::net::TcpListener::bind(&addr).await?;
    axum::serve(listener, app).await?;

    Ok(())
}
"#;

pub const APP_RS: &str = r#"use axum::Router;
use rustwork::AppState;

use crate::routes;

pub fn build_app_router(state: AppState) -> Router {
    Router::new()
        .merge(routes::create_routes())
        .with_state(state)
}
"#;

pub const ROUTES_RS: &str = r#"use axum::{routing::get, Router};
use rustwork::AppState;

use crate::controllers;

pub fn create_routes() -> Router<AppState> {
    Router::new()
        .route("/health", get(controllers::health::health_check))
}
"#;

pub const ERRORS_RS: &str = r#"use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
};

pub type AppResult<T> = Result<T, AppError>;

#[derive(Debug)]
pub enum AppError {
    Internal(String),
}

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        let (status, message) = match self {
            AppError::Internal(msg) => (StatusCode::INTERNAL_SERVER_ERROR, msg),
        };

        (status, message).into_response()
    }
}

impl From<anyhow::Error> for AppError {
    fn from(err: anyhow::Error) -> Self {
        AppError::Internal(err.to_string())
    }
}
"#;

pub const HEALTH_RS: &str = r#"use axum::Json;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct HealthResponse {
    pub status: String,
    pub service: String,
}

pub async fn health_check() -> Json<HealthResponse> {
    Json(HealthResponse {
        status: "ok".to_string(),
        service: "{{ project_name }}".to_string(),
    })
}
"#;

pub const DEFAULT_TOML: &str = r#"[server]
host = "127.0.0.1"
port = 3000

[database]
url = "sqlite://data/db.sqlite?mode=rwc"
max_connections = 5
"#;

pub const DEV_TOML: &str = r#"[server]
host = "0.0.0.0"
port = 3000

[database]
url = "sqlite://data/dev.db?mode=rwc"
max_connections = 10
"#;

pub const ENV_EXAMPLE: &str = r#"# Database URL (override config)
# DATABASE_URL=sqlite://data/db.sqlite?mode=rwc

# Server configuration
# SERVER_HOST=127.0.0.1
# SERVER_PORT=3000

# Logging level
# RUST_LOG=info,{{ project_name }}=debug
"#;

pub const CARGO_TOML: &str = r#"[package]
name = "{{ project_name }}"
version = "0.1.0"
edition = "2021"

[workspace]
members = ["migration"]

[dependencies]
rustwork = { path = "../../../../rustwork/crates/rustwork" }
migration = { path = "migration" }
axum = "0.7"
tokio = { version = "1.40", features = ["full"] }
tracing = "0.1"
tracing-subscriber = { version = "0.3", features = ["env-filter"] }
anyhow = "1.0"
serde = { version = "1.0", features = ["derive"] }
sea-orm = { version = "1.0", features = ["sqlx-sqlite", "sqlx-postgres", "sqlx-mysql", "runtime-tokio-native-tls", "macros"] }
sea-orm-migration = { version = "1.0" }
"#;

pub const GITIGNORE: &str = r#"# Rust
/target
Cargo.lock

# Environment
.env

# SQLite Database
/data/*.db
/data/*.db-shm
/data/*.db-wal

# IDE
*.swp
*.swo

# Rustwork
.rustwork/
"#;

pub const README_MD: &str = r#"# {{ project_name }}

Microservice built with Rustwork.

## Running

```bash
cargo run
```

## Development

```bash
cargo watch -x run
```

## Testing

```bash
cargo test
```
"#;
