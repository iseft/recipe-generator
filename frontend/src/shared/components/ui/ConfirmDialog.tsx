import { ExclamationTriangleIcon } from "@heroicons/react/24/outline";
import Modal from "./Modal";
import ModalFooter from "./ModalFooter";

interface ConfirmDialogProps {
  open: boolean;
  onClose: () => void;
  onConfirm: () => void;
  title: string;
  message: string;
  confirmText?: string;
  cancelText?: string;
  confirmButtonClassName?: string;
  isLoading?: boolean;
}

export default function ConfirmDialog({
  open,
  onClose,
  onConfirm,
  title,
  message,
  confirmText = "Confirm",
  cancelText = "Cancel",
  confirmButtonClassName = "bg-red-600 hover:bg-red-700 dark:bg-red-700 dark:hover:bg-red-600",
  isLoading = false,
}: ConfirmDialogProps) {
  const handleConfirm = () => {
    onConfirm();
    onClose();
  };

  return (
    <Modal
      open={open}
      onClose={onClose}
      title={title}
      footer={
        <ModalFooter
          primaryButton={{
            label: confirmText,
            onClick: handleConfirm,
            isLoading,
            className: confirmButtonClassName,
          }}
          secondaryButton={{
            label: cancelText,
            onClick: onClose,
          }}
        />
      }
    >
      <div className="flex items-start">
        <div className="flex-shrink-0">
          <ExclamationTriangleIcon
            className="h-6 w-6 text-red-600 dark:text-red-400"
            aria-hidden="true"
          />
        </div>
        <div className="ml-4 flex-1">
          <p className="text-sm text-gray-500 dark:text-gray-400">{message}</p>
        </div>
      </div>
    </Modal>
  );
}
