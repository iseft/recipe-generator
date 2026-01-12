use std::sync::Arc;
use uuid::Uuid;

use crate::recipes::domain::{RecipeShare, RecipeShareRepository, RepositoryError};

pub struct ListRecipeSharesUseCase<S: RecipeShareRepository> {
    share_repository: Arc<S>,
}

impl<S: RecipeShareRepository> ListRecipeSharesUseCase<S> {
    pub fn new(share_repository: Arc<S>) -> Self {
        Self { share_repository }
    }

    pub async fn execute(&self, recipe_id: Uuid) -> Result<Vec<RecipeShare>, RepositoryError> {
        self.share_repository.find_by_recipe_id(recipe_id).await
    }
}
