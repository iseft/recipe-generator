use serde::{Deserialize, Serialize};
use validator::Validate;

use chrono::{DateTime, Utc};
use uuid::Uuid;

use crate::recipes::domain::{GeneratedRecipe, Recipe};

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
    pub owner_id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub owner_email: Option<String>,
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
            owner_id: recipe.owner_id,
            owner_email: None,
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

impl RecipeResponse {
    pub fn with_owner_email(mut self, email: Option<String>) -> Self {
        self.owner_email = email;
        self
    }
}

#[derive(Debug, Deserialize, Validate)]
#[serde(rename_all = "camelCase")]
pub struct SaveRecipeRequest {
    #[validate(length(min = 1))]
    pub title: String,
    #[validate(custom(function = "validate_ingredients"))]
    pub ingredients: Vec<String>,
    #[validate(length(min = 1))]
    pub instructions: Vec<String>,
    pub prep_time_minutes: Option<i32>,
    pub cook_time_minutes: Option<i32>,
    pub servings: Option<i32>,
}

impl From<SaveRecipeRequest> for GeneratedRecipe {
    fn from(request: SaveRecipeRequest) -> Self {
        Self {
            title: request.title,
            ingredients: request.ingredients,
            instructions: request.instructions,
            prep_time_minutes: request.prep_time_minutes,
            cook_time_minutes: request.cook_time_minutes,
            servings: request.servings,
        }
    }
}

#[derive(Debug, Deserialize, Validate)]
#[serde(rename_all = "camelCase")]
pub struct CreateShareRequest {
    #[validate(email)]
    pub email: String,
}
