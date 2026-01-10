import { useMutation } from "@tanstack/react-query";
import { recipeApiClient } from "../api";
import type { GenerateRecipeRequest, Recipe } from "../types";

export function useGenerateRecipe() {
  return useMutation({
    mutationFn: (request: GenerateRecipeRequest) =>
      recipeApiClient.post<GenerateRecipeRequest, Recipe>(request),
  });
}
