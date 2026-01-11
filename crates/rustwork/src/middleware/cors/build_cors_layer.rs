use crate::config::types::CorsConfig;
use tower_http::cors::{AllowOrigin, CorsLayer};

/// Crée un layer CORS sécurisé depuis la configuration
/// - Fail-by-default: si enabled=false, retourne None
/// - Si enabled=true mais pas d'origines, panic au boot (déjà validé dans config)
pub fn build_cors_layer(config: &CorsConfig) -> Option<CorsLayer> {
    if !config.enabled {
        return None;
    }

    // Construire la liste des origines autorisées
    let origins: Vec<_> = config
        .allowed_origins
        .iter()
        .filter_map(|s: &String| s.parse::<axum::http::HeaderValue>().ok())
        .collect();

    // Construire les méthodes autorisées
    let methods: Vec<_> = config
        .allowed_methods
        .iter()
        .filter_map(|m: &String| m.parse::<axum::http::Method>().ok())
        .collect();

    // Construire les headers autorisés
    let headers: Vec<_> = config
        .allowed_headers
        .iter()
        .filter_map(|h: &String| h.parse::<axum::http::HeaderName>().ok())
        .collect();

    let mut cors = CorsLayer::new()
        .allow_origin(AllowOrigin::list(origins))
        .allow_methods(methods)
        .allow_headers(headers)
        .max_age(std::time::Duration::from_secs(config.max_age_seconds));

    if config.allow_credentials {
        cors = cors.allow_credentials(true);
    }

    Some(cors)
}
