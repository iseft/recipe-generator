import { Outlet } from "react-router-dom";
import Navigation from "./Navigation";

export default function AppShell() {
  return (
    <div className="min-h-full pt-16">
      <Navigation />
      <Outlet />
    </div>
  );
}
