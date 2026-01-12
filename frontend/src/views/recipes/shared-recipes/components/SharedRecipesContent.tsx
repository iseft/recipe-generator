import { useSharedRecipes } from "../hooks/useSharedRecipes";
import RecipeCard from "../../shared/components/RecipeCard";
import LoadingState from "../../../../shared/components/ui/LoadingState";
import ErrorState from "../../../../shared/components/ui/ErrorState";

export default function SharedRecipesContent() {
  const { data: recipes, isLoading, error } = useSharedRecipes();

  if (isLoading) {
    return <LoadingState message="Loading shared recipes..." />;
  }

  if (error) {
    return (
      <ErrorState message="Failed to load shared recipes. Please try again." />
    );
  }

  const recipesArray = Array.isArray(recipes) ? recipes : [];

  if (recipesArray.length === 0) {
    return (
      <div className="text-center py-12">
        <p className="text-gray-500 dark:text-gray-400">
          No shared recipes yet. Other users haven't shared any recipes with
          you.
        </p>
      </div>
    );
  }

  return (
    <div className="space-y-6">
      {recipesArray.map((recipe) => (
        <RecipeCard
          key={recipe.id}
          recipe={recipe}
          linkTo={`/shared-recipes/${recipe.id}`}
          showSharedBy={true}
        />
      ))}
    </div>
  );
}
