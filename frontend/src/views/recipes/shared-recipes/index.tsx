import PageLayout from "../../../shared/components/ui/PageLayout";
import SharedRecipesContent from "./components/SharedRecipesContent";
import { recipeNavigation } from "../shared/navigation";

export default function SharedRecipesPage() {
  return (
    <PageLayout
      title="Shared with Me"
      breadcrumbs={[
        { name: "Shared with Me", href: "/shared-recipes", current: true },
      ]}
      sidebarNavigation={recipeNavigation}
    >
      <SharedRecipesContent />
    </PageLayout>
  );
}
