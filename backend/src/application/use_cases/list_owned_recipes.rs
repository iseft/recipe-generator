use crate::domain::entities::Recipe;
use crate::domain::repositories::{RecipeRepository, RepositoryError};
use std::sync::Arc;

pub struct ListOwnedRecipesUseCase<T: RecipeRepository> {
    repository: Arc<T>,
}

impl<T: RecipeRepository> ListOwnedRecipesUseCase<T> {
    pub fn new(repository: Arc<T>) -> Self {
        Self { repository }
    }

    pub async fn execute(&self, owner_id: &str) -> Result<Vec<Recipe>, RepositoryError> {
        self.repository.find_by_owner(owner_id).await
    }
}
