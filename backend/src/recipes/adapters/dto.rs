use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
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

#[derive(Debug, Deserialize, Validate, ToSchema)]
#[serde(rename_all = "camelCase")]
#[schema(example = json!({
    "ingredients": ["chicken breast", "rice", "garlic", "onion"],
    "dietaryRestrictions": ["gluten-free"]
}))]
pub struct GenerateRecipeRequest {
    #[validate(custom(function = "validate_ingredients"))]
    #[schema(example = json!(["chicken breast", "rice", "garlic", "onion"]))]
    pub ingredients: Vec<String>,
    #[schema(example = json!(["gluten-free", "dairy-free"]))]
    pub dietary_restrictions: Option<Vec<String>>,
}

#[derive(Debug, Serialize, ToSchema)]
#[serde(rename_all = "camelCase")]
#[schema(example = json!({
    "title": "Garlic Chicken and Rice",
    "ingredients": ["2 chicken breasts", "1 cup rice", "3 cloves garlic", "1 onion"],
    "instructions": ["Cook chicken", "Add rice", "Season with garlic"],
    "prepTimeMinutes": 15,
    "cookTimeMinutes": 30,
    "servings": 4
}))]
pub struct GeneratedRecipeResponse {
    #[schema(example = "Garlic Chicken and Rice")]
    pub title: String,
    #[schema(example = json!(["2 chicken breasts", "1 cup rice", "3 cloves garlic"]))]
    pub ingredients: Vec<String>,
    #[schema(example = json!(["Preheat pan", "Cook chicken for 10 minutes", "Add rice and cook for 20 minutes"]))]
    pub instructions: Vec<String>,
    #[schema(example = 15, minimum = 0, maximum = 300)]
    pub prep_time_minutes: Option<i32>,
    #[schema(example = 30, minimum = 0, maximum = 600)]
    pub cook_time_minutes: Option<i32>,
    #[schema(example = 4, minimum = 1, maximum = 50)]
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

#[derive(Debug, Serialize, ToSchema)]
#[serde(rename_all = "camelCase")]
#[schema(example = json!({
    "id": "123e4567-e89b-12d3-a456-426614174000",
    "ownerId": "user_123",
    "ownerEmail": "user@example.com",
    "title": "Garlic Chicken and Rice",
    "ingredients": ["2 chicken breasts", "1 cup rice"],
    "instructions": ["Cook chicken", "Add rice"],
    "prepTimeMinutes": 15,
    "cookTimeMinutes": 30,
    "servings": 4,
    "createdAt": "2024-01-12T10:00:00Z"
}))]
pub struct RecipeResponse {
    #[schema(example = "123e4567-e89b-12d3-a456-426614174000")]
    pub id: Uuid,
    #[schema(example = "user_123")]
    pub owner_id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[schema(example = "user@example.com")]
    pub owner_email: Option<String>,
    #[schema(example = "Garlic Chicken and Rice")]
    pub title: String,
    #[schema(example = json!(["2 chicken breasts", "1 cup rice"]))]
    pub ingredients: Vec<String>,
    #[schema(example = json!(["Cook chicken", "Add rice"]))]
    pub instructions: Vec<String>,
    #[schema(example = 15, minimum = 0, maximum = 300)]
    pub prep_time_minutes: Option<i32>,
    #[schema(example = 30, minimum = 0, maximum = 600)]
    pub cook_time_minutes: Option<i32>,
    #[schema(example = 4, minimum = 1, maximum = 50)]
    pub servings: Option<i32>,
    #[schema(example = "2024-01-12T10:00:00Z")]
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

#[derive(Debug, Deserialize, Validate, ToSchema)]
#[serde(rename_all = "camelCase")]
#[schema(example = json!({
    "title": "Garlic Chicken and Rice",
    "ingredients": ["2 chicken breasts", "1 cup rice", "3 cloves garlic"],
    "instructions": ["Cook chicken", "Add rice", "Season"],
    "prepTimeMinutes": 15,
    "cookTimeMinutes": 30,
    "servings": 4
}))]
pub struct SaveRecipeRequest {
    #[validate(length(min = 1))]
    #[schema(example = "Garlic Chicken and Rice")]
    pub title: String,
    #[validate(custom(function = "validate_ingredients"))]
    #[schema(example = json!(["2 chicken breasts", "1 cup rice"]))]
    pub ingredients: Vec<String>,
    #[validate(length(min = 1))]
    #[schema(example = json!(["Cook chicken", "Add rice"]))]
    pub instructions: Vec<String>,
    #[schema(example = 15, minimum = 0, maximum = 300)]
    pub prep_time_minutes: Option<i32>,
    #[schema(example = 30, minimum = 0, maximum = 600)]
    pub cook_time_minutes: Option<i32>,
    #[schema(example = 4, minimum = 1, maximum = 50)]
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

#[derive(Debug, Deserialize, Validate, ToSchema)]
#[serde(rename_all = "camelCase")]
#[schema(example = json!({
    "email": "friend@example.com"
}))]
pub struct CreateShareRequest {
    #[validate(email)]
    #[schema(example = "friend@example.com")]
    pub email: String,
}

#[derive(Debug, Serialize, ToSchema)]
#[serde(rename_all = "camelCase")]
#[schema(example = json!({
    "userId": "user_123",
    "email": "friend@example.com",
    "createdAt": "2024-01-12T10:00:00Z"
}))]
pub struct ShareResponse {
    #[schema(example = "user_123")]
    pub user_id: String,
    #[schema(example = "friend@example.com")]
    pub email: String,
    #[schema(example = "2024-01-12T10:00:00Z")]
    pub created_at: chrono::DateTime<chrono::Utc>,
}
