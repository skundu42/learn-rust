"use client";

import { useState } from "react";
import Sidebar from "@/components/sidebar";
import { Menu } from "lucide-react";

interface UserInfo {
  id: string;
  email: string;
  name: string;
}

interface Props {
  children: React.ReactNode;
  user: UserInfo | null;
  completedIds: number[];
}

export default function LearnLayoutClient({ children, user, completedIds }: Props) {
  const [sidebarOpen, setSidebarOpen] = useState(false);

  return (
    <div className="flex h-screen overflow-hidden bg-background">
      {/* Mobile overlay */}
      {sidebarOpen && (
        <div
          className="fixed inset-0 z-20 bg-black/60 md:hidden"
          onClick={() => setSidebarOpen(false)}
        />
      )}

      {/* Sidebar */}
      <div
        className={`
          fixed inset-y-0 left-0 z-30 md:relative md:z-auto
          transition-transform duration-300 ease-in-out
          ${sidebarOpen ? "translate-x-0" : "-translate-x-full md:translate-x-0"}
        `}
      >
        <Sidebar
          onClose={() => setSidebarOpen(false)}
          user={user}
          completedIds={completedIds}
        />
      </div>

      {/* Main content */}
      <div className="flex-1 flex flex-col min-w-0 overflow-hidden">
        {/* Mobile header */}
        <header className="md:hidden flex items-center gap-3 px-4 py-3 border-b border-[var(--border)] bg-surface shrink-0">
          <button
            onClick={() => setSidebarOpen(true)}
            className="text-muted hover:text-foreground transition-colors"
            aria-label="Open menu"
          >
            <Menu size={20} />
          </button>
          <span className="font-mono font-bold">
            <span className="text-accent">rust</span>learn
          </span>
        </header>

        <main className="flex-1 overflow-hidden">{children}</main>
      </div>
    </div>
  );
}
