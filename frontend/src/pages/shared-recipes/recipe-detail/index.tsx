import { useParams } from "react-router-dom";
import PageLayout from "../../../shared/components/PageLayout";
import SharedRecipeDetailContent from "./components/SharedRecipeDetailContent";
import { useRecipe } from "../../../shared/hooks/useRecipe";

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
    <PageLayout title="Recipe Details" breadcrumbs={breadcrumbs}>
      <SharedRecipeDetailContent
        recipe={recipe}
        isLoading={isLoading}
        error={error}
      />
    </PageLayout>
  );
}
