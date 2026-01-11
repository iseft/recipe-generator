import { classNames } from "../../utils/classNames";

type ButtonSize = "xs" | "sm" | "md" | "lg" | "xl";

const sizeClasses: Record<ButtonSize, string> = {
  xs: "rounded-sm px-2 py-1 text-xs",
  sm: "rounded-sm px-2 py-1 text-sm",
  md: "rounded-md px-2.5 py-1.5 text-sm",
  lg: "rounded-md px-3 py-2 text-sm",
  xl: "rounded-md px-3.5 py-2.5 text-sm",
};

const baseClasses =
  "font-semibold text-white shadow-xs bg-indigo-600 hover:bg-indigo-500 focus-visible:outline-2 focus-visible:outline-offset-2 focus-visible:outline-indigo-600 dark:bg-indigo-500 dark:shadow-none dark:hover:bg-indigo-400 dark:focus-visible:outline-indigo-500 disabled:opacity-50 disabled:cursor-not-allowed";

type ButtonBaseProps = {
  size?: ButtonSize;
  children: React.ReactNode;
  className?: string;
  disabled?: boolean;
};

type ButtonAsButton = ButtonBaseProps & {
  href?: never;
  onClick?: () => void;
  type?: "button" | "submit" | "reset";
};

type ButtonAsLink = ButtonBaseProps & {
  href: string;
  onClick?: never;
  type?: never;
};

type ButtonProps = ButtonAsButton | ButtonAsLink;

export default function PrimaryButton({
  size = "md",
  children,
  className,
  disabled,
  href,
  onClick,
  type = "button",
}: ButtonProps) {
  const classes = classNames(baseClasses, sizeClasses[size], className);

  if (href) {
    return (
      <a href={href} className={classes}>
        {children}
      </a>
    );
  }

  return (
    <button
      type={type}
      onClick={onClick}
      disabled={disabled}
      className={classes}
    >
      {children}
    </button>
  );
}
