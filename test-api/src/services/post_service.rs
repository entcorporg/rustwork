use rustwork::{AppResult, AppError};
use sea_orm::{DatabaseConnection, EntityTrait};

use crate::models::post;

pub struct PostService {
    db: DatabaseConnection,
}

impl PostService {
    pub fn new(db: DatabaseConnection) -> Self {
        Self { db }
    }

    pub async fn find_all(&self) -> AppResult<Vec<post::Model>> {
        let items = post::Entity::find()
            .all(&self.db)
            .await?;
        Ok(items)
    }

    pub async fn find_by_id(&self, id: i32) -> AppResult<post::Model> {
        post::Entity::find_by_id(id)
            .one(&self.db)
            .await?
            .ok_or_else(|| AppError::NotFound(format!("Post {} not found", id)))
    }

    // Add more methods as needed
}