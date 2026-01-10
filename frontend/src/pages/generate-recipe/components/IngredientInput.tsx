import { useForm } from "react-hook-form";
import { zodResolver } from "@hookform/resolvers/zod";
import { z } from "zod";
import { ExclamationCircleIcon } from "@heroicons/react/16/solid";
import PrimaryButton from "../../../shared/components/PrimaryButton";
import type { GenerateRecipeRequest } from "../types";

const schema = z.object({
  ingredients: z.string().min(1, "At least one ingredient is required"),
  dietaryRestrictions: z.string().optional(),
});

type FormData = z.infer<typeof schema>;

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
  } = useForm<FormData>({
    resolver: zodResolver(schema),
    mode: "onChange",
  });

  const onFormSubmit = (data: FormData) => {
    const ingredients = parseCommaSeparated(data.ingredients);
    if (ingredients.length === 0) return;

    onSubmit({
      ingredients,
      dietaryRestrictions: data.dietaryRestrictions
        ? parseCommaSeparated(data.dietaryRestrictions)
        : undefined,
    });
  };

  const ingredientsError = errors.ingredients?.message || error;
  const hasIngredientsError = !!ingredientsError;

  const inputBaseClasses =
    "col-start-1 row-start-1 block w-full rounded-md py-1.5 pl-3 sm:text-sm/6 dark:bg-white/5";
  const inputNormalClasses =
    "bg-white pr-3 text-gray-900 outline-1 -outline-offset-1 outline-gray-300 placeholder:text-gray-400 focus:outline-2 focus:-outline-offset-2 focus:outline-indigo-600 dark:text-white dark:outline-white/10 dark:placeholder:text-gray-500 dark:focus:outline-indigo-500";
  const inputErrorClasses =
    "bg-white pr-10 text-red-900 outline-1 -outline-offset-1 outline-red-300 placeholder:text-red-300 focus:outline-2 focus:-outline-offset-2 focus:outline-red-600 sm:pr-9 dark:text-red-400 dark:outline-red-500/50 dark:placeholder:text-red-400/70 dark:focus:outline-red-400";

  return (
    <form onSubmit={handleSubmit(onFormSubmit)} className="space-y-4">
      <div>
        <label
          htmlFor="ingredients"
          className="block text-sm/6 font-medium text-gray-900 dark:text-white"
        >
          Ingredients
        </label>
        <div className="mt-2 grid grid-cols-1">
          <input
            id="ingredients"
            type="text"
            placeholder="chicken, rice, garlic, onion..."
            aria-invalid={hasIngredientsError}
            aria-describedby={
              hasIngredientsError ? "ingredients-error" : undefined
            }
            className={`${inputBaseClasses} ${
              hasIngredientsError ? inputErrorClasses : inputNormalClasses
            }`}
            disabled={isLoading}
            {...register("ingredients")}
          />
          {hasIngredientsError && (
            <ExclamationCircleIcon
              aria-hidden="true"
              className="pointer-events-none col-start-1 row-start-1 mr-3 size-5 self-center justify-self-end text-red-500 sm:size-4 dark:text-red-400"
            />
          )}
        </div>
        {hasIngredientsError && (
          <p
            id="ingredients-error"
            className="mt-2 text-sm text-red-600 dark:text-red-400"
          >
            {ingredientsError}
          </p>
        )}
      </div>

      <div>
        <label
          htmlFor="dietaryRestrictions"
          className="block text-sm/6 font-medium text-gray-900 dark:text-white"
        >
          Dietary Restrictions (optional)
        </label>
        <div className="mt-2">
          <input
            id="dietaryRestrictions"
            type="text"
            placeholder="vegan, gluten-free, dairy-free..."
            className={`${inputBaseClasses} ${inputNormalClasses}`}
            disabled={isLoading}
            {...register("dietaryRestrictions")}
          />
        </div>
      </div>

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
