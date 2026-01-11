#[allow(dead_code)]
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
