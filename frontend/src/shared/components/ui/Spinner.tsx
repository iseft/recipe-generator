import { classNames } from "../../utils/classNames";

type SpinnerSize = "sm" | "md" | "lg";

const sizeClasses: Record<SpinnerSize, string> = {
  sm: "h-4 w-4",
  md: "h-8 w-8",
  lg: "h-12 w-12",
};

interface SpinnerProps {
  size?: SpinnerSize;
  className?: string;
}

export default function Spinner({ size = "md", className }: SpinnerProps) {
  return (
    <div
      className={classNames(
        "animate-spin rounded-full border-b-2 border-indigo-600 dark:border-indigo-400",
        sizeClasses[size],
        className
      )}
    />
  );
}
