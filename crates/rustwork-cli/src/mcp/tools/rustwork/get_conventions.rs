use crate::mcp::protocol::RpcError;
use serde_json::{json, Value};

/// rustwork.getConventions - Get Rustwork framework conventions
pub async fn rustwork_get_conventions() -> Result<Value, RpcError> {
    Ok(json!({
        "error_handling": {
            "type": "AppError",
            "file": "src/errors.rs",
            "variants": ["NotFound", "BadRequest", "Unauthorized", "InternalError", "DatabaseError"],
            "pattern": "Result<ApiResponse<T>, AppError>"
        },
        "response": {
            "type": "ApiResponse<T>",
            "file": "src/response.rs",
            "methods": ["success(data)", "error(message)"]
        },
        "handler_patterns": {
            "basic": "async fn handler(State(state): State<AppState>) -> Result<ApiResponse<T>, AppError>",
            "with_json": "async fn handler(State(state): State<AppState>, Json(payload): Json<P>) -> Result<ApiResponse<T>, AppError>",
            "with_path": "async fn handler(State(state): State<AppState>, Path(id): Path<i32>) -> Result<ApiResponse<T>, AppError>"
        },
        "config": {
            "file": "src/config.rs",
            "env_vars": {
                "database": ["DB_CONNECTION", "DB_HOST", "DB_PORT", "DB_DATABASE", "DB_USERNAME", "DB_PASSWORD"],
                "app": ["APP_HOST", "APP_PORT", "APP_ENV"],
                "jwt": ["JWT_SECRET", "JWT_EXPIRATION"]
            }
        },
        "middleware": {
            "file": "src/middleware.rs",
            "available": ["RequestId", "Logger", "Cors", "Auth"]
        }
    }))
}
