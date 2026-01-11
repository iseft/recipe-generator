import { useRecipe } from "../../hooks/useRecipe";
import RecipeCard from "../../../../shared/components/recipe/RecipeCard";
import LoadingState from "../../../../shared/components/ui/LoadingState";
import ErrorState from "../../../../shared/components/ui/ErrorState";
import Breadcrumbs from "../../../../shared/components/ui/Breadcrumbs";

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
      <Breadcrumbs
        items={[
          { name: "Saved Recipes", href: "/recipes" },
          { name: recipe.title, href: `/recipes/${recipe.id}`, current: true },
        ]}
      />
      <RecipeCard recipe={recipe} />
    </div>
  );
}
