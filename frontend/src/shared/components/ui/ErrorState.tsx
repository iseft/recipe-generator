import { Link } from "react-router-dom";

interface ErrorStateProps {
  message?: string;
  variant?: "error" | "info";
  backLink?: {
    to: string;
    text: string;
  };
}

export default function ErrorState({
  message = "Something went wrong. Please try again.",
  variant = "error",
  backLink,
}: ErrorStateProps) {
  const messageClassName =
    variant === "error"
      ? "text-red-600 dark:text-red-400"
      : "text-gray-500 dark:text-gray-400";

  return (
    <div className="text-center py-12 space-y-4">
      <p className={messageClassName}>{message}</p>
      {backLink && (
        <Link
          to={backLink.to}
          className="text-indigo-600 hover:text-indigo-500 dark:text-indigo-400 dark:hover:text-indigo-300"
        >
          {backLink.text}
        </Link>
      )}
    </div>
  );
}
