import { useState } from "react";
import { ShareIcon } from "@heroicons/react/24/outline";
import ShareRecipeModal from "./ShareRecipeModal";

interface ShareButtonProps {
  recipeId: string;
}

export default function ShareRecipeButton({ recipeId }: ShareButtonProps) {
  const [showShareModal, setShowShareModal] = useState(false);

  return (
    <>
      <button
        onClick={() => setShowShareModal(true)}
        className="inline-flex items-center px-3 py-1.5 border border-transparent text-sm font-medium rounded-md shadow-sm text-white bg-indigo-600 hover:bg-indigo-700 focus:outline-none focus:ring-2 focus:ring-offset-2 focus:ring-indigo-500"
        title="Share recipe"
      >
        <ShareIcon className="h-5 w-5" />
      </button>

      <ShareRecipeModal
        recipeId={recipeId}
        open={showShareModal}
        onClose={() => setShowShareModal(false)}
      />
    </>
  );
}
