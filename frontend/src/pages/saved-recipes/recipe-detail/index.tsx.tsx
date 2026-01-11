import { useParams } from "react-router-dom";
import PageLayout from "../../../shared/components/PageLayout";
import RecipeDetailContent from "./components/RecipeDetailContent";

export default function RecipeDetailPage() {
  const { id } = useParams<{ id: string }>();

  return (
    <PageLayout title="Recipe Details">
      <RecipeDetailContent id={id} />
    </PageLayout>
  );
}
