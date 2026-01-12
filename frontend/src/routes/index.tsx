import { Routes, Route } from "react-router-dom";
import AppShell from "../shared/components/AppShell";
import ProtectedRoute from "./ProtectedRoute";
import { AuthRoutes } from "../views/auth/routes";
import {
  PublicRecipeRoutes,
  ProtectedRecipeRoutes,
} from "../views/recipes/routes";
import NotFoundPage from "../views/not-found/NotFoundPage";

export function AppRoutes() {
  return (
    <Routes>
      {...AuthRoutes()}
      <Route element={<AppShell />}>{...PublicRecipeRoutes()}</Route>
      <Route
        element={
          <ProtectedRoute>
            <AppShell />
          </ProtectedRoute>
        }
      >
        {...ProtectedRecipeRoutes()}
      </Route>
      <Route path="*" element={<NotFoundPage />} />
    </Routes>
  );
}
