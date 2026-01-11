#[allow(dead_code)]
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
