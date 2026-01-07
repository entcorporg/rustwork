use axum::{
    extract::{State, Path},
    Json,
};
use rustwork::{AppState, AppResult, ApiResponse, ok, created};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct User {
    pub id: i32,
    pub name: String,
}

#[derive(Debug, Deserialize)]
pub struct CreateUserRequest {
    pub name: String,
}

#[derive(Debug, Deserialize)]
pub struct UpdateUserRequest {
    pub name: String,
}

/// GET /users
pub async fn index(
    State(state): State<AppState>,
) -> AppResult<(axum::http::StatusCode, Json<ApiResponse<Vec<User>>>)> {
    // TODO: Fetch from database
    let items = vec![];
    Ok(ok(items))
}

/// GET /users/:id
pub async fn show(
    State(state): State<AppState>,
    Path(id): Path<i32>,
) -> AppResult<(axum::http::StatusCode, Json<ApiResponse<User>>)> {
    // TODO: Fetch from database
    todo!("Implement show")
}

/// POST /users
pub async fn create(
    State(state): State<AppState>,
    Json(payload): Json<CreateUserRequest>,
) -> AppResult<(axum::http::StatusCode, Json<ApiResponse<User>>)> {
    // TODO: Save to database
    todo!("Implement create")
}

/// PUT /users/:id
pub async fn update(
    State(state): State<AppState>,
    Path(id): Path<i32>,
    Json(payload): Json<UpdateUserRequest>,
) -> AppResult<(axum::http::StatusCode, Json<ApiResponse<User>>)> {
    // TODO: Update in database
    todo!("Implement update")
}

/// DELETE /users/:id
pub async fn delete(
    State(state): State<AppState>,
    Path(id): Path<i32>,
) -> AppResult<(axum::http::StatusCode, Json<ApiResponse<()>>)> {
    // TODO: Delete from database
    todo!("Implement delete")
}