use crate::domain::entities::Recipe;
use crate::domain::repositories::{RecipeRepository, RepositoryError};
use std::sync::Arc;

pub struct ListRecipesUseCase<T: RecipeRepository> {
    repository: Arc<T>,
}

impl<T: RecipeRepository> ListRecipesUseCase<T> {
    pub fn new(repository: Arc<T>) -> Self {
        Self { repository }
    }

    pub async fn execute_owned(&self, owner_id: &str) -> Result<Vec<Recipe>, RepositoryError> {
        self.repository.find_by_owner(owner_id).await
    }

    pub async fn execute_shared(&self, user_id: &str) -> Result<Vec<Recipe>, RepositoryError> {
        self.repository.find_shared_with_user(user_id).await
    }
}
