import { useForm } from "react-hook-form";
import { zodResolver } from "@hookform/resolvers/zod";
import PrimaryButton from "../../../shared/components/PrimaryButton";
import Input from "../../../shared/components/Input";
import {
  generateRecipeFormSchema,
  type GenerateRecipeFormData,
  type GenerateRecipeRequest,
} from "../types";

function parseCommaSeparated(value: string): string[] {
  return value
    .split(",")
    .map((s) => s.trim())
    .filter((s) => s.length > 0);
}

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
  } = useForm<GenerateRecipeFormData>({
    resolver: zodResolver(generateRecipeFormSchema),
    mode: "onChange",
  });

  const onFormSubmit = (data: GenerateRecipeFormData) => {
    const ingredients = parseCommaSeparated(data.ingredients);
    if (ingredients.length === 0) return;

    onSubmit({
      ingredients,
      dietaryRestrictions: data.dietaryRestrictions
        ? parseCommaSeparated(data.dietaryRestrictions)
        : undefined,
    });
  };

  return (
    <form onSubmit={handleSubmit(onFormSubmit)} className="space-y-4">
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
