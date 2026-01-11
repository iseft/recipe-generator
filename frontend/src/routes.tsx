import { Routes, Route } from "react-router-dom";
import { PublicRoutes } from "./routes/PublicRoutes";
import { ProtectedRoutes } from "./routes/ProtectedRoutes";
import NotFoundPage from "./pages/NotFoundPage";

export function AppRoutes() {
  return (
    <Routes>
      {...PublicRoutes()}
      {...ProtectedRoutes()}
      <Route path="*" element={<NotFoundPage />} />
    </Routes>
  );
}
