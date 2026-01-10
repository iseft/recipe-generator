use serde::Deserialize;

#[derive(Debug, Clone, Deserialize)]
pub struct Recipe {
    pub title: String,
    pub ingredients: Vec<String>,
    pub instructions: Vec<String>,
    pub prep_time_minutes: Option<u32>,
    pub cook_time_minutes: Option<u32>,
    pub servings: Option<u32>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_complete_recipe_json() {
        let json = r#"{
            "title": "Garlic Chicken",
            "ingredients": ["chicken", "garlic"],
            "instructions": ["cook chicken", "add garlic"],
            "prep_time_minutes": 10,
            "cook_time_minutes": 20,
            "servings": 4
        }"#;

        let recipe: Recipe = serde_json::from_str(json).unwrap();

        assert_eq!(recipe.title, "Garlic Chicken");
        assert_eq!(recipe.ingredients.len(), 2);
        assert_eq!(recipe.instructions.len(), 2);
        assert_eq!(recipe.prep_time_minutes, Some(10));
        assert_eq!(recipe.cook_time_minutes, Some(20));
        assert_eq!(recipe.servings, Some(4));
    }

    #[test]
    fn parses_recipe_with_optional_fields_missing() {
        let json = r#"{
            "title": "Simple Dish",
            "ingredients": ["salt"],
            "instructions": ["add salt"]
        }"#;

        let recipe: Recipe = serde_json::from_str(json).unwrap();

        assert_eq!(recipe.title, "Simple Dish");
        assert!(recipe.prep_time_minutes.is_none());
        assert!(recipe.cook_time_minutes.is_none());
        assert!(recipe.servings.is_none());
    }

    #[test]
    fn fails_on_missing_required_fields() {
        let json = r#"{"title": "No Ingredients"}"#;

        let result: Result<Recipe, _> = serde_json::from_str(json);

        assert!(result.is_err());
    }
}
