use std::sync::Arc;
use uuid::Uuid;

use crate::domain::repositories::{RecipeRepository, RecipeShareRepository, RepositoryError};

pub struct DeleteShareUseCase<R: RecipeRepository, S: RecipeShareRepository> {
    recipe_repository: Arc<R>,
    share_repository: Arc<S>,
}

impl<R: RecipeRepository, S: RecipeShareRepository> DeleteShareUseCase<R, S> {
    pub fn new(recipe_repository: Arc<R>, share_repository: Arc<S>) -> Self {
        Self {
            recipe_repository,
            share_repository,
        }
    }

    pub async fn execute(
        &self,
        recipe_id: Uuid,
        owner_id: &str,
        shared_user_id: &str,
    ) -> Result<(), RepositoryError> {
        let recipe = self.recipe_repository.find_by_id(recipe_id).await?;

        if recipe.owner_id != owner_id {
            return Err(RepositoryError::AccessDenied);
        }

        self.share_repository.delete(recipe_id, shared_user_id).await
    }
}
