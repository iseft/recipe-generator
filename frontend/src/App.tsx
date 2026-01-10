import { BrowserRouter, Routes, Route } from "react-router-dom";
import AppShell from "./shared/components/AppShell";
import GenerateRecipe from "./pages/generate-recipe";
import "./index.css";

export default function App() {
  return (
    <BrowserRouter>
      <Routes>
        <Route element={<AppShell />}>
          <Route index element={<GenerateRecipe />} />
        </Route>
      </Routes>
    </BrowserRouter>
  );
}
