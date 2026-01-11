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
use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Serialize, Deserialize)]
#[sea_orm(table_name = "{{ name }}s")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i32,
    pub name: String,
    pub created_at: DateTimeUtc,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}

pub type {{ name | capitalize }} = Model;
"#;

pub const SERVICE_RS: &str = r#"use anyhow::Result;
use sea_orm::DatabaseConnection;

use crate::models::{{ name }}::{{ name | capitalize }};

pub async fn list_{{ name }}s(db: &DatabaseConnection) -> Result<Vec<{{ name | capitalize }}>> {
    // TODO: Implement list logic
    Ok(vec![])
}

pub async fn get_{{ name }}_by_id(db: &DatabaseConnection, id: i32) -> Result<{{ name | capitalize }}> {
    // TODO: Implement get logic
    unimplemented!()
}
"#;
