use uuid::Uuid;

use super::entities::{Recipe, RecipeShare};

#[derive(Debug, thiserror::Error)]
pub enum RepositoryError {
    #[error("Recipe not found")]
    NotFound,
    #[error("Access denied")]
    AccessDenied,
    #[error("Database error: {0}")]
    DatabaseError(String),
}

pub trait RecipeRepository: Send + Sync {
    fn save(
        &self,
        recipe: &Recipe,
    ) -> impl std::future::Future<Output = Result<(), RepositoryError>> + Send;

    fn find_by_id(
        &self,
        id: Uuid,
    ) -> impl std::future::Future<Output = Result<Recipe, RepositoryError>> + Send;

    fn find_by_owner(
        &self,
        owner_id: &str,
    ) -> impl std::future::Future<Output = Result<Vec<Recipe>, RepositoryError>> + Send;

    fn find_shared_with_user(
        &self,
        user_id: &str,
    ) -> impl std::future::Future<Output = Result<Vec<Recipe>, RepositoryError>> + Send;
}

pub trait RecipeShareRepository: Send + Sync {
    fn create(
        &self,
        share: &RecipeShare,
    ) -> impl std::future::Future<Output = Result<(), RepositoryError>> + Send;

    fn delete(
        &self,
        recipe_id: Uuid,
        user_id: &str,
    ) -> impl std::future::Future<Output = Result<(), RepositoryError>> + Send;

    fn is_shared_with_user(
        &self,
        recipe_id: Uuid,
        user_id: &str,
    ) -> impl std::future::Future<Output = Result<bool, RepositoryError>> + Send;

    fn find_by_recipe_id(
        &self,
        recipe_id: Uuid,
    ) -> impl std::future::Future<Output = Result<Vec<RecipeShare>, RepositoryError>> + Send;
}
