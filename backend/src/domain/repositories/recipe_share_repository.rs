use uuid::Uuid;

use super::RepositoryError;
use crate::domain::entities::RecipeShare;

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
}
