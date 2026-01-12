import PageLayout from "../../../shared/components/PageLayout";
import GenerateRecipeContent from "./components/GenerateRecipeContent";

export default function GenerateRecipePage() {
  return (
    <PageLayout
      title="Recipe Generator"
      breadcrumbs={[{ name: "Recipe Generator", href: "/", current: true }]}
    >
      <GenerateRecipeContent />
    </PageLayout>
  );
}
