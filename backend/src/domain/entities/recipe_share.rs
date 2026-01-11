use chrono::{DateTime, Utc};
use serde::Serialize;
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, sqlx::FromRow)]
pub struct RecipeShare {
    pub recipe_id: Uuid,
    pub user_id: String,
    pub created_at: DateTime<Utc>,
}

impl RecipeShare {
    pub fn new(recipe_id: Uuid, user_id: String) -> Self {
        Self {
            recipe_id,
            user_id,
            created_at: Utc::now(),
        }
    }
}
