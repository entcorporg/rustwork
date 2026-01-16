use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
    Json,
};
use sqlx::Error as SqlxError;
use serde_json::json;
use thiserror::Error;

pub type AppResult<T> = Result<T, AppError>;

#[derive(Error, Debug, PartialEq)]
pub enum AppError {
    #[error("Not found: {0}")]
    NotFound(String),

    #[error("Bad request: {0}")]
    BadRequest(String),

    #[error("Forbidden: {0}")]
    Forbidden(String),

    #[error("Internal server error: {0}")]
    InternalError(String),

    #[error("Database error: {0}")]
    Database(String),

    #[error("Validation error: {0}")]
    Validation(String),

    #[error("Conflict: {0}")]
    Conflict(String),

    #[error("Not implemented: {0}")]
    NotImplemented(String),
}

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        let (status, error_message) = match self {
            AppError::NotFound(msg) => (StatusCode::NOT_FOUND, msg),
            AppError::BadRequest(msg) => (StatusCode::BAD_REQUEST, msg),
            AppError::Forbidden(msg) => (StatusCode::FORBIDDEN, msg),
            AppError::Database(msg) => (
                StatusCode::INTERNAL_SERVER_ERROR,
                format!("Database error: {}", msg),
            ),
            AppError::Validation(msg) => (StatusCode::UNPROCESSABLE_ENTITY, msg),
            AppError::Conflict(msg) => (StatusCode::CONFLICT, msg),
            AppError::InternalError(msg) => (StatusCode::INTERNAL_SERVER_ERROR, msg),
            AppError::NotImplemented(msg) => (StatusCode::NOT_IMPLEMENTED, msg),
        };

        let body = Json(json!({
            "success": false,
            "error": error_message,
            "status": status.as_u16(),
        }));

        (status, body).into_response()
    }
}

// Conversion depuis SqlxError
impl From<SqlxError> for AppError {
    fn from(err: SqlxError) -> Self {
        match err {
            SqlxError::RowNotFound => AppError::NotFound("Record not found".to_string()),
            SqlxError::Database(db_err) => AppError::Database(db_err.to_string()),
            SqlxError::Configuration(msg) => AppError::Database(msg.to_string()),
            SqlxError::Io(io_err) => AppError::Database(io_err.to_string()),
            SqlxError::Tls(tls_err) => AppError::Database(tls_err.to_string()),
            _ => AppError::Database(err.to_string()),
        }
    }
}

// Conversion depuis anyhow::Error
impl From<anyhow::Error> for AppError {
    fn from(err: anyhow::Error) -> Self {
        AppError::InternalError(err.to_string())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_not_found_error_message() {
        let error = AppError::NotFound("user".to_string());
        assert_eq!(error.to_string(), "Not found: user");
    }

    #[test]
    fn test_bad_request_error_message() {
        let error = AppError::BadRequest("invalid input".to_string());
        assert_eq!(error.to_string(), "Bad request: invalid input");
    }

    #[test]
    fn test_validation_error_message() {
        let error = AppError::Validation("email format".to_string());
        assert_eq!(error.to_string(), "Validation error: email format");
    }

    #[test]
    fn test_database_error_message() {
        let error = AppError::Database("connection failed".to_string());
        assert_eq!(error.to_string(), "Database error: connection failed");
    }

    #[test]
    fn test_conflict_error_message() {
        let error = AppError::Conflict("duplicate key".to_string());
        assert_eq!(error.to_string(), "Conflict: duplicate key");
    }

    #[test]
    fn test_not_implemented_error() {
        let error = AppError::NotImplemented("feature X".to_string());
        assert_eq!(error.to_string(), "Not implemented: feature X");
    }

    #[test]
    fn test_forbidden_error() {
        let error = AppError::Forbidden("access denied".to_string());
        assert_eq!(error.to_string(), "Forbidden: access denied");
    }

    #[test]
    fn test_internal_error() {
        let error = AppError::InternalError("unexpected".to_string());
        assert_eq!(error.to_string(), "Internal server error: unexpected");
    }

    #[test]
    fn test_app_result_ok() {
        let result: AppResult<i32> = Ok(42);
        assert!(result.is_ok());
        assert_eq!(result, Ok(42));
    }

    #[test]
    fn test_app_result_err() {
        let result: AppResult<i32> = Err(AppError::NotFound("item".to_string()));
        assert!(result.is_err());
    }

    #[test]
    fn test_from_anyhow_error() {
        let anyhow_err = anyhow::anyhow!("something went wrong");
        let app_err: AppError = anyhow_err.into();
        assert!(matches!(app_err, AppError::InternalError(_)));
        assert!(app_err.to_string().contains("something went wrong"));
    }

    #[test]
    fn test_sqlx_row_not_found_conversion() {
        let sqlx_err = SqlxError::RowNotFound;
        let app_err: AppError = sqlx_err.into();
        assert!(matches!(app_err, AppError::NotFound(_)));
    }

    #[test]
    fn test_error_debug_impl() {
        let error = AppError::BadRequest("test".to_string());
        let debug_str = format!("{:?}", error);
        assert!(debug_str.contains("BadRequest"));
    }
}
