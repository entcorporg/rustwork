use crate::{middleware, state::AppState};
use axum::{middleware as axum_middleware, routing::get, Router};
use tower_http::trace::TraceLayer;

use super::handlers::{db_info, health_check};

/// Construit le router Axum avec les middlewares par défaut
pub fn build_router(state: AppState) -> Router {
    let mut router = Router::new()
        .route("/health", get(health_check))
        .route("/db/info", get(db_info))
        // Ajoutez vos routes ici
        .layer(axum_middleware::from_fn(middleware::request_id_middleware));

    // Ajouter CORS uniquement si activé
    if let Some(cors_layer) = middleware::build_cors_layer(&state.config.cors) {
        router = router.layer(cors_layer);
    }

    router.layer(TraceLayer::new_for_http()).with_state(state)
}
