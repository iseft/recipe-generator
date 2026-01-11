import { Link } from "react-router-dom";
import { ChevronRightIcon, HomeIcon } from "@heroicons/react/20/solid";

interface BreadcrumbItem {
  name: string;
  href: string;
  current?: boolean;
}

interface BreadcrumbsProps {
  items: BreadcrumbItem[];
}

export default function Breadcrumbs({ items }: BreadcrumbsProps) {
  return (
    <nav aria-label="Breadcrumb" className="flex">
      <ol role="list" className="flex items-center space-x-4">
        <li>
          <div>
            <Link
              to="/"
              className="text-gray-400 hover:text-gray-500 dark:text-gray-500 dark:hover:text-gray-300"
            >
              <HomeIcon aria-hidden="true" className="size-5 shrink-0" />
              <span className="sr-only">Home</span>
            </Link>
          </div>
        </li>
        {items.map((item) => (
          <li key={item.name}>
            <div className="flex items-center">
              <ChevronRightIcon
                aria-hidden="true"
                className="size-5 shrink-0 text-gray-400 dark:text-gray-500"
              />
              <Link
                to={item.href}
                aria-current={item.current ? "page" : undefined}
                className="ml-4 text-sm font-medium text-gray-500 hover:text-gray-700 dark:text-gray-400 dark:hover:text-gray-200"
              >
                {item.name}
              </Link>
            </div>
          </li>
        ))}
      </ol>
    </nav>
  );
}
