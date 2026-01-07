use axum::{http::StatusCode, Json};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct ApiResponse<T> {
    pub success: bool,
    pub data: Option<T>,
    pub message: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error: Option<String>,
}

impl<T> ApiResponse<T> {
    pub fn new(success: bool, data: Option<T>, message: Option<String>) -> Self {
        Self {
            success,
            data,
            message,
            error: None,
        }
    }

    pub fn with_error(error: String) -> Self {
        Self {
            success: false,
            data: None,
            message: None,
            error: Some(error),
        }
    }
}

/// Helper: réponse success 200
pub fn ok<T: Serialize>(data: T) -> (StatusCode, Json<ApiResponse<T>>) {
    (
        StatusCode::OK,
        Json(ApiResponse::new(true, Some(data), None)),
    )
}

/// Helper: réponse created 201
pub fn created<T: Serialize>(data: T) -> (StatusCode, Json<ApiResponse<T>>) {
    (
        StatusCode::CREATED,
        Json(ApiResponse::new(true, Some(data), Some("Resource created".to_string()))),
    )
}

/// Helper: réponse error
pub fn error<T>(status: StatusCode, message: String) -> (StatusCode, Json<ApiResponse<T>>) {
    (
        status,
        Json(ApiResponse::with_error(message)),
    )
}
