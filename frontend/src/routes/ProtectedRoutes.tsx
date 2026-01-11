import { Route } from "react-router-dom";
import AppShell from "../shared/components/AppShell";
import ProtectedRoute from "../shared/components/ProtectedRoute";
import GenerateRecipePage from "../pages/generate-recipe";
import MyRecipesPage from "../pages/my-recipes";
import MyRecipeDetailPage from "../pages/my-recipes/recipe-detail";
import SharedRecipesPage from "../pages/shared-recipes";
import SharedRecipeDetailPage from "../pages/shared-recipes/recipe-detail";

export function ProtectedRoutes() {
  return [
    <Route
      key="protected"
      element={
        <ProtectedRoute>
          <AppShell />
        </ProtectedRoute>
      }
    >
      <Route index element={<GenerateRecipePage />} />
      <Route path="my-recipes" element={<MyRecipesPage />} />
      <Route path="my-recipes/:id" element={<MyRecipeDetailPage />} />
      <Route path="shared-recipes" element={<SharedRecipesPage />} />
      <Route path="shared-recipes/:id" element={<SharedRecipeDetailPage />} />
    </Route>,
  ];
}
