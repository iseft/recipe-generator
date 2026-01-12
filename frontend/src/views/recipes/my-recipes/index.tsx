import PageLayout from "../../../shared/components/ui/PageLayout";
import MyRecipesContent from "./components/MyRecipesContent";
import { recipeNavigation } from "../shared/navigation";

export default function MyRecipesPage() {
  return (
    <PageLayout
      title="My Recipes"
      breadcrumbs={[{ name: "My Recipes", href: "/my-recipes", current: true }]}
      sidebarNavigation={recipeNavigation}
    >
      <MyRecipesContent />
    </PageLayout>
  );
}
