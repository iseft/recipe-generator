import { useState } from "react";
import { ChevronDoubleRightIcon } from "@heroicons/react/24/outline";
import Breadcrumbs from "./Breadcrumbs";
import SidebarLayout from "./SidebarLayout";

interface BreadcrumbItem {
  name: string;
  href: string;
  current?: boolean;
}

interface SidebarItem {
  name: string;
  href: string;
  icon: React.ComponentType<{ className?: string }>;
}

interface PageLayoutProps {
  title: string;
  children: React.ReactNode;
  breadcrumbs?: BreadcrumbItem[];
  sidebarNavigation?: SidebarItem[];
}

export default function PageLayout({
  title,
  children,
  breadcrumbs,
  sidebarNavigation,
}: PageLayoutProps) {
  const [sidebarOpen, setSidebarOpen] = useState(false);

  if (sidebarNavigation) {
    return (
      <>
        <SidebarLayout
          navigation={sidebarNavigation}
          sidebarOpen={sidebarOpen}
          setSidebarOpen={setSidebarOpen}
        />
        <header className="fixed top-16 left-0 right-0 z-30 bg-white shadow-sm dark:bg-gray-800 dark:shadow-none dark:after:pointer-events-none dark:after:absolute dark:after:inset-x-0 dark:after:inset-y-0 dark:after:border-y dark:after:border-white/10">
          <div className="mx-auto max-w-7xl px-4 py-6 sm:px-6 lg:px-8">
            <div className="flex items-center gap-4 lg:ml-80">
              <button
                type="button"
                onClick={() => setSidebarOpen(true)}
                className="-m-2.5 p-2.5 text-gray-700 hover:text-gray-900 lg:hidden dark:text-gray-400 dark:hover:text-white"
              >
                <span className="sr-only">Open sidebar</span>
                <ChevronDoubleRightIcon aria-hidden="true" className="size-6" />
              </button>
              <div className="flex-1">
                {breadcrumbs && (
                  <div className="mb-4">
                    <Breadcrumbs items={breadcrumbs} />
                  </div>
                )}
                <h1 className="text-3xl font-bold tracking-tight text-gray-900 dark:text-white">
                  {title}
                </h1>
              </div>
            </div>
          </div>
        </header>
        <main className="pt-32">
          <div className="mx-auto max-w-7xl px-4 py-6 sm:px-6 lg:px-8">
            <div className="lg:ml-80">{children}</div>
          </div>
        </main>
      </>
    );
  }

  return (
    <>
      <header className="fixed top-16 left-0 right-0 z-40 bg-white shadow-sm dark:bg-gray-800 dark:shadow-none dark:after:pointer-events-none dark:after:absolute dark:after:inset-x-0 dark:after:inset-y-0 dark:after:border-y dark:after:border-white/10">
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
      <main className="pt-32">
        <div className="mx-auto max-w-7xl px-4 py-6 sm:px-6 lg:px-8">
          {children}
        </div>
      </main>
    </>
  );
}
