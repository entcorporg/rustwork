use crate::state::AppState;
use axum::{extract::State, Json};
use serde_json::json;

/// Handler pour /db/info (debug)
pub async fn db_info(State(state): State<AppState>) -> Json<serde_json::Value> {
    let config = &state.config.database;
    let sanitized = config
        .sanitized_url()
        .unwrap_or_else(|_| "[error]".to_string());

    Json(json!({
        "connection": format!("{:?}", config.connection).to_lowercase(),
        "url": sanitized,
        "pool": {
            "max_connections": config.pool.max_connections,
            "min_connections": config.pool.min_connections,
            "connect_timeout_ms": config.pool.connect_timeout_ms,
        }
    }))
}
