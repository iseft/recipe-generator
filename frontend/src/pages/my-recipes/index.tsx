import PageLayout from "../../shared/components/PageLayout";
import MyRecipesContent from "./components/MyRecipesContent";

export default function MyRecipesPage() {
  return (
    <PageLayout
      title="My Recipes"
      breadcrumbs={[{ name: "My Recipes", href: "/my-recipes", current: true }]}
    >
      <MyRecipesContent />
    </PageLayout>
  );
}
