use crate::domain::entities::Recipe;
use crate::domain::services::{LlmError, LlmService};
use reqwest::Client;

pub struct OpenAiClient {
    client: Client,
    api_key: String,
}

impl OpenAiClient {
    pub fn new(api_key: String) -> Self {
        Self {
            client: Client::new(),
            api_key,
        }
    }
}

impl LlmService for OpenAiClient {
    async fn generate_recipe(
        &self,
        ingredients: Vec<String>,
        dietary_restrictions: Option<Vec<String>>,
    ) -> Result<Recipe, LlmError> {
        todo!("Implement OpenAI API call")
    }
}
