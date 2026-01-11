use uuid::Uuid;

use crate::domain::entities::RecipeShare;
use super::RepositoryError;

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
}
