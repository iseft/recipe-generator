import { useGenerateRecipe } from "./hooks/useGenerateRecipe";
import IngredientInput from "./components/IngredientInput";
import RecipeCard from "./components/RecipeCard";
import Spinner from "../../shared/components/Spinner";
import type { GenerateRecipeRequest } from "./types";

export default function GenerateRecipe() {
  const { mutate, data, isPending, error } = useGenerateRecipe();

  const handleSubmit = (request: GenerateRecipeRequest) => {
    mutate(request);
  };

  return (
    <div className="space-y-6">
      <IngredientInput
        onSubmit={handleSubmit}
        isLoading={isPending}
        error={error?.message ?? null}
      />

      {isPending && (
        <div className="flex items-center justify-center py-12">
          <Spinner size="md" />
          <span className="ml-3 text-sm text-gray-500 dark:text-gray-400">
            Generating your recipe...
          </span>
        </div>
      )}

      {data && !isPending && <RecipeCard recipe={data} />}
    </div>
  );
}
