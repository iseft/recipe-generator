import { Outlet } from "react-router-dom";
import Navigation from "./Navigation";

export default function AppShell() {
  return (
    <div className="min-h-full">
      <Navigation />
      <Outlet />
    </div>
  );
}
