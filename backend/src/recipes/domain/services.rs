use super::entities::GeneratedRecipe;
use std::future::Future;

pub trait LlmService: Send + Sync {
    fn generate_recipe(
        &self,
        ingredients: Vec<String>,
        dietary_restrictions: Option<Vec<String>>,
    ) -> impl Future<Output = Result<GeneratedRecipe, LlmError>> + Send;
}

#[derive(Debug, Clone)]
pub enum LlmError {
    ApiError(String),
    ParseError(String),
}
