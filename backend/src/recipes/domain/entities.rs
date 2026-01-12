use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Deserialize)]
pub struct GeneratedRecipe {
    pub title: String,
    pub ingredients: Vec<String>,
    pub instructions: Vec<String>,
    pub prep_time_minutes: Option<i32>,
    pub cook_time_minutes: Option<i32>,
    pub servings: Option<i32>,
}

#[derive(Debug, Clone, Serialize, sqlx::FromRow)]
pub struct Recipe {
    pub id: Uuid,
    pub owner_id: String,
    pub title: String,
    pub ingredients: Vec<String>,
    pub instructions: Vec<String>,
    pub prep_time_minutes: Option<i32>,
    pub cook_time_minutes: Option<i32>,
    pub servings: Option<i32>,
    pub created_at: DateTime<Utc>,
}

impl Recipe {
    pub fn from_generated(generated: GeneratedRecipe, owner_id: String) -> Self {
        Self {
            id: Uuid::new_v4(),
            owner_id,
            title: generated.title,
            ingredients: generated.ingredients,
            instructions: generated.instructions,
            prep_time_minutes: generated.prep_time_minutes,
            cook_time_minutes: generated.cook_time_minutes,
            servings: generated.servings,
            created_at: Utc::now(),
        }
    }
}

#[derive(Debug, Clone, Serialize, sqlx::FromRow)]
pub struct RecipeShare {
    pub recipe_id: Uuid,
    pub user_id: String,
    pub created_at: DateTime<Utc>,
}

impl RecipeShare {
    pub fn new(recipe_id: Uuid, user_id: String) -> Self {
        Self {
            recipe_id,
            user_id,
            created_at: Utc::now(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_complete_generated_recipe_json() {
        let json = r#"{
            "title": "Garlic Chicken",
            "ingredients": ["chicken", "garlic"],
            "instructions": ["cook chicken", "add garlic"],
            "prep_time_minutes": 10,
            "cook_time_minutes": 20,
            "servings": 4
        }"#;

        let recipe: GeneratedRecipe = serde_json::from_str(json).unwrap();

        assert_eq!(recipe.title, "Garlic Chicken");
        assert_eq!(recipe.ingredients.len(), 2);
        assert_eq!(recipe.instructions.len(), 2);
        assert_eq!(recipe.prep_time_minutes, Some(10));
        assert_eq!(recipe.cook_time_minutes, Some(20));
        assert_eq!(recipe.servings, Some(4));
    }

    #[test]
    fn parses_generated_recipe_with_optional_fields_missing() {
        let json = r#"{
            "title": "Simple Dish",
            "ingredients": ["salt"],
            "instructions": ["add salt"]
        }"#;

        let recipe: GeneratedRecipe = serde_json::from_str(json).unwrap();

        assert_eq!(recipe.title, "Simple Dish");
        assert!(recipe.prep_time_minutes.is_none());
        assert!(recipe.cook_time_minutes.is_none());
        assert!(recipe.servings.is_none());
    }

    #[test]
    fn fails_on_missing_required_fields() {
        let json = r#"{"title": "No Ingredients"}"#;

        let result: Result<GeneratedRecipe, _> = serde_json::from_str(json);

        assert!(result.is_err());
    }

    #[test]
    fn converts_generated_to_recipe() {
        let generated = GeneratedRecipe {
            title: "Test".to_string(),
            ingredients: vec!["a".to_string()],
            instructions: vec!["b".to_string()],
            prep_time_minutes: Some(5),
            cook_time_minutes: Some(10),
            servings: Some(2),
        };

        let recipe = Recipe::from_generated(generated, "user_123".to_string());

        assert_eq!(recipe.title, "Test");
        assert_eq!(recipe.owner_id, "user_123");
        assert!(!recipe.id.is_nil());
    }
}
