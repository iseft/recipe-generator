use serde::{Deserialize, Serialize};
use validator::Validate;

#[derive(Debug, Deserialize, Validate)]
#[serde(rename_all = "camelCase")]
pub struct GenerateRecipeRequest {
    #[validate(length(min = 1, message = "At least one ingredient is required"))]
    pub ingredients: Vec<String>,
    pub dietary_restrictions: Option<Vec<String>>,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct RecipeResponse {
    pub title: String,
    pub ingredients: Vec<String>,
    pub instructions: Vec<String>,
    pub prep_time_minutes: Option<u32>,
    pub cook_time_minutes: Option<u32>,
    pub servings: Option<u32>,
}
