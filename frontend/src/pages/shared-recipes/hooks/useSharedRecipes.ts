import { useQuery } from "@tanstack/react-query";
import APIClient from "../../../shared/api/api-client";
import type { Recipe } from "../../generate-recipe/types";

const sharedRecipesClient = new APIClient<Recipe>("/api/recipes/shared");

export function useSharedRecipes() {
  return useQuery<Recipe[]>({
    queryKey: ["recipes", "shared"],
    queryFn: () => sharedRecipesClient.getAll(),
  });
}
