use axum::{
    extract::Request,
    http::HeaderValue,
    middleware::Next,
    response::Response,
};
use tower_http::cors::{Any, CorsLayer};
use tracing::info;
use uuid::Uuid;

pub const REQUEST_ID_HEADER: &str = "x-request-id";

/// Middleware pour ajouter un request_id à chaque requête
pub async fn request_id_middleware(mut req: Request, next: Next) -> Response {
    let request_id = Uuid::new_v4().to_string();
    
    // Ajoute le request_id dans les headers de la requête
    req.headers_mut().insert(
        REQUEST_ID_HEADER,
        HeaderValue::from_str(&request_id).unwrap(),
    );

    info!(request_id = %request_id, method = %req.method(), uri = %req.uri(), "Incoming request");

    let mut response = next.run(req).await;

    // Ajoute le request_id dans les headers de la réponse
    response.headers_mut().insert(
        REQUEST_ID_HEADER,
        HeaderValue::from_str(&request_id).unwrap(),
    );

    response
}

/// Crée un layer CORS par défaut
pub fn cors_layer() -> CorsLayer {
    CorsLayer::new()
        .allow_origin(Any)
        .allow_methods(Any)
        .allow_headers(Any)
}
