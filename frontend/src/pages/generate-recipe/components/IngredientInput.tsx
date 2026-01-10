import { useState } from "react";
import { ExclamationCircleIcon } from "@heroicons/react/16/solid";
import PrimaryButton from "../../../shared/components/PrimaryButton";

interface IngredientInputProps {
  onSubmit: (ingredients: string[]) => void;
  isLoading?: boolean;
  error?: string | null;
}

export default function IngredientInput({
  onSubmit,
  isLoading,
  error,
}: IngredientInputProps) {
  const [value, setValue] = useState("");

  const handleSubmit = (e: React.FormEvent) => {
    e.preventDefault();
    const ingredients = value
      .split(",")
      .map((s) => s.trim())
      .filter((s) => s.length > 0);
    if (ingredients.length > 0) {
      onSubmit(ingredients);
    }
  };

  const hasError = !!error;
  const inputClasses = hasError
    ? "col-start-1 row-start-1 block w-full rounded-md bg-white py-1.5 pr-10 pl-3 text-red-900 outline-1 -outline-offset-1 outline-red-300 placeholder:text-red-300 focus:outline-2 focus:-outline-offset-2 focus:outline-red-600 sm:pr-9 sm:text-sm/6 dark:bg-white/5 dark:text-red-400 dark:outline-red-500/50 dark:placeholder:text-red-400/70 dark:focus:outline-red-400"
    : "col-start-1 row-start-1 block w-full rounded-md bg-white py-1.5 pr-3 pl-3 text-gray-900 outline-1 -outline-offset-1 outline-gray-300 placeholder:text-gray-400 focus:outline-2 focus:-outline-offset-2 focus:outline-indigo-600 sm:text-sm/6 dark:bg-white/5 dark:text-white dark:outline-white/10 dark:placeholder:text-gray-500 dark:focus:outline-indigo-500";

  return (
    <form onSubmit={handleSubmit}>
      <label
        htmlFor="ingredients"
        className="block text-sm/6 font-medium text-gray-900 dark:text-white"
      >
        Ingredients
      </label>
      <div className="mt-2 grid grid-cols-1">
        <input
          id="ingredients"
          name="ingredients"
          type="text"
          value={value}
          onChange={(e) => setValue(e.target.value)}
          placeholder="chicken, rice, garlic, onion..."
          aria-invalid={hasError}
          aria-describedby={hasError ? "ingredients-error" : undefined}
          className={inputClasses}
          disabled={isLoading}
        />
        {hasError && (
          <ExclamationCircleIcon
            aria-hidden="true"
            className="pointer-events-none col-start-1 row-start-1 mr-3 size-5 self-center justify-self-end text-red-500 sm:size-4 dark:text-red-400"
          />
        )}
      </div>
      {hasError && (
        <p
          id="ingredients-error"
          className="mt-2 text-sm text-red-600 dark:text-red-400"
        >
          {error}
        </p>
      )}
      <PrimaryButton
        type="submit"
        size="lg"
        disabled={isLoading || value.trim().length === 0}
        className="mt-4 w-full"
      >
        {isLoading ? "Generating..." : "Generate Recipe"}
      </PrimaryButton>
    </form>
  );
}
