#[allow(dead_code)]
pub const CONTROLLER_RS: &str = r#"use axum::{
    extract::{State, Path},
    Json,
};
use rustwork::{AppState, AppResult, AppError, ApiResponse, ok, created};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct {{ struct_name }} {
    pub id: i32,
    pub name: String,
}

#[derive(Debug, Deserialize)]
pub struct Create{{ struct_name }}Request {
    pub name: String,
}

#[derive(Debug, Deserialize)]
pub struct Update{{ struct_name }}Request {
    pub name: String,
}

/// GET /{{ plural_name }}
pub async fn index(
    State(state): State<AppState>,
) -> AppResult<(axum::http::StatusCode, Json<ApiResponse<Vec<{{ struct_name }}>>>)> {
    // TODO: Fetch from database
    let items = vec![];
    Ok(ok(items))
}

/// GET /{{ plural_name }}/:id
pub async fn show(
    State(state): State<AppState>,
    Path(id): Path<i32>,
) -> AppResult<(axum::http::StatusCode, Json<ApiResponse<{{ struct_name }}>>)> {
    // TODO: Fetch from database
    Err(AppError::NotImplemented("Endpoint not implemented yet".into()))
}

/// POST /{{ plural_name }}
pub async fn create(
    State(state): State<AppState>,
    Json(payload): Json<Create{{ struct_name }}Request>,
) -> AppResult<(axum::http::StatusCode, Json<ApiResponse<{{ struct_name }}>>)> {
    // TODO: Save to database
    Err(AppError::NotImplemented("Endpoint not implemented yet".into()))
}

/// PUT /{{ plural_name }}/:id
pub async fn update(
    State(state): State<AppState>,
    Path(id): Path<i32>,
    Json(payload): Json<Update{{ struct_name }}Request>,
) -> AppResult<(axum::http::StatusCode, Json<ApiResponse<{{ struct_name }}>>)> {
    // TODO: Update in database
    Err(AppError::NotImplemented("Endpoint not implemented yet".into()))
}

/// DELETE /{{ plural_name }}/:id
pub async fn delete(
    State(state): State<AppState>,
    Path(id): Path<i32>,
) -> AppResult<(axum::http::StatusCode, Json<ApiResponse<()>>)> {
    // TODO: Delete from database
    Err(AppError::NotImplemented("Endpoint not implemented yet".into()))
}
"#;
