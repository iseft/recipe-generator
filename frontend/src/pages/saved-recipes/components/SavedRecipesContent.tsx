import { useRecipes } from "../hooks/useRecipes";
import RecipeCard from "../../../shared/components/recipe/RecipeCard";
import LoadingState from "../../../shared/components/ui/LoadingState";
import ErrorState from "../../../shared/components/ui/ErrorState";
import Breadcrumbs from "../../../shared/components/ui/Breadcrumbs";

export default function SavedRecipesContent() {
  const { data: recipes, isLoading, error } = useRecipes();

  if (isLoading) {
    return <LoadingState message="Loading recipes..." />;
  }

  if (error) {
    return <ErrorState message="Failed to load recipes. Please try again." />;
  }

  if (!recipes || recipes.length === 0) {
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
      <Breadcrumbs
        items={[{ name: "Saved Recipes", href: "/recipes", current: true }]}
      />
      {recipes.map((recipe) => (
        <RecipeCard key={recipe.id} recipe={recipe} />
      ))}
    </div>
  );
}
