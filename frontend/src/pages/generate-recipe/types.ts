import { z } from "zod";
import { parseCommaSeparated } from "../../shared/utils/parseCommaSeparated";

export const generateRecipeFormSchema = z.object({
  ingredients: z
    .string()
    .min(1, "At least one ingredient is required")
    .transform(parseCommaSeparated)
    .refine(
      (arr) => arr.length > 0,
      "At least one valid ingredient is required"
    ),
  dietaryRestrictions: z
    .string()
    .optional()
    .transform((val) => (val ? parseCommaSeparated(val) : undefined)),
});

export type GenerateRecipeFormData = z.input<typeof generateRecipeFormSchema>;
export type GenerateRecipeRequest = z.output<typeof generateRecipeFormSchema>;

export interface Recipe {
  id?: string;
  ownerId?: string;
  ownerEmail?: string;
  title: string;
  ingredients: string[];
  instructions: string[];
  prepTimeMinutes?: number;
  cookTimeMinutes?: number;
  servings?: number;
  createdAt?: string;
}

export interface ApiError {
  error: string;
}
