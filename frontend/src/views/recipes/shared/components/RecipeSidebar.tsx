import {
  SparklesIcon,
  BookOpenIcon,
  ShareIcon,
} from "@heroicons/react/24/outline";
import SidebarLayout from "../../../../shared/components/SidebarLayout";

interface RecipeSidebarProps {
  sidebarOpen: boolean;
  setSidebarOpen: (open: boolean) => void;
}

const recipeNavigation = [
  { name: "Generate Recipe", href: "/", icon: SparklesIcon },
  { name: "My Recipes", href: "/my-recipes", icon: BookOpenIcon },
  { name: "Shared with Me", href: "/shared-recipes", icon: ShareIcon },
];

export default function RecipeSidebar({
  sidebarOpen,
  setSidebarOpen,
}: RecipeSidebarProps) {
  return (
    <SidebarLayout
      navigation={recipeNavigation}
      sidebarOpen={sidebarOpen}
      setSidebarOpen={setSidebarOpen}
    />
  );
}
