use crate::recipes::domain::{GeneratedRecipe, LlmError, LlmService};
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

    /// Builds the LLM prompt for recipe generation.
    /// Prompt design choices:
    /// 1. **JSON-only response**: We explicitly request "valid JSON only, no markdown" to ensure
    ///    the response can be directly parsed by serde_json without stripping markdown code fences.
    /// 2. **Exact schema provided**: Including the full JSON structure as an example ensures
    ///    consistent field names and types across all responses, making parsing reliable.
    /// 3. **Dietary restrictions inline**: Placed directly in the prompt context so the model
    ///    considers them when selecting ingredients and cooking methods.
    /// 4. **Use only listed ingredients**: The model must use primarily the provided ingredients,
    ///    with only basic kitchen staples (salt, pepper, herbs, sugar, etc.) allowed.
    /// 5. **LLM determines times and servings**: The model should calculate realistic prep_time_minutes,
    ///    cook_time_minutes, and servings based on the recipe complexity and yield.
    fn build_prompt(ingredients: &[String], dietary_restrictions: &Option<Vec<String>>) -> String {
        let restrictions = dietary_restrictions
            .as_ref()
            .map(|r| format!("Dietary restrictions: {}", r.join(", ")))
            .unwrap_or_default();

        format!(
            r#"Generate a recipe using ONLY these ingredients: {}.
You may also use basic kitchen staples that are commonly available such as salt, pepper, herbs, spices, sugar, oil, butter, or water. Do not add ingredients that are not in this list.

{}
IMPORTANT: Calculate realistic prep_time_minutes, cook_time_minutes, and servings based on the actual recipe you create. These should be integers that reflect the real complexity, cooking time, and yield of your recipe.

Respond with valid JSON only, no markdown, in this exact format (the numbers shown are examples - calculate appropriate values for your recipe):
{{
  "title": "Recipe Name",
  "ingredients": ["ingredient 1 with amount", "ingredient 2 with amount"],
  "instructions": ["step 1", "step 2"],
  "prep_time_minutes": 0,
  "cook_time_minutes": 0,
  "servings": 0
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
    ) -> Result<GeneratedRecipe, LlmError> {
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn build_prompt_includes_ingredients() {
        let ingredients = vec!["chicken".to_string(), "rice".to_string()];
        let prompt = OpenAiClient::build_prompt(&ingredients, &None);

        assert!(prompt.contains("chicken, rice"));
    }

    #[test]
    fn build_prompt_includes_dietary_restrictions() {
        let ingredients = vec!["tofu".to_string()];
        let restrictions = Some(vec!["vegan".to_string(), "gluten-free".to_string()]);
        let prompt = OpenAiClient::build_prompt(&ingredients, &restrictions);

        assert!(prompt.contains("Dietary restrictions: vegan, gluten-free"));
    }

    #[test]
    fn build_prompt_omits_restrictions_when_none() {
        let ingredients = vec!["beef".to_string()];
        let prompt = OpenAiClient::build_prompt(&ingredients, &None);

        assert!(!prompt.contains("Dietary restrictions"));
    }

    #[test]
    fn build_prompt_requests_json_format() {
        let ingredients = vec!["pasta".to_string()];
        let prompt = OpenAiClient::build_prompt(&ingredients, &None);

        assert!(prompt.contains("valid JSON"));
        assert!(prompt.contains("\"title\""));
        assert!(prompt.contains("\"ingredients\""));
        assert!(prompt.contains("\"instructions\""));
    }
}
