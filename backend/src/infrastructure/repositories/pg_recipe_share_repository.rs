use sqlx::PgPool;
use uuid::Uuid;

use crate::domain::entities::RecipeShare;
use crate::domain::repositories::{RecipeShareRepository, RepositoryError};

pub struct PgRecipeShareRepository {
    pool: PgPool,
}

impl PgRecipeShareRepository {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }
}

impl RecipeShareRepository for PgRecipeShareRepository {
    async fn create(&self, share: &RecipeShare) -> Result<(), RepositoryError> {
        sqlx::query(
            r#"
            INSERT INTO recipe_shares (recipe_id, user_id, created_at)
            VALUES ($1, $2, $3)
            ON CONFLICT (recipe_id, user_id) DO NOTHING
            "#,
        )
        .bind(&share.recipe_id)
        .bind(&share.user_id)
        .bind(&share.created_at)
        .execute(&self.pool)
        .await
        .map_err(|e| RepositoryError::DatabaseError(e.to_string()))?;

        Ok(())
    }

    async fn delete(&self, recipe_id: Uuid, user_id: &str) -> Result<(), RepositoryError> {
        let result = sqlx::query(
            "DELETE FROM recipe_shares WHERE recipe_id = $1 AND user_id = $2",
        )
        .bind(recipe_id)
        .bind(user_id)
        .execute(&self.pool)
        .await
        .map_err(|e| RepositoryError::DatabaseError(e.to_string()))?;

        if result.rows_affected() == 0 {
            return Err(RepositoryError::NotFound);
        }

        Ok(())
    }

    async fn exists(&self, recipe_id: Uuid, user_id: &str) -> Result<bool, RepositoryError> {
        let result: Option<(i64,)> = sqlx::query_as(
            "SELECT 1 FROM recipe_shares WHERE recipe_id = $1 AND user_id = $2",
        )
        .bind(recipe_id)
        .bind(user_id)
        .fetch_optional(&self.pool)
        .await
        .map_err(|e| RepositoryError::DatabaseError(e.to_string()))?;

        Ok(result.is_some())
    }
}
