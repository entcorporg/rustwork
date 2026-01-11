#[allow(dead_code)]
pub const SERVICE_RS: &str = r#"use rustwork::{AppResult, AppError};
use sea_orm::{DatabaseConnection, EntityTrait};

use crate::models::{{ snake_name }};

pub struct {{ struct_name }}Service {
    db: DatabaseConnection,
}

impl {{ struct_name }}Service {
    pub fn new(db: DatabaseConnection) -> Self {
        Self { db }
    }

    pub async fn find_all(&self) -> AppResult<Vec<{{ snake_name }}::Model>> {
        let items = {{ snake_name }}::Entity::find()
            .all(&self.db)
            .await?;
        Ok(items)
    }

    pub async fn find_by_id(&self, id: i32) -> AppResult<{{ snake_name }}::Model> {
        {{ snake_name }}::Entity::find_by_id(id)
            .one(&self.db)
            .await?
            .ok_or_else(|| AppError::NotFound(format!("{{ struct_name }} {} not found", id)))
    }

    // Add more methods as needed
}
"#;
