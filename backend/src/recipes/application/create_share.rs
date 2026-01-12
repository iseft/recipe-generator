use std::sync::Arc;
use uuid::Uuid;

use crate::recipes::domain::{
    RecipeRepository, RecipeShare, RecipeShareRepository, RepositoryError,
};

pub struct CreateShareUseCase<R: RecipeRepository, S: RecipeShareRepository> {
    recipe_repository: Arc<R>,
    share_repository: Arc<S>,
}

impl<R: RecipeRepository, S: RecipeShareRepository> CreateShareUseCase<R, S> {
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
        share_with_user_id: String,
    ) -> Result<RecipeShare, RepositoryError> {
        let recipe = self.recipe_repository.find_by_id(recipe_id).await?;

        if recipe.owner_id != owner_id {
            return Err(RepositoryError::AccessDenied);
        }

        let share = RecipeShare::new(recipe_id, share_with_user_id);
        self.share_repository.create(&share).await?;

        Ok(share)
    }
}
