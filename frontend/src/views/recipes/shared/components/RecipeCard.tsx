import { Link } from "react-router-dom";
import ShareRecipeButton from "./ShareRecipeButton";
import RecipeCardFooter from "./RecipeCardFooter";
import type { Recipe } from "../../generate-recipe/types";

interface RecipeCardProps {
  recipe: Recipe;
  linkTo?: string;
  showSharedBy?: boolean;
  showShareButton?: boolean;
  showFooter?: boolean;
}

export default function RecipeCard({
  recipe,
  linkTo,
  showSharedBy = false,
  showShareButton = false,
  showFooter = false,
}: RecipeCardProps) {
  return (
    <div className="divide-y divide-gray-200 overflow-hidden rounded-lg bg-white shadow-sm dark:divide-white/10 dark:bg-gray-800/50 dark:shadow-none dark:outline dark:-outline-offset-1 dark:outline-white/10">
      <div className="px-4 py-5 sm:px-6">
        <div className="flex items-center justify-between">
          <div className="flex-1">
            <div>
              {linkTo ? (
                <Link
                  to={linkTo}
                  className="text-lg font-semibold text-gray-900 dark:text-white hover:text-indigo-600 dark:hover:text-indigo-400 transition-colors"
                >
                  {recipe.title}
                </Link>
              ) : (
                <h2 className="text-lg font-semibold text-gray-900 dark:text-white">
                  {recipe.title}
                </h2>
              )}
              {showSharedBy && recipe.ownerEmail && (
                <p className="text-xs text-gray-500 dark:text-gray-400 mt-1">
                  Shared by {recipe.ownerEmail}
                </p>
              )}
            </div>
          </div>
          <div className="flex items-center gap-4">
            <div className="flex gap-4 text-sm text-gray-500 dark:text-gray-400">
              {recipe.prepTimeMinutes && (
                <span>Prep: {recipe.prepTimeMinutes}min</span>
              )}
              {recipe.cookTimeMinutes && (
                <span>Cook: {recipe.cookTimeMinutes}min</span>
              )}
              {recipe.servings && <span>Serves: {recipe.servings}</span>}
            </div>
            {showShareButton && recipe.id && (
              <ShareRecipeButton recipeId={recipe.id} />
            )}
          </div>
        </div>
      </div>
      <div className="px-4 py-5 sm:p-6">
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
      </div>
      {showFooter && recipe.id && <RecipeCardFooter recipeId={recipe.id} />}
    </div>
  );
}
