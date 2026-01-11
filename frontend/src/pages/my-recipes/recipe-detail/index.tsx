import { useParams } from "react-router-dom";
import PageLayout from "../../../shared/components/PageLayout";
import MyRecipeDetailContent from "./components/MyRecipeDetailContent";
import { useRecipe } from "../../../shared/hooks/useRecipe";

export default function MyRecipeDetailPage() {
  const { id } = useParams<{ id: string }>();
  const { data: recipe, isLoading, error } = useRecipe(id);

  const breadcrumbs = recipe
    ? [
        { name: "My Recipes", href: "/my-recipes" },
        { name: recipe.title, href: `/my-recipes/${recipe.id}`, current: true },
      ]
    : [{ name: "My Recipes", href: "/my-recipes" }];

  return (
    <PageLayout title="Recipe Details" breadcrumbs={breadcrumbs}>
      <MyRecipeDetailContent
        recipe={recipe}
        isLoading={isLoading}
        error={error}
      />
    </PageLayout>
  );
}
