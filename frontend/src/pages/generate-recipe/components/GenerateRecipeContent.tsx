import { useGenerateRecipe } from "../hooks/useGenerateRecipe";
import { useSaveRecipe } from "../hooks/useSaveRecipe";
import IngredientInput from "./IngredientInput";
import RecipeCard from "../../../shared/components/recipe/RecipeCard";
import PrimaryButton from "../../../shared/components/ui/PrimaryButton";
import LoadingState from "../../../shared/components/ui/LoadingState";
import Breadcrumbs from "../../../shared/components/ui/Breadcrumbs";
import type { GenerateRecipeRequest } from "../types";

export default function GenerateRecipeContent() {
  const { mutate, data, isPending, error } = useGenerateRecipe();
  const {
    mutate: saveRecipe,
    isPending: isSaving,
    isSuccess: isSaved,
  } = useSaveRecipe();

  const handleSubmit = (request: GenerateRecipeRequest) => {
    mutate(request);
  };

  const handleSave = () => {
    if (data) {
      saveRecipe(data);
    }
  };

  return (
    <div className="space-y-6">
      <Breadcrumbs
        items={[{ name: "Recipe Generator", href: "/", current: true }]}
      />
      <IngredientInput
        onSubmit={handleSubmit}
        isLoading={isPending}
        error={error?.message ?? null}
      />

      {isPending && <LoadingState message="Generating your recipe..." />}

      {data && !isPending && (
        <>
          <RecipeCard recipe={data} />
          {!data.id && (
            <div className="pt-4">
              <PrimaryButton
                onClick={handleSave}
                disabled={isSaving || isSaved}
                className="w-full"
              >
                {isSaving ? "Saving..." : isSaved ? "Saved!" : "Save Recipe"}
              </PrimaryButton>
            </div>
          )}
        </>
      )}
    </div>
  );
}
