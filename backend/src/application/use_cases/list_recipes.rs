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

    pub async fn execute(&self) -> Result<Vec<Recipe>, RepositoryError> {
        self.repository.find_all().await
    }
}
