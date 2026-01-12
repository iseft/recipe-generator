import { Routes, Route } from "react-router-dom";
import AppShell from "../shared/components/AppShell";
import { PublicRoutes } from "./PublicRoutes";
import { ProtectedRoutes } from "./ProtectedRoutes";
import GenerateRecipePage from "../pages/generate-recipe";
import NotFoundPage from "../pages/NotFoundPage";

export function AppRoutes() {
  return (
    <Routes>
      {...PublicRoutes()}
      <Route element={<AppShell />}>
        <Route index element={<GenerateRecipePage />} />
      </Route>
      {...ProtectedRoutes()}
      <Route path="*" element={<NotFoundPage />} />
    </Routes>
  );
}
