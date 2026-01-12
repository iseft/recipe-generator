use crate::recipes::domain::{Recipe, RecipeRepository, RepositoryError};
use std::sync::Arc;

pub struct SaveRecipeUseCase<T: RecipeRepository> {
    repository: Arc<T>,
}

impl<T: RecipeRepository> SaveRecipeUseCase<T> {
    pub fn new(repository: Arc<T>) -> Self {
        Self { repository }
    }

    pub async fn execute(&self, recipe: Recipe) -> Result<(), RepositoryError> {
        self.repository.save(&recipe).await
    }
}
