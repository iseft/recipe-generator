import { BrowserRouter, Routes, Route } from "react-router-dom";
import AppShell from "./shared/components/AppShell";
import GenerateRecipePage from "./pages/generate-recipe";
import SavedRecipesPage from "./pages/saved-recipes";
import RecipeDetailPage from "./pages/saved-recipes/recipe-detail/index.tsx";
import "./index.css";

export default function App() {
  return (
    <BrowserRouter>
      <Routes>
        <Route element={<AppShell />}>
          <Route index element={<GenerateRecipePage />} />
          <Route path="recipes" element={<SavedRecipesPage />} />
          <Route path="recipes/:id" element={<RecipeDetailPage />} />
        </Route>
      </Routes>
    </BrowserRouter>
  );
}
