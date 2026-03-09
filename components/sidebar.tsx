"use client";

import Link from "next/link";
import { usePathname } from "next/navigation";
import { useState, useEffect, useTransition } from "react";
import {
  CheckCircle2,
  ChevronDown,
  ChevronRight,
  BookOpen,
  Cpu,
  BarChart3,
  Wrench,
  X,
  LogOut,
  LogIn,
  Lock,
  User,
} from "lucide-react";
import { LESSONS, TRACKS, getLessonsByTrack, type Track } from "@/lib/lessons";
import { signOut } from "@/app/auth/actions";
import { cn } from "@/lib/utils";

const TRACK_ICONS: Record<Track, React.ReactNode> = {
  fundamentals: <BookOpen size={14} />,
  advanced: <Cpu size={14} />,
  dsa: <BarChart3 size={14} />,
  projects: <Wrench size={14} />,
};

interface UserInfo {
  id: string;
  email: string;
  name: string;
}

interface SidebarProps {
  onClose?: () => void;
  user: UserInfo | null;
  completedIds: number[];
}

const FREE_LESSON_ID = 1;

export default function Sidebar({ onClose, user, completedIds }: SidebarProps) {
  const pathname = usePathname();
  const [isPending, startTransition] = useTransition();
  const [openTracks, setOpenTracks] = useState<Record<Track, boolean>>({
    fundamentals: true,
    advanced: false,
    dsa: false,
    projects: false,
  });

  // Auto-expand track for the active lesson
  useEffect(() => {
    const slug = pathname.split("/learn/")[1];
    if (!slug) return;
    const lesson = LESSONS.find((l) => l.slug === slug);
    if (lesson) {
      setOpenTracks((prev) => ({ ...prev, [lesson.track]: true }));
    }
  }, [pathname]);

  const toggleTrack = (track: Track) => {
    setOpenTracks((prev) => ({ ...prev, [track]: !prev[track] }));
  };

  const handleSignOut = () => {
    startTransition(async () => {
      await signOut();
    });
  };

  const totalDone = completedIds.length;
  const totalLessons = LESSONS.length;
  const overallPct = Math.round((totalDone / totalLessons) * 100);

  return (
    <aside className="flex flex-col h-full bg-surface border-r border-[var(--border)] w-64 shrink-0">
      {/* Header */}
      <div className="flex items-center justify-between px-4 py-4 border-b border-[var(--border)]">
        <Link href="/" className="flex items-center gap-2 group">
          <span className="text-accent font-mono font-bold text-lg tracking-tight">rust</span>
          <span className="text-foreground font-mono font-bold text-lg tracking-tight">learn</span>
        </Link>
        {onClose && (
          <button
            onClick={onClose}
            className="text-muted hover:text-foreground transition-colors md:hidden"
            aria-label="Close sidebar"
          >
            <X size={18} />
          </button>
        )}
      </div>

      {/* User info / guest banner */}
      {user ? (
        <div className="px-4 py-3 border-b border-[var(--border)]">
          <div className="flex items-center gap-2">
            <div className="w-7 h-7 rounded-full bg-accent/15 flex items-center justify-center shrink-0">
              <User size={13} className="text-accent" />
            </div>
            <div className="flex-1 min-w-0">
              <p className="text-xs font-medium text-foreground truncate">
                {user.name || user.email}
              </p>
              {user.name && (
                <p className="text-[10px] text-muted truncate">{user.email}</p>
              )}
            </div>
          </div>
        </div>
      ) : (
        <div className="px-4 py-3 border-b border-[var(--border)]">
          <div className="flex items-center gap-2 p-2 rounded bg-accent/5 border border-accent/15">
            <Lock size={12} className="text-accent shrink-0" />
            <p className="text-[11px] text-muted leading-tight flex-1">
              Sign in to unlock all lessons
            </p>
            <Link
              href="/auth/login"
              className="text-[11px] text-accent font-medium hover:underline shrink-0"
              onClick={onClose}
            >
              Sign in
            </Link>
          </div>
        </div>
      )}

      {/* Overall progress */}
      <div className="px-4 py-3 border-b border-[var(--border)]">
        <div className="flex justify-between items-center mb-1.5">
          <span className="text-xs text-muted">Overall progress</span>
          <span className="text-xs text-foreground font-mono">
            {totalDone}/{totalLessons}
          </span>
        </div>
        <div className="h-1.5 rounded-full bg-[var(--surface-2)] overflow-hidden">
          <div
            className="h-full rounded-full bg-accent transition-all duration-500"
            style={{ width: `${overallPct}%` }}
          />
        </div>
      </div>

      {/* Track list */}
      <nav className="flex-1 overflow-y-auto py-2" aria-label="Lesson navigation">
        {TRACKS.map((track) => {
          const lessons = getLessonsByTrack(track.id);
          const donePct = Math.round(
            (lessons.filter((l) => completedIds.includes(l.id)).length / lessons.length) * 100
          );
          const isExpanded = openTracks[track.id];

          return (
            <div key={track.id} className="mb-0.5">
              <button
                onClick={() => toggleTrack(track.id)}
                className="w-full flex items-center gap-2 px-4 py-2.5 hover:bg-[var(--surface-2)] transition-colors text-left"
              >
                <span style={{ color: track.color }} className="shrink-0">
                  {TRACK_ICONS[track.id]}
                </span>
                <span className="flex-1 text-xs font-semibold text-foreground tracking-wide uppercase">
                  {track.label}
                </span>
                <span className="text-xs font-mono text-muted mr-1">{donePct}%</span>
                {isExpanded ? (
                  <ChevronDown size={12} className="text-muted shrink-0" />
                ) : (
                  <ChevronRight size={12} className="text-muted shrink-0" />
                )}
              </button>

              {isExpanded && (
                <ul>
                  {lessons.map((lesson) => {
                    const isActive = pathname === `/learn/${lesson.slug}`;
                    const isDone = completedIds.includes(lesson.id);
                    const isLocked = !user && lesson.id !== FREE_LESSON_ID;

                    return (
                      <li key={lesson.id}>
                        <Link
                          href={
                            isLocked
                              ? `/auth/login?next=/learn/${lesson.slug}`
                              : `/learn/${lesson.slug}`
                          }
                          className={cn(
                            "flex items-center gap-2.5 pl-8 pr-4 py-2 text-sm transition-colors group",
                            isActive
                              ? "bg-accent/10 text-foreground border-r-2 border-accent"
                              : isLocked
                              ? "text-muted/40 hover:bg-[var(--surface-2)]"
                              : "text-muted hover:text-foreground hover:bg-[var(--surface-2)]"
                          )}
                          onClick={onClose}
                        >
                          {isLocked ? (
                            <Lock size={11} className="text-muted/40 shrink-0" />
                          ) : isDone ? (
                            <CheckCircle2 size={13} className="text-success shrink-0" />
                          ) : (
                            <span
                              className={cn(
                                "w-3.5 h-3.5 rounded-full border shrink-0 flex items-center justify-center",
                                isActive ? "border-accent" : "border-[var(--border)]"
                              )}
                            >
                              <span className="text-[8px] font-mono text-muted">{lesson.id}</span>
                            </span>
                          )}
                          <span className={cn("truncate leading-relaxed", isLocked && "opacity-40")}>
                            {lesson.title}
                          </span>
                        </Link>
                      </li>
                    );
                  })}
                </ul>
              )}
            </div>
          );
        })}
      </nav>

      {/* Footer */}
      <div className="px-4 py-3 border-t border-[var(--border)] flex items-center justify-between">
        <Link
          href="/"
          className="text-xs text-muted hover:text-foreground transition-colors"
          onClick={onClose}
        >
          Home
        </Link>
        {user ? (
          <button
            onClick={handleSignOut}
            disabled={isPending}
            className="flex items-center gap-1.5 text-xs text-muted hover:text-danger transition-colors disabled:opacity-50"
          >
            <LogOut size={12} />
            {isPending ? "Signing out..." : "Sign out"}
          </button>
        ) : (
          <Link
            href="/auth/login"
            className="flex items-center gap-1.5 text-xs text-accent hover:underline"
            onClick={onClose}
          >
            <LogIn size={12} />
            Sign in
          </Link>
        )}
      </div>
    </aside>
  );
}
