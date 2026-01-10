import APIClient from "../../shared/api/api-client";
import type { Recipe } from "./types";

export const recipeApiClient = new APIClient<Recipe>("/api/recipe");
