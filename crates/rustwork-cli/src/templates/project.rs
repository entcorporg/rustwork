// Templates pour génération de nouveau projet

pub const CARGO_TOML: &str = r#"[package]
name = "{{ project_name }}"
version = "0.1.0"
edition = "2021"

# This project is standalone, not part of rustwork workspace
[workspace]

[dependencies]
rustwork = { path = "../crates/rustwork" }
axum = "0.7"
tokio = { version = "1.40", features = ["full"] }
tracing = "0.1"
tracing-subscriber = { version = "0.3", features = ["env-filter"] }
anyhow = "1.0"
serde = { version = "1.0", features = ["derive"] }
"#;

pub const MAIN_RS: &str = r#"use rustwork::{AppConfig, AppState, init_database};
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

mod app;
mod routes;
mod errors;
mod controllers;
mod services;
mod models;
mod middlewares;

#[cfg(feature = "graphql")]
mod graphql;

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
    tracing::info!("Configuration loaded for environment");

    // Initialize database
    let db = init_database(&config.database).await?;
    tracing::info!("Database initialized");

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

pub const APP_RS: &str = r#"use rustwork::{AppState, build_router};
use axum::Router;

use crate::routes;

/// Build the application router with all routes
pub fn build_app_router(state: AppState) -> Router {
    // Start with the base router from rustwork (includes /health)
    let app = build_router(state.clone());
    
    // Merge with our custom routes
    app.merge(routes::api_routes(state))
}
"#;

pub const ROUTES_RS: &str = r#"use rustwork::AppState;
use axum::{Router, routing::get};

use crate::controllers::health;

/// Define all API routes
pub fn api_routes(state: AppState) -> Router {
    Router::new()
        .route("/api/health", get(health::check))
        // Add your routes here
        .with_state(state)
}
"#;

pub const ERRORS_RS: &str = r#"// Custom errors for your application
// You can extend rustwork::AppError here if needed

pub use rustwork::{AppError, AppResult};
"#;

pub const HEALTH_RS: &str = r#"use axum::Json;
use rustwork::{response::ApiResponse, ok};

/// Health check endpoint
pub async fn check() -> (axum::http::StatusCode, Json<ApiResponse<String>>) {
    ok("{{ project_name }} is running!".to_string())
}
"#;

pub const DEFAULT_TOML: &str = r#"[server]
host = "0.0.0.0"
port = 3000

[database]
url = "postgres://postgres:postgres@localhost/{{ project_name }}"
max_connections = 10
min_connections = 2

[auth]
jwt_secret = "change-me-in-production"
jwt_expiration = 86400
"#;

pub const DEV_TOML: &str = r#"[server]
host = "127.0.0.1"
port = 3001

[database]
url = "postgres://postgres:postgres@localhost/{{ project_name }}_dev"
max_connections = 5
min_connections = 1
"#;

pub const ENV_EXAMPLE: &str = r#"APP_ENV=dev
APP__DATABASE__URL=postgres://postgres:postgres@localhost/{{ project_name }}
APP__SERVER__PORT=3000
APP__AUTH__JWT_SECRET=your-secret-key-here
"#;

pub const GITIGNORE: &str = r#"# Rust
/target
Cargo.lock

# Environment
.env

# IDE
.vscode/
.idea/
*.swp
*.swo

# Rustwork
.rustwork/
"#;

pub const CONTROLLER_RS: &str = r#"use axum::{
    extract::{State, Path},
    Json,
};
use rustwork::{AppState, AppResult, ApiResponse, ok, created};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct {{ struct_name }} {
    pub id: i32,
    pub name: String,
}

#[derive(Debug, Deserialize)]
pub struct Create{{ struct_name }}Request {
    pub name: String,
}

#[derive(Debug, Deserialize)]
pub struct Update{{ struct_name }}Request {
    pub name: String,
}

/// GET /{{ plural_name }}
pub async fn index(
    State(state): State<AppState>,
) -> AppResult<(axum::http::StatusCode, Json<ApiResponse<Vec<{{ struct_name }}>>>)> {
    // TODO: Fetch from database
    let items = vec![];
    Ok(ok(items))
}

/// GET /{{ plural_name }}/:id
pub async fn show(
    State(state): State<AppState>,
    Path(id): Path<i32>,
) -> AppResult<(axum::http::StatusCode, Json<ApiResponse<{{ struct_name }}>>)> {
    // TODO: Fetch from database
    todo!("Implement show")
}

/// POST /{{ plural_name }}
pub async fn create(
    State(state): State<AppState>,
    Json(payload): Json<Create{{ struct_name }}Request>,
) -> AppResult<(axum::http::StatusCode, Json<ApiResponse<{{ struct_name }}>>)> {
    // TODO: Save to database
    todo!("Implement create")
}

/// PUT /{{ plural_name }}/:id
pub async fn update(
    State(state): State<AppState>,
    Path(id): Path<i32>,
    Json(payload): Json<Update{{ struct_name }}Request>,
) -> AppResult<(axum::http::StatusCode, Json<ApiResponse<{{ struct_name }}>>)> {
    // TODO: Update in database
    todo!("Implement update")
}

/// DELETE /{{ plural_name }}/:id
pub async fn delete(
    State(state): State<AppState>,
    Path(id): Path<i32>,
) -> AppResult<(axum::http::StatusCode, Json<ApiResponse<()>>)> {
    // TODO: Delete from database
    todo!("Implement delete")
}
"#;

pub const MODEL_RS: &str = r#"// SeaORM entity for {{ struct_name }}
// Run: sea-orm-cli generate entity -o src/models

use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel)]
#[sea_orm(table_name = "{{ table_name }}")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i32,
    pub name: String,
    pub created_at: DateTimeUtc,
    pub updated_at: DateTimeUtc,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}
"#;

pub const SERVICE_RS: &str = r#"use rustwork::{AppResult, AppError};
use sea_orm::{DatabaseConnection, EntityTrait};

use crate::models::{{ snake_name }};

pub struct {{ struct_name }}Service {
    db: DatabaseConnection,
}

impl {{ struct_name }}Service {
    pub fn new(db: DatabaseConnection) -> Self {
        Self { db }
    }

    pub async fn find_all(&self) -> AppResult<Vec<{{ snake_name }}::Model>> {
        let items = {{ snake_name }}::Entity::find()
            .all(&self.db)
            .await?;
        Ok(items)
    }

    pub async fn find_by_id(&self, id: i32) -> AppResult<{{ snake_name }}::Model> {
        {{ snake_name }}::Entity::find_by_id(id)
            .one(&self.db)
            .await?
            .ok_or_else(|| AppError::NotFound(format!("{{ struct_name }} {} not found", id)))
    }

    // Add more methods as needed
}
"#;

pub const MIGRATION_RS: &str = r#"// Migration: create {{ table_name }} table
// To apply: sea-orm-cli migrate up

use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table({{ struct_name }}::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new({{ struct_name }}::Id)
                            .integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(ColumnDef::new({{ struct_name }}::Name).string().not_null())
                    .col(
                        ColumnDef::new({{ struct_name }}::CreatedAt)
                            .timestamp_with_time_zone()
                            .not_null()
                            .default(Expr::current_timestamp()),
                    )
                    .col(
                        ColumnDef::new({{ struct_name }}::UpdatedAt)
                            .timestamp_with_time_zone()
                            .not_null()
                            .default(Expr::current_timestamp()),
                    )
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table({{ struct_name }}::Table).to_owned())
            .await
    }
}

#[derive(Iden)]
enum {{ struct_name }} {
    Table,
    Id,
    Name,
    CreatedAt,
    UpdatedAt,
}
"#;
