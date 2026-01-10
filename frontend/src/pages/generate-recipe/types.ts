export interface GenerateRecipeRequest {
  ingredients: string[];
  dietaryRestrictions?: string[];
}

export interface Recipe {
  title: string;
  ingredients: string[];
  instructions: string[];
  prepTimeMinutes?: number;
  cookTimeMinutes?: number;
  servings?: number;
}

export interface ApiError {
  error: string;
}
