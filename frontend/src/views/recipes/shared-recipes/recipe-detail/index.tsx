import { useParams } from "react-router-dom";
import PageLayout from "../../../../shared/components/PageLayout";
import SharedRecipeDetailContent from "./SharedRecipeDetailContent";
import { useRecipe } from "../../shared/hooks/useRecipe";
import { recipeNavigation } from "../../shared/navigation";

export default function SharedRecipeDetailPage() {
  const { id } = useParams<{ id: string }>();
  const { data: recipe, isLoading, error } = useRecipe(id);

  const breadcrumbs = recipe
    ? [
        { name: "Shared with Me", href: "/shared-recipes" },
        {
          name: recipe.title,
          href: `/shared-recipes/${recipe.id}`,
          current: true,
        },
      ]
    : [{ name: "Shared with Me", href: "/shared-recipes" }];

  return (
    <PageLayout title="Recipe Details" breadcrumbs={breadcrumbs} sidebarNavigation={recipeNavigation}>
      <SharedRecipeDetailContent
        recipe={recipe}
        isLoading={isLoading}
        error={error}
      />
    </PageLayout>
  );
}
