use axum::Json;
use rustwork::{response::ApiResponse, ok};

/// Health check endpoint
pub async fn check() -> (axum::http::StatusCode, Json<ApiResponse<String>>) {
    ok("test-api is running!".to_string())
}