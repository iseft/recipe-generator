import { Link } from "react-router-dom";
import CardWithHeader from "../ui/CardWithHeader";
import type { Recipe } from "../../../pages/generate-recipe/types";

interface RecipeCardProps {
  recipe: Recipe;
}

export default function RecipeCard({ recipe }: RecipeCardProps) {
  return (
    <CardWithHeader
      header={
        <div className="flex items-center justify-between">
          {recipe.id ? (
            <Link
              to={`/recipes/${recipe.id}`}
              className="text-lg font-semibold text-gray-900 dark:text-white hover:text-indigo-600 dark:hover:text-indigo-400 transition-colors"
            >
              {recipe.title}
            </Link>
          ) : (
            <h2 className="text-lg font-semibold text-gray-900 dark:text-white">
              {recipe.title}
            </h2>
          )}
          <div className="flex gap-4 text-sm text-gray-500 dark:text-gray-400">
            {recipe.prepTimeMinutes && (
              <span>Prep: {recipe.prepTimeMinutes}min</span>
            )}
            {recipe.cookTimeMinutes && (
              <span>Cook: {recipe.cookTimeMinutes}min</span>
            )}
            {recipe.servings && <span>Serves: {recipe.servings}</span>}
          </div>
        </div>
      }
    >
      <div className="space-y-6">
        <section>
          <h3 className="text-sm font-medium text-gray-900 dark:text-white mb-2">
            Ingredients
          </h3>
          <ul className="list-disc list-inside space-y-1 text-sm text-gray-600 dark:text-gray-300">
            {recipe.ingredients.map((ingredient, index) => (
              <li key={index}>{ingredient}</li>
            ))}
          </ul>
        </section>

        <section>
          <h3 className="text-sm font-medium text-gray-900 dark:text-white mb-2">
            Instructions
          </h3>
          <ol className="list-decimal list-inside space-y-2 text-sm text-gray-600 dark:text-gray-300">
            {recipe.instructions.map((step, index) => (
              <li key={index}>{step}</li>
            ))}
          </ol>
        </section>
      </div>
    </CardWithHeader>
  );
}
