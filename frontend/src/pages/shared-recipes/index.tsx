import PageLayout from "../../shared/components/PageLayout";
import SharedRecipesContent from "./components/SharedRecipesContent";

export default function SharedRecipesPage() {
  return (
    <PageLayout
      title="Shared with Me"
      breadcrumbs={[
        { name: "Shared with Me", href: "/shared-recipes", current: true },
      ]}
    >
      <SharedRecipesContent />
    </PageLayout>
  );
}
