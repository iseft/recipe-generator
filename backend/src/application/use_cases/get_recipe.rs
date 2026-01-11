use crate::domain::repositories::{RecipeRepository, RepositoryError};
use std::sync::Arc;
use uuid::Uuid;

pub struct GetRecipeUseCase<T: RecipeRepository> {
    repository: Arc<T>,
}

impl<T: RecipeRepository> GetRecipeUseCase<T> {
    pub fn new(repository: Arc<T>) -> Self {
        Self { repository }
    }

    pub async fn execute(
        &self,
        id: Uuid,
    ) -> Result<crate::domain::entities::Recipe, RepositoryError> {
        self.repository.find_by_id(id).await
    }
}
