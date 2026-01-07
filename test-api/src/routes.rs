use rustwork::AppState;
use axum::{Router, routing::{get, post, put, delete}};

use crate::controllers::health;
use crate::controllers::user;

/// Define all API routes
pub fn api_routes(state: AppState) -> Router {
    Router::new()
        .route("/api/health", get(health::check))
                .route("/api/users", get(user::index).post(user::create))
        .route("/api/users/:id", get(user::show).put(user::update).delete(user::delete))
        // Add your routes here
        .with_state(state)
}