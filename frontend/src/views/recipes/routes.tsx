import { Route } from "react-router-dom";
import GenerateRecipePage from "./generate-recipe";
import MyRecipesPage from "./my-recipes";
import MyRecipeDetailPage from "./my-recipes/recipe-detail";
import SharedRecipesPage from "./shared-recipes";
import SharedRecipeDetailPage from "./shared-recipes/recipe-detail";

export function PublicRecipeRoutes(): React.ReactElement[] {
  return [<Route key="generate" index element={<GenerateRecipePage />} />];
}

export function ProtectedRecipeRoutes(): React.ReactElement[] {
  return [
    <Route key="my-recipes" path="my-recipes" element={<MyRecipesPage />} />,
    <Route
      key="my-recipes-detail"
      path="my-recipes/:id"
      element={<MyRecipeDetailPage />}
    />,
    <Route
      key="shared-recipes"
      path="shared-recipes"
      element={<SharedRecipesPage />}
    />,
    <Route
      key="shared-recipes-detail"
      path="shared-recipes/:id"
      element={<SharedRecipeDetailPage />}
    />,
  ];
}
