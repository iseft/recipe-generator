import { useMutation, useQueryClient } from "@tanstack/react-query";
import { recipesClient } from "../../views/recipes/generate-recipe/api";

interface ShareRecipeData {
  email: string;
}

export function useShareRecipe(recipeId: string) {
  const queryClient = useQueryClient();

  return useMutation({
    mutationFn: (data: ShareRecipeData) =>
      recipesClient.post(data, `${recipeId}/shares`),
    onSuccess: () => {
      queryClient.invalidateQueries({ queryKey: ["recipe", recipeId] });
    },
  });
}
