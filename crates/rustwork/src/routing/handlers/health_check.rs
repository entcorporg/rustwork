use crate::response::ApiResponse;
use axum::Json;

/// Handler de base pour /health
pub async fn health_check() -> Json<ApiResponse<String>> {
    Json(ApiResponse::new(
        true,
        Some("OK".to_string()),
        Some("Service is healthy".to_string()),
    ))
}
