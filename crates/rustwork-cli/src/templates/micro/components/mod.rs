pub const CONTROLLER_RS: &str = r#"use axum::{extract::State, Json};
use rustwork::AppState;

use crate::errors::AppResult;
use crate::models::{{ name }}::{{ name | capitalize }};
use crate::services::{{ name }}_service;

pub async fn list_{{ name }}s(
    State(state): State<AppState>,
) -> AppResult<Json<Vec<{{ name | capitalize }}}>> {
    let items = {{ name }}_service::list_{{ name }}s(&state.db).await?;
    Ok(Json(items))
}

pub async fn get_{{ name }}(
    State(state): State<AppState>,
    id: i32,
) -> AppResult<Json<{{ name | capitalize }}>> {
    let item = {{ name }}_service::get_{{ name }}_by_id(&state.db, id).await?;
    Ok(Json(item))
}
"#;

pub const MODEL_RS: &str = r#"use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow)]
pub struct Model {
    pub id: i32,
    pub name: String,
    pub created_at: chrono::DateTime<chrono::Utc>,
}

pub type {{ name | capitalize }} = Model;
"#;

pub const SERVICE_RS: &str = r#"use anyhow::Result;
use rustwork::DatabaseConnection;

use crate::models::{{ name }}::{{ name | capitalize }};

pub async fn list_{{ name }}s(db: &DatabaseConnection) -> Result<Vec<{{ name | capitalize }}>> {
    // TODO: Implement list logic with sqlx
    Ok(vec![])
}

pub async fn get_{{ name }}_by_id(db: &DatabaseConnection, id: i32) -> Result<{{ name | capitalize }}> {
    // TODO: Implement get logic
    unimplemented!()
}
"#;
