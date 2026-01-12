import PageLayout from "../../../shared/components/PageLayout";
import GenerateRecipeContent from "./components/GenerateRecipeContent";
import { recipeNavigation } from "../shared/navigation";

export default function GenerateRecipePage() {
  return (
    <PageLayout
      title="Recipe Generator"
      breadcrumbs={[{ name: "Recipe Generator", href: "/", current: true }]}
      sidebarNavigation={recipeNavigation}
    >
      <GenerateRecipeContent />
    </PageLayout>
  );
}
