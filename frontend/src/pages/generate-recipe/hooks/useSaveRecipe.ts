import { useMutation, useQueryClient } from "@tanstack/react-query";
import { recipesClient } from "../api";
import type { Recipe } from "../types";

export function useSaveRecipe() {
  const queryClient = useQueryClient();

  return useMutation({
    mutationFn: (recipe: Recipe) => recipesClient.post<Recipe, Recipe>(recipe),
    onSuccess: () => {
      queryClient.invalidateQueries({ queryKey: ["recipes"] });
    },
  });
}
