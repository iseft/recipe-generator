import { useQuery } from "@tanstack/react-query";
import { recipesClient } from "../../generate-recipe/api";
import type { Recipe } from "../../generate-recipe/types";

export function useRecipe(id: string | undefined) {
  return useQuery<Recipe>({
    queryKey: ["recipe", id],
    queryFn: () => recipesClient.get(id!),
    enabled: !!id,
  });
}
