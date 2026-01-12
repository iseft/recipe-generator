import { useForm } from "react-hook-form";
import { zodResolver } from "@hookform/resolvers/zod";
import { z } from "zod";
import { useShareRecipe } from "../../../../shared/hooks/useShareRecipe";
import Input from "../../../../shared/components/ui/Input";
import PrimaryButton from "../../../../shared/components/ui/PrimaryButton";

const shareRecipeFormSchema = z.object({
  email: z.email("Please enter a valid email address"),
});

type ShareRecipeFormData = z.infer<typeof shareRecipeFormSchema>;

interface ShareRecipeModalProps {
  recipeId: string;
  onClose: () => void;
}

export default function ShareRecipeModal({
  recipeId,
  onClose,
}: ShareRecipeModalProps) {
  const {
    register,
    handleSubmit,
    formState: { errors, isValid },
    reset,
  } = useForm<ShareRecipeFormData>({
    resolver: zodResolver(shareRecipeFormSchema),
    mode: "onChange",
  });

  const { mutate: shareRecipe, isPending, error } = useShareRecipe(recipeId);

  const onSubmit = (data: ShareRecipeFormData) => {
    shareRecipe(
      { email: data.email.trim() },
      {
        onSuccess: () => {
          reset();
          onClose();
        },
      }
    );
  };

  return (
    <div className="fixed inset-0 bg-black bg-opacity-50 flex items-center justify-center z-50">
      <div className="bg-white dark:bg-gray-800 rounded-lg p-6 max-w-md w-full mx-4">
        <h2 className="text-xl font-semibold text-gray-900 dark:text-white mb-4">
          Share Recipe
        </h2>

        <form onSubmit={handleSubmit(onSubmit)} className="space-y-4">
          <Input
            label="Email Address"
            type="email"
            placeholder="user@example.com"
            error={
              errors.email?.message ||
              (error
                ? "Failed to share recipe. User may not exist or already has access."
                : undefined)
            }
            disabled={isPending}
            {...register("email")}
          />

          <p className="text-xs text-gray-500 dark:text-gray-400">
            The user must have an account on this app
          </p>

          <div className="flex justify-end gap-3 pt-2">
            <PrimaryButton
              type="button"
              size="sm"
              onClick={onClose}
              disabled={isPending}
              className="bg-gray-600 hover:bg-gray-700 dark:bg-gray-700 dark:hover:bg-gray-600"
            >
              Cancel
            </PrimaryButton>
            <PrimaryButton
              type="submit"
              size="sm"
              disabled={isPending || !isValid}
            >
              {isPending ? "Sharing..." : "Share"}
            </PrimaryButton>
          </div>
        </form>
      </div>
    </div>
  );
}
