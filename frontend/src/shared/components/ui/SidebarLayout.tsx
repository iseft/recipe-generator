import {
  Dialog,
  DialogBackdrop,
  DialogPanel,
  TransitionChild,
} from "@headlessui/react";
import { XMarkIcon } from "@heroicons/react/24/outline";
import { NavLink } from "react-router-dom";
import { classNames } from "../../utils/classNames";

interface SidebarItem {
  name: string;
  href: string;
  icon: React.ComponentType<{ className?: string }>;
}

interface SidebarLayoutProps {
  navigation: SidebarItem[];
  sidebarOpen: boolean;
  setSidebarOpen: (open: boolean) => void;
}

export default function SidebarLayout({
  navigation,
  sidebarOpen,
  setSidebarOpen,
}: SidebarLayoutProps) {
  return (
    <>
      <Dialog
        open={sidebarOpen}
        onClose={setSidebarOpen}
        className="relative z-50 lg:hidden"
      >
        <DialogBackdrop
          transition
          className="fixed inset-0 bg-gray-900/80 transition-opacity duration-300 ease-linear data-closed:opacity-0"
        />

        <div className="fixed inset-0 flex">
          <DialogPanel
            transition
            className="relative mr-16 flex w-full max-w-xs flex-1 transform transition duration-300 ease-in-out data-closed:-translate-x-full"
          >
            <TransitionChild>
              <div className="absolute top-0 left-full flex w-16 justify-center pt-5 duration-300 ease-in-out data-closed:opacity-0">
                <button
                  type="button"
                  onClick={() => setSidebarOpen(false)}
                  className="-m-2.5 p-2.5"
                >
                  <span className="sr-only">Close sidebar</span>
                  <XMarkIcon aria-hidden="true" className="size-6 text-white" />
                </button>
              </div>
            </TransitionChild>

            <div className="relative flex grow flex-col gap-y-5 overflow-y-auto bg-white px-6 pb-2 dark:bg-gray-900 dark:ring dark:ring-white/10 dark:before:pointer-events-none dark:before:absolute dark:before:inset-0 dark:before:bg-black/10">
              <nav className="relative flex flex-1 flex-col">
                <ul role="list" className="flex flex-1 flex-col gap-y-7">
                  <li>
                    <ul role="list" className="-mx-2 space-y-1">
                      {navigation.map((item) => (
                        <li key={item.name}>
                          <NavLink
                            to={item.href}
                            end={item.href === "/"}
                            className={({ isActive }) =>
                              classNames(
                                isActive
                                  ? "bg-gray-50 text-indigo-600 dark:bg-white/5 dark:text-white"
                                  : "text-gray-700 hover:bg-gray-50 hover:text-indigo-600 dark:text-gray-400 dark:hover:bg-white/5 dark:hover:text-white",
                                "group flex gap-x-3 rounded-md p-2 text-xs font-medium"
                              )
                            }
                            onClick={() => setSidebarOpen(false)}
                          >
                            {({ isActive }) => (
                              <>
                                <item.icon
                                  aria-hidden="true"
                                  className={classNames(
                                    isActive
                                      ? "text-indigo-600 dark:text-white"
                                      : "text-gray-400 group-hover:text-indigo-600 dark:group-hover:text-white",
                                    "size-6 shrink-0"
                                  )}
                                />
                                {item.name}
                              </>
                            )}
                          </NavLink>
                        </li>
                      ))}
                    </ul>
                  </li>
                </ul>
              </nav>
            </div>
          </DialogPanel>
        </div>
      </Dialog>

      <div className="hidden lg:fixed lg:top-16 lg:bottom-0 lg:left-0 lg:right-0 lg:z-40 lg:flex lg:pointer-events-none">
        <div className="mx-auto w-full max-w-7xl px-4 sm:px-6 lg:px-8 pointer-events-none h-full">
          <div className="w-72 h-full flex flex-col gap-y-5 overflow-y-auto bg-indigo-50 dark:bg-indigo-950 pointer-events-auto shadow-sm">
            <nav className="flex flex-1 flex-col px-6 py-6">
              <ul role="list" className="flex flex-1 flex-col gap-y-7">
                <li>
                  <ul role="list" className="space-y-1">
                  {navigation.map((item) => (
                    <li key={item.name}>
                      <NavLink
                        to={item.href}
                        end={item.href === "/"}
                        className={({ isActive }) =>
                          classNames(
                            isActive
                              ? "bg-gray-50 text-indigo-600 dark:bg-white/5 dark:text-white"
                              : "text-gray-700 hover:bg-gray-50 hover:text-indigo-600 dark:text-gray-400 dark:hover:bg-white/5 dark:hover:text-white",
                            "group flex gap-x-3 rounded-md p-2 text-xs font-medium"
                          )
                        }
                      >
                        {({ isActive }) => (
                          <>
                            <item.icon
                              aria-hidden="true"
                              className={classNames(
                                isActive
                                  ? "text-indigo-600 dark:text-white"
                                  : "text-gray-400 group-hover:text-indigo-600 dark:group-hover:text-white",
                                "size-6 shrink-0"
                              )}
                            />
                            {item.name}
                          </>
                        )}
                      </NavLink>
                    </li>
                  ))}
                </ul>
              </li>
            </ul>
          </nav>
          </div>
        </div>
      </div>
    </>
  );
}
