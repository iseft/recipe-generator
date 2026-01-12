import { Route } from "react-router-dom";
import AppShell from "../shared/components/AppShell";
import ProtectedRoute from "../shared/components/ProtectedRoute";
import MyRecipesPage from "../views/recipes/my-recipes";
import MyRecipeDetailPage from "../views/recipes/my-recipes/recipe-detail";
import SharedRecipesPage from "../views/recipes/shared-recipes";
import SharedRecipeDetailPage from "../views/recipes/shared-recipes/recipe-detail";

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
      <Route path="my-recipes" element={<MyRecipesPage />} />
      <Route path="my-recipes/:id" element={<MyRecipeDetailPage />} />
      <Route path="shared-recipes" element={<SharedRecipesPage />} />
      <Route path="shared-recipes/:id" element={<SharedRecipeDetailPage />} />
    </Route>,
  ];
}
