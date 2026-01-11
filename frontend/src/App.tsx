import { BrowserRouter } from "react-router-dom";
import { AppRoutes } from "./routes";
import { useAuthSetup } from "./shared/hooks/useAuthSetup";
import "./index.css";

export default function App() {
  useAuthSetup();

  return (
    <BrowserRouter>
      <AppRoutes />
    </BrowserRouter>
  );
}
