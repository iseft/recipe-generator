import { z } from "zod";

export const generateRecipeFormSchema = z.object({
  ingredients: z.string().min(1, "At least one ingredient is required"),
  dietaryRestrictions: z.string().optional(),
});

export type GenerateRecipeFormData = z.infer<typeof generateRecipeFormSchema>;

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
