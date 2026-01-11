#[allow(dead_code)]
pub const HEALTH_RS: &str = r#"use axum::Json;
use rustwork::{response::ApiResponse, ok};

/// Health check endpoint
pub async fn check() -> (axum::http::StatusCode, Json<ApiResponse<String>>) {
    ok("{{ project_name }} is running!".to_string())
}
"#;
