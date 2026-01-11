use crate::domain::entities::Recipe;
use crate::domain::repositories::{RecipeRepository, RepositoryError};
use std::sync::Arc;

pub struct ListSharedRecipesUseCase<T: RecipeRepository> {
    repository: Arc<T>,
}

impl<T: RecipeRepository> ListSharedRecipesUseCase<T> {
    pub fn new(repository: Arc<T>) -> Self {
        Self { repository }
    }

    pub async fn execute(&self, user_id: &str) -> Result<Vec<Recipe>, RepositoryError> {
        self.repository.find_shared_with_user(user_id).await
    }
}
