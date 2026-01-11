use axum::{extract::Request, http::HeaderValue, middleware::Next, response::Response};
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_request_id_header_constant() {
        assert_eq!(REQUEST_ID_HEADER, "x-request-id");
    }

    #[test]
    fn test_uuid_generation_format() {
        let id = Uuid::new_v4().to_string();
        // UUID v4 format: 8-4-4-4-12 caractères hexadécimaux
        assert_eq!(id.len(), 36); // 32 hex + 4 tirets
        assert_eq!(id.chars().filter(|c| *c == '-').count(), 4);
    }

    #[test]
    fn test_header_value_from_uuid() {
        let id = Uuid::new_v4().to_string();
        let header_val = HeaderValue::from_str(&id);
        assert!(header_val.is_ok());
    }

    #[test]
    fn test_request_id_header_name_lowercase() {
        // Les noms de headers HTTP sont case-insensitive mais conventionnellement lowercase
        assert!(REQUEST_ID_HEADER
            .chars()
            .all(|c| c.is_lowercase() || c == '-'));
    }
}
