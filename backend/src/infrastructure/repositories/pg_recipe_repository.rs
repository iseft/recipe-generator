use sqlx::PgPool;
use uuid::Uuid;

use crate::domain::entities::Recipe;
use crate::domain::repositories::{RecipeRepository, RepositoryError};

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
            INSERT INTO recipes (id, title, ingredients, instructions, prep_time_minutes, cook_time_minutes, servings, created_at)
            VALUES ($1, $2, $3, $4, $5, $6, $7, $8)
            "#,
        )
        .bind(&recipe.id)
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
            "SELECT id, title, ingredients, instructions, prep_time_minutes, cook_time_minutes, servings, created_at FROM recipes WHERE id = $1",
        )
        .bind(id)
        .fetch_optional(&self.pool)
        .await
        .map_err(|e| RepositoryError::DatabaseError(e.to_string()))?
        .ok_or(RepositoryError::NotFound)
    }

    async fn find_all(&self) -> Result<Vec<Recipe>, RepositoryError> {
        sqlx::query_as::<_, Recipe>(
            "SELECT id, title, ingredients, instructions, prep_time_minutes, cook_time_minutes, servings, created_at FROM recipes ORDER BY created_at DESC",
        )
        .fetch_all(&self.pool)
        .await
        .map_err(|e| RepositoryError::DatabaseError(e.to_string()))
    }
}
