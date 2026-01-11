import { Link } from "react-router-dom";
import { useRecipe } from "../../hooks/useRecipe";
import RecipeCard from "../../../../shared/components/recipe/RecipeCard";
import LoadingState from "../../../../shared/components/ui/LoadingState";
import ErrorState from "../../../../shared/components/ui/ErrorState";

interface RecipeDetailContentProps {
  id: string | undefined;
}

export default function RecipeDetailContent({ id }: RecipeDetailContentProps) {
  const { data: recipe, isLoading, error } = useRecipe(id);

  if (isLoading) {
    return <LoadingState message="Loading recipe..." />;
  }

  if (error) {
    return (
      <ErrorState
        message="Failed to load recipe. Please try again."
        backLink={{ to: "/recipes", text: "← Back to Saved Recipes" }}
      />
    );
  }

  if (!recipe) {
    return (
      <ErrorState
        message="Recipe not found."
        variant="info"
        backLink={{ to: "/recipes", text: "← Back to Saved Recipes" }}
      />
    );
  }

  return (
    <div className="space-y-6">
      <Link
        to="/recipes"
        className="inline-flex items-center text-sm text-indigo-600 hover:text-indigo-500 dark:text-indigo-400 dark:hover:text-indigo-300"
      >
        ← Back to Saved Recipes
      </Link>
      <RecipeCard recipe={recipe} />
    </div>
  );
}
