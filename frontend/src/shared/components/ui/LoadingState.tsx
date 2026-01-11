import Spinner from "./Spinner";

interface LoadingStateProps {
  message?: string;
}

export default function LoadingState({
  message = "Loading...",
}: LoadingStateProps) {
  return (
    <div className="flex items-center justify-center py-12">
      <Spinner size="md" />
      <span className="ml-3 text-sm text-gray-500 dark:text-gray-400">
        {message}
      </span>
    </div>
  );
}
