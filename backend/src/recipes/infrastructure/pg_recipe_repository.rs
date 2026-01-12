use sqlx::PgPool;
use uuid::Uuid;

use crate::recipes::domain::{Recipe, RecipeRepository, RepositoryError};

pub struct PgRecipeRepository {
    pool: PgPool,
}

impl PgRecipeRepository {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }
}

impl RecipeRepository for PgRecipeRepository {
    async fn save(&self, recipe: &Recipe) -> Result<(), RepositoryError> {
        sqlx::query(
            r#"
            INSERT INTO recipes (id, owner_id, title, ingredients, instructions, prep_time_minutes, cook_time_minutes, servings, created_at)
            VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9)
            "#,
        )
        .bind(&recipe.id)
        .bind(&recipe.owner_id)
        .bind(&recipe.title)
        .bind(&recipe.ingredients)
        .bind(&recipe.instructions)
        .bind(&recipe.prep_time_minutes)
        .bind(&recipe.cook_time_minutes)
        .bind(&recipe.servings)
        .bind(&recipe.created_at)
        .execute(&self.pool)
        .await
        .map_err(|e| RepositoryError::DatabaseError(e.to_string()))?;

        Ok(())
    }

    async fn find_by_id(&self, id: Uuid) -> Result<Recipe, RepositoryError> {
        sqlx::query_as::<_, Recipe>(
            "SELECT id, owner_id, title, ingredients, instructions, prep_time_minutes, cook_time_minutes, servings, created_at FROM recipes WHERE id = $1",
        )
        .bind(id)
        .fetch_optional(&self.pool)
        .await
        .map_err(|e| RepositoryError::DatabaseError(e.to_string()))?
        .ok_or(RepositoryError::NotFound)
    }

    async fn find_by_owner(&self, owner_id: &str) -> Result<Vec<Recipe>, RepositoryError> {
        sqlx::query_as::<_, Recipe>(
            "SELECT id, owner_id, title, ingredients, instructions, prep_time_minutes, cook_time_minutes, servings, created_at FROM recipes WHERE owner_id = $1 ORDER BY created_at DESC",
        )
        .bind(owner_id)
        .fetch_all(&self.pool)
        .await
        .map_err(|e| RepositoryError::DatabaseError(e.to_string()))
    }

    async fn find_shared_with_user(&self, user_id: &str) -> Result<Vec<Recipe>, RepositoryError> {
        sqlx::query_as::<_, Recipe>(
            r#"
            SELECT r.id, r.owner_id, r.title, r.ingredients, r.instructions, r.prep_time_minutes, r.cook_time_minutes, r.servings, r.created_at
            FROM recipes r
            INNER JOIN recipe_shares rs ON r.id = rs.recipe_id
            WHERE rs.user_id = $1
            ORDER BY rs.created_at DESC
            "#,
        )
        .bind(user_id)
        .fetch_all(&self.pool)
        .await
        .map_err(|e| RepositoryError::DatabaseError(e.to_string()))
    }
}
