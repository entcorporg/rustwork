use crate::{middleware, state::AppState, response::ApiResponse};
use axum::{
    middleware as axum_middleware,
    routing::get,
    Router,
    Json,
};
use tower_http::trace::TraceLayer;

/// Construit le router Axum avec les middlewares par défaut
pub fn build_router(state: AppState) -> Router {
    Router::new()
        .route("/health", get(health_check))
        // Ajoutez vos routes ici
        .layer(axum_middleware::from_fn(middleware::request_id_middleware))
        .layer(middleware::cors_layer())
        .layer(TraceLayer::new_for_http())
        .with_state(state)
}

/// Handler de base pour /health
async fn health_check() -> Json<ApiResponse<String>> {
    Json(ApiResponse::new(
        true,
        Some("OK".to_string()),
        Some("Service is healthy".to_string()),
    ))
}
