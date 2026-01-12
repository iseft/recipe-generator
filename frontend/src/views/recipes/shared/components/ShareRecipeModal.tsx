import { useForm } from "react-hook-form";
import { zodResolver } from "@hookform/resolvers/zod";
import { z } from "zod";
import { useShareRecipe } from "../hooks/useShareRecipe";
import Input from "../../../../shared/components/ui/Input";
import Modal from "../../../../shared/components/ui/Modal";
import ModalFooter from "../../../../shared/components/ui/ModalFooter";

const shareRecipeFormSchema = z.object({
  email: z.email("Please enter a valid email address"),
});

type ShareRecipeFormData = z.infer<typeof shareRecipeFormSchema>;

interface ShareRecipeModalProps {
  recipeId: string;
  open: boolean;
  onClose: () => void;
}

export default function ShareRecipeModal({
  recipeId,
  open,
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
    <Modal
      open={open}
      onClose={onClose}
      title="Share Recipe"
      footer={
        <ModalFooter
          primaryButton={{
            label: isPending ? "Sharing..." : "Share",
            onClick: () => {
              // Trigger form submission
              const form = document.getElementById(
                "share-recipe-form"
              ) as HTMLFormElement;
              form?.requestSubmit();
            },
            disabled: isPending || !isValid,
          }}
          secondaryButton={{
            label: "Cancel",
            onClick: onClose,
          }}
        />
      }
    >
      <form
        id="share-recipe-form"
        onSubmit={handleSubmit(onSubmit)}
        className="space-y-4"
      >
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
      </form>
    </Modal>
  );
}
