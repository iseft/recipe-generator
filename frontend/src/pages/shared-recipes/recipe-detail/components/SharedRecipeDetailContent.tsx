import RecipeCard from "../../../../shared/components/recipe/RecipeCard";
import LoadingState from "../../../../shared/components/ui/LoadingState";
import ErrorState from "../../../../shared/components/ui/ErrorState";
import type { Recipe } from "../../../generate-recipe/types";

interface SharedRecipeDetailContentProps {
  recipe: Recipe | undefined;
  isLoading: boolean;
  error: Error | null;
}

export default function SharedRecipeDetailContent({
  recipe,
  isLoading,
  error,
}: SharedRecipeDetailContentProps) {
  if (isLoading) {
    return <LoadingState message="Loading recipe..." />;
  }

  if (error || !recipe) {
    return (
      <ErrorState
        message="Recipe not found."
        backLink={{ to: "/shared-recipes", text: "← Back to Shared Recipes" }}
      />
    );
  }

  return (
    <div className="space-y-6">
      <RecipeCard recipe={recipe} />
    </div>
  );
}
