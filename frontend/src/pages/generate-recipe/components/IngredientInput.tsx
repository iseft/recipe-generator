import { useForm } from "react-hook-form";
import { zodResolver } from "@hookform/resolvers/zod";
import PrimaryButton from "../../../shared/components/ui/PrimaryButton";
import Input from "../../../shared/components/ui/Input";
import {
  generateRecipeFormSchema,
  type GenerateRecipeFormData,
  type GenerateRecipeRequest,
} from "../types";

interface IngredientInputProps {
  onSubmit: (data: GenerateRecipeRequest) => void;
  isLoading?: boolean;
  error?: string | null;
}

export default function IngredientInput({
  onSubmit,
  isLoading,
  error,
}: IngredientInputProps) {
  const {
    register,
    handleSubmit,
    formState: { errors, isValid },
  } = useForm<GenerateRecipeFormData, unknown, GenerateRecipeRequest>({
    resolver: zodResolver(generateRecipeFormSchema),
    mode: "onChange",
  });

  return (
    <form onSubmit={handleSubmit(onSubmit)} className="space-y-4">
      <Input
        label="Ingredients"
        placeholder="chicken, rice, garlic, onion..."
        error={errors.ingredients?.message || error || undefined}
        disabled={isLoading}
        {...register("ingredients")}
      />

      <Input
        label="Dietary Restrictions (optional)"
        placeholder="vegan, gluten-free, dairy-free..."
        disabled={isLoading}
        {...register("dietaryRestrictions")}
      />

      <PrimaryButton
        type="submit"
        size="lg"
        disabled={isLoading || !isValid || !!error}
        className="w-full"
      >
        {isLoading ? "Generating..." : "Generate Recipe"}
      </PrimaryButton>
    </form>
  );
}
