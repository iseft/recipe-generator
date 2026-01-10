use serde::{Deserialize, Serialize};
use validator::Validate;

fn validate_ingredients(ingredients: &[String]) -> Result<(), validator::ValidationError> {
    let valid_count = ingredients.iter().filter(|s| !s.trim().is_empty()).count();
    if valid_count == 0 {
        return Err(validator::ValidationError::new(
            "at_least_one_non_empty_ingredient",
        ));
    }
    Ok(())
}

#[derive(Debug, Deserialize, Validate)]
#[serde(rename_all = "camelCase")]
pub struct GenerateRecipeRequest {
    #[validate(custom(function = "validate_ingredients"))]
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
