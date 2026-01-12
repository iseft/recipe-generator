import { useMutation } from "@tanstack/react-query";
import { recipesClient } from "../api";
import type { GenerateRecipeRequest, Recipe } from "../types";

export function useGenerateRecipe() {
  return useMutation({
    mutationFn: (request: GenerateRecipeRequest) =>
      recipesClient.post<GenerateRecipeRequest, Recipe>(request, "generate"),
  });
}
