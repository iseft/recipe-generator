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

use chrono::{DateTime, Utc};
use uuid::Uuid;

use crate::domain::entities::{GeneratedRecipe, Recipe};

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct GeneratedRecipeResponse {
    pub title: String,
    pub ingredients: Vec<String>,
    pub instructions: Vec<String>,
    pub prep_time_minutes: Option<i32>,
    pub cook_time_minutes: Option<i32>,
    pub servings: Option<i32>,
}

impl From<GeneratedRecipe> for GeneratedRecipeResponse {
    fn from(recipe: GeneratedRecipe) -> Self {
        Self {
            title: recipe.title,
            ingredients: recipe.ingredients,
            instructions: recipe.instructions,
            prep_time_minutes: recipe.prep_time_minutes,
            cook_time_minutes: recipe.cook_time_minutes,
            servings: recipe.servings,
        }
    }
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct RecipeResponse {
    pub id: Uuid,
    pub title: String,
    pub ingredients: Vec<String>,
    pub instructions: Vec<String>,
    pub prep_time_minutes: Option<i32>,
    pub cook_time_minutes: Option<i32>,
    pub servings: Option<i32>,
    pub created_at: DateTime<Utc>,
}

impl From<Recipe> for RecipeResponse {
    fn from(recipe: Recipe) -> Self {
        Self {
            id: recipe.id,
            title: recipe.title,
            ingredients: recipe.ingredients,
            instructions: recipe.instructions,
            prep_time_minutes: recipe.prep_time_minutes,
            cook_time_minutes: recipe.cook_time_minutes,
            servings: recipe.servings,
            created_at: recipe.created_at,
        }
    }
}
