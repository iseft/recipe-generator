use crate::domain::entities::Recipe;
use std::future::Future;

pub trait LlmService: Send + Sync {
    fn generate_recipe(
        &self,
        ingredients: Vec<String>,
        dietary_restrictions: Option<Vec<String>>,
    ) -> impl Future<Output = Result<Recipe, LlmError>> + Send;
}

#[derive(Debug)]
pub enum LlmError {
    ApiError(String),
    ParseError(String),
}
