import { useState } from "react";
import { TrashIcon } from "@heroicons/react/24/outline";
import { useRecipeShares, type Share } from "../hooks/useRecipeShares";
import { useDeleteShare } from "../hooks/useDeleteShare";
import LoadingState from "../../../../shared/components/ui/LoadingState";
import ConfirmDialog from "../../../../shared/components/ui/ConfirmDialog";

interface RecipeCardFooterProps {
  recipeId: string;
}

export default function RecipeCardFooter({ recipeId }: RecipeCardFooterProps) {
  const {
    data: shares,
    isLoading: sharesLoading,
    error: sharesError,
  } = useRecipeShares(recipeId);
  const { mutate: deleteShare, isPending: isDeleting } =
    useDeleteShare(recipeId);
  const [confirmDialog, setConfirmDialog] = useState<{
    open: boolean;
    userId: string | null;
    userEmail: string | null;
  }>({
    open: false,
    userId: null,
    userEmail: null,
  });

  const handleRemoveShare = (userId: string, userEmail: string) => {
    setConfirmDialog({
      open: true,
      userId,
      userEmail,
    });
  };

  const handleConfirmDelete = () => {
    if (confirmDialog.userId) {
      deleteShare(confirmDialog.userId);
    }
  };

  return (
    <>
      <div className="px-4 py-5 sm:px-6 border-t border-gray-200 dark:border-white/10">
        <h3 className="text-sm font-medium text-gray-900 dark:text-white mb-3">
          Shared With
        </h3>
        {sharesLoading ? (
          <LoadingState message="Loading shared users..." />
        ) : sharesError ? (
          <p className="text-sm text-red-600 dark:text-red-400">
            Failed to load shared users.
          </p>
        ) : !shares || shares.length === 0 ? (
          <p className="text-sm text-gray-500 dark:text-gray-400">
            This recipe is not shared with anyone yet.
          </p>
        ) : (
          <ul className="space-y-3">
            {shares.map((share: Share) => (
              <li
                key={share.userId}
                className="flex items-center justify-between py-2 border-b border-gray-200 dark:border-white/10 last:border-0"
              >
                <div className="flex-1 min-w-0">
                  <p className="text-sm font-medium text-gray-900 dark:text-white">
                    {share.email}
                  </p>
                  <p className="text-xs text-gray-500 dark:text-gray-400">
                    Shared on {new Date(share.createdAt).toLocaleDateString()}
                  </p>
                </div>
                <button
                  onClick={() => handleRemoveShare(share.userId, share.email)}
                  disabled={isDeleting}
                  className="ml-4 p-2 text-red-600 hover:text-red-700 hover:bg-red-50 dark:text-red-400 dark:hover:text-red-300 dark:hover:bg-red-900/20 rounded-md transition-colors disabled:opacity-50 disabled:cursor-not-allowed"
                  title="Remove sharing access"
                >
                  <TrashIcon className="h-5 w-5" />
                </button>
              </li>
            ))}
          </ul>
        )}
      </div>
      <ConfirmDialog
        open={confirmDialog.open}
        onClose={() =>
          setConfirmDialog({ open: false, userId: null, userEmail: null })
        }
        onConfirm={handleConfirmDelete}
        title="Remove Sharing Access"
        message={`Are you sure you want to remove sharing access for ${confirmDialog.userEmail}? They will no longer be able to view this recipe.`}
        confirmText="Remove Access"
        cancelText="Cancel"
        isLoading={isDeleting}
      />
    </>
  );
}
