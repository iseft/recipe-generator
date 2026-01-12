import { useQuery } from "@tanstack/react-query";
import { recipesClient } from "../../generate-recipe/api";
import type { Recipe } from "../../generate-recipe/types";

export function useMyRecipes() {
  return useQuery<Recipe[]>({
    queryKey: ["recipes", "my"],
    queryFn: () => recipesClient.getAll(),
  });
}
