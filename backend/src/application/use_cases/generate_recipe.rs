use crate::domain::entities::GeneratedRecipe;
use crate::domain::services::{LlmError, LlmService};
use std::sync::Arc;

pub struct GenerateRecipeUseCase<T: LlmService> {
    llm_service: Arc<T>,
}

impl<T: LlmService> GenerateRecipeUseCase<T> {
    pub fn new(llm_service: Arc<T>) -> Self {
        Self { llm_service }
    }

    pub async fn execute(
        &self,
        ingredients: Vec<String>,
        dietary_restrictions: Option<Vec<String>>,
    ) -> Result<GeneratedRecipe, LlmError> {
        self.llm_service
            .generate_recipe(ingredients, dietary_restrictions)
            .await
    }
}
