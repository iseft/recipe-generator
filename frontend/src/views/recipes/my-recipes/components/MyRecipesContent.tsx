import { useMyRecipes } from "../hooks/useMyRecipes";
import RecipeCard from "../../../../shared/components/recipe/RecipeCard";
import LoadingState from "../../../../shared/components/ui/LoadingState";
import ErrorState from "../../../../shared/components/ui/ErrorState";

export default function MyRecipesContent() {
  const { data: recipes, isLoading, error } = useMyRecipes();

  if (isLoading) {
    return <LoadingState message="Loading recipes..." />;
  }

  if (error) {
    return <ErrorState message="Failed to load recipes. Please try again." />;
  }

  const recipesArray = Array.isArray(recipes) ? recipes : [];

  if (recipesArray.length === 0) {
    return (
      <div className="text-center py-12">
        <p className="text-gray-500 dark:text-gray-400">
          No saved recipes yet. Generate and save a recipe to see it here!
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
          linkTo={`/my-recipes/${recipe.id}`}
          showShareButton={true}
        />
      ))}
    </div>
  );
}
