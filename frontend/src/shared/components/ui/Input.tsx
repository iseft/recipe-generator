import { forwardRef } from "react";
import { ExclamationCircleIcon } from "@heroicons/react/16/solid";
import { classNames } from "../../utils/classNames";

const baseClasses =
  "block w-full rounded-md py-1.5 pl-3 sm:text-sm/6 dark:bg-white/5";

const normalClasses =
  "bg-white pr-3 text-gray-900 outline-1 -outline-offset-1 outline-gray-300 placeholder:text-gray-400 focus:outline-2 focus:-outline-offset-2 focus:outline-indigo-600 dark:text-white dark:outline-white/10 dark:placeholder:text-gray-500 dark:focus:outline-indigo-500";

const errorClasses =
  "bg-white pr-10 text-red-900 outline-1 -outline-offset-1 outline-red-300 placeholder:text-red-300 focus:outline-2 focus:-outline-offset-2 focus:outline-red-600 sm:pr-9 dark:text-red-400 dark:outline-red-500/50 dark:placeholder:text-red-400/70 dark:focus:outline-red-400";

interface InputProps extends React.InputHTMLAttributes<HTMLInputElement> {
  label: string;
  error?: string;
}

const Input = forwardRef<HTMLInputElement, InputProps>(
  ({ label, error, id, className, ...props }, ref) => {
    const hasError = !!error;
    const inputId = id || label.toLowerCase().replace(/\s+/g, "-");

    return (
      <div>
        <label
          htmlFor={inputId}
          className="block text-sm/6 font-medium text-gray-900 dark:text-white"
        >
          {label}
        </label>
        <div className="mt-2 grid grid-cols-1">
          <input
            ref={ref}
            id={inputId}
            aria-invalid={hasError}
            aria-describedby={hasError ? `${inputId}-error` : undefined}
            className={classNames(
              baseClasses,
              hasError ? errorClasses : normalClasses,
              hasError ? "col-start-1 row-start-1" : "",
              className
            )}
            {...props}
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
            id={`${inputId}-error`}
            className="mt-2 text-sm text-red-600 dark:text-red-400"
          >
            {error}
          </p>
        )}
      </div>
    );
  }
);

Input.displayName = "Input";

export default Input;
