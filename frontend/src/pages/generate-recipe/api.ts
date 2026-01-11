import APIClient from "../../shared/api/api-client";
import type { Recipe } from "./types";

export const recipesClient = new APIClient<Recipe>("/api/recipes");
