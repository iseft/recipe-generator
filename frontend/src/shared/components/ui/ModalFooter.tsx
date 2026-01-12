import PrimaryButton from "./PrimaryButton";

interface ModalFooterProps {
  primaryButton: {
    label: string;
    onClick: () => void;
    disabled?: boolean;
    isLoading?: boolean;
    className?: string;
  };
  secondaryButton: {
    label: string;
    onClick: () => void;
    disabled?: boolean;
    className?: string;
  };
}

export default function ModalFooter({
  primaryButton,
  secondaryButton,
}: ModalFooterProps) {
  return (
    <div className="flex justify-end gap-3">
      <PrimaryButton
        type="button"
        size="sm"
        onClick={secondaryButton.onClick}
        disabled={secondaryButton.disabled || primaryButton.isLoading}
        className={
          secondaryButton.className ||
          "bg-gray-600 hover:bg-gray-700 dark:bg-gray-700 dark:hover:bg-gray-600"
        }
      >
        {secondaryButton.label}
      </PrimaryButton>
      <PrimaryButton
        type="button"
        size="sm"
        onClick={primaryButton.onClick}
        disabled={primaryButton.disabled || primaryButton.isLoading}
        className={
          primaryButton.className ||
          "bg-indigo-600 hover:bg-indigo-700 dark:bg-indigo-700 dark:hover:bg-indigo-600"
        }
      >
        {primaryButton.isLoading ? "Processing..." : primaryButton.label}
      </PrimaryButton>
    </div>
  );
}
