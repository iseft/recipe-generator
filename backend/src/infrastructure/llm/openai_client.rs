use crate::domain::entities::Recipe;
use crate::domain::services::{LlmError, LlmService};
use reqwest::Client;
use serde::{Deserialize, Serialize};

const OPENAI_API_URL: &str = "https://api.openai.com/v1/chat/completions";

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

    fn build_prompt(ingredients: &[String], dietary_restrictions: &Option<Vec<String>>) -> String {
        let restrictions = dietary_restrictions
            .as_ref()
            .map(|r| format!("Dietary restrictions: {}", r.join(", ")))
            .unwrap_or_default();

        format!(
            r#"Generate a recipe using these ingredients: {}.
{}

Respond with valid JSON only, no markdown, in this exact format:
{{
  "title": "Recipe Name",
  "ingredients": ["ingredient 1 with amount", "ingredient 2 with amount"],
  "instructions": ["step 1", "step 2"],
  "prep_time_minutes": 10,
  "cook_time_minutes": 20,
  "servings": 4
}}"#,
            ingredients.join(", "),
            restrictions
        )
    }
}

#[derive(Serialize)]
struct ChatRequest {
    model: String,
    messages: Vec<Message>,
}

#[derive(Serialize)]
struct Message {
    role: String,
    content: String,
}

#[derive(Deserialize)]
struct ChatResponse {
    choices: Vec<Choice>,
}

#[derive(Deserialize)]
struct Choice {
    message: ResponseMessage,
}

#[derive(Deserialize)]
struct ResponseMessage {
    content: String,
}

impl LlmService for OpenAiClient {
    async fn generate_recipe(
        &self,
        ingredients: Vec<String>,
        dietary_restrictions: Option<Vec<String>>,
    ) -> Result<Recipe, LlmError> {
        let prompt = Self::build_prompt(&ingredients, &dietary_restrictions);

        let request = ChatRequest {
            model: "gpt-4o-mini".to_string(),
            messages: vec![Message {
                role: "user".to_string(),
                content: prompt,
            }],
        };

        let response = self
            .client
            .post(OPENAI_API_URL)
            .header("Authorization", format!("Bearer {}", self.api_key))
            .json(&request)
            .send()
            .await
            .map_err(|e| LlmError::ApiError(e.to_string()))?;

        let chat_response: ChatResponse = response
            .json()
            .await
            .map_err(|e| LlmError::ApiError(e.to_string()))?;

        let content = chat_response
            .choices
            .first()
            .ok_or_else(|| LlmError::ParseError("No choices in response".to_string()))?
            .message
            .content
            .clone();

        serde_json::from_str(&content).map_err(|e| LlmError::ParseError(e.to_string()))
    }
}
