import Breadcrumbs from "./ui/Breadcrumbs";

interface BreadcrumbItem {
  name: string;
  href: string;
  current?: boolean;
}

interface PageLayoutProps {
  title: string;
  children: React.ReactNode;
  breadcrumbs?: BreadcrumbItem[];
}

export default function PageLayout({
  title,
  children,
  breadcrumbs,
}: PageLayoutProps) {
  return (
    <>
      <header className="relative bg-white shadow-sm dark:bg-gray-800 dark:shadow-none dark:after:pointer-events-none dark:after:absolute dark:after:inset-x-0 dark:after:inset-y-0 dark:after:border-y dark:after:border-white/10">
        <div className="mx-auto max-w-7xl px-4 py-6 sm:px-6 lg:px-8">
          {breadcrumbs && (
            <div className="mb-4">
              <Breadcrumbs items={breadcrumbs} />
            </div>
          )}
          <h1 className="text-3xl font-bold tracking-tight text-gray-900 dark:text-white">
            {title}
          </h1>
        </div>
      </header>
      <main>
        <div className="mx-auto max-w-7xl px-4 py-6 sm:px-6 lg:px-8">
          {children}
        </div>
      </main>
    </>
  );
}
