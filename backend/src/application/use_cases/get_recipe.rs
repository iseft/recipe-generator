use crate::domain::repositories::{RecipeRepository, RecipeShareRepository, RepositoryError};
use std::sync::Arc;
use uuid::Uuid;

pub struct GetRecipeUseCase<R: RecipeRepository, S: RecipeShareRepository> {
    recipe_repository: Arc<R>,
    share_repository: Arc<S>,
}

impl<R: RecipeRepository, S: RecipeShareRepository> GetRecipeUseCase<R, S> {
    pub fn new(recipe_repository: Arc<R>, share_repository: Arc<S>) -> Self {
        Self {
            recipe_repository,
            share_repository,
        }
    }

    pub async fn execute(
        &self,
        id: Uuid,
        user_id: &str,
    ) -> Result<crate::domain::entities::Recipe, RepositoryError> {
        let recipe = self.recipe_repository.find_by_id(id).await?;

        if recipe.owner_id == user_id {
            return Ok(recipe);
        }

        let is_shared = self
            .share_repository
            .is_shared_with_user(id, user_id)
            .await?;

        if is_shared {
            Ok(recipe)
        } else {
            Err(RepositoryError::AccessDenied)
        }
    }
}
