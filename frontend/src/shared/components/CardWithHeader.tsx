interface CardWithHeaderProps {
  header: React.ReactNode;
  children: React.ReactNode;
  className?: string;
}

export default function CardWithHeader({
  header,
  children,
  className,
}: CardWithHeaderProps) {
  return (
    <div
      className={`divide-y divide-gray-200 overflow-hidden rounded-lg bg-white shadow-sm dark:divide-white/10 dark:bg-gray-800/50 dark:shadow-none dark:outline dark:-outline-offset-1 dark:outline-white/10 ${
        className ?? ""
      }`}
    >
      <div className="px-4 py-5 sm:px-6">{header}</div>
      <div className="px-4 py-5 sm:p-6">{children}</div>
    </div>
  );
}
