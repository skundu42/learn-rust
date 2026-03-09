"use client";

import Link from "next/link";
import { usePathname } from "next/navigation";
import { useState, useEffect } from "react";
import { CheckCircle2, ChevronDown, ChevronRight, BookOpen, Cpu, BarChart3, Wrench, X } from "lucide-react";
import { LESSONS, TRACKS, getLessonsByTrack, type Track } from "@/lib/lessons";
import { getCompletedLessons } from "@/lib/utils";
import { cn } from "@/lib/utils";

const TRACK_ICONS: Record<Track, React.ReactNode> = {
  fundamentals: <BookOpen size={14} />,
  advanced: <Cpu size={14} />,
  dsa: <BarChart3 size={14} />,
  projects: <Wrench size={14} />,
};

interface SidebarProps {
  open?: boolean;
  onClose?: () => void;
}

export default function Sidebar({ open, onClose }: SidebarProps) {
  const pathname = usePathname();
  const [completed, setCompleted] = useState<number[]>([]);
  const [openTracks, setOpenTracks] = useState<Record<Track, boolean>>({
    fundamentals: true,
    advanced: false,
    dsa: false,
    projects: false,
  });

  useEffect(() => {
    setCompleted(getCompletedLessons());
    const handler = () => setCompleted(getCompletedLessons());
    window.addEventListener("storage", handler);
    window.addEventListener("lesson-completed", handler);
    return () => {
      window.removeEventListener("storage", handler);
      window.removeEventListener("lesson-completed", handler);
    };
  }, []);

  // Auto-expand track for current lesson
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

  const totalDone = completed.length;
  const totalLessons = LESSONS.length;
  const overallPct = Math.round((totalDone / totalLessons) * 100);

  return (
    <aside
      className={cn(
        "flex flex-col h-full bg-surface border-r border-[var(--border)] w-64 shrink-0",
        "transition-transform duration-300",
        open === false ? "-translate-x-full" : "translate-x-0"
      )}
    >
      {/* Header */}
      <div className="flex items-center justify-between px-4 py-4 border-b border-[var(--border)]">
        <Link href="/" className="flex items-center gap-2 group">
          <span className="text-accent font-mono font-bold text-lg tracking-tight">
            rust
          </span>
          <span className="text-foreground font-mono font-bold text-lg tracking-tight">
            learn
          </span>
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
            (lessons.filter((l) => completed.includes(l.id)).length / lessons.length) * 100
          );
          const isExpanded = openTracks[track.id];

          return (
            <div key={track.id} className="mb-0.5">
              {/* Track header */}
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
                <span className="text-xs font-mono text-muted mr-1">
                  {donePct}%
                </span>
                {isExpanded ? (
                  <ChevronDown size={12} className="text-muted shrink-0" />
                ) : (
                  <ChevronRight size={12} className="text-muted shrink-0" />
                )}
              </button>

              {/* Lessons */}
              {isExpanded && (
                <ul>
                  {lessons.map((lesson) => {
                    const isActive = pathname === `/learn/${lesson.slug}`;
                    const isDone = completed.includes(lesson.id);

                    return (
                      <li key={lesson.id}>
                        <Link
                          href={`/learn/${lesson.slug}`}
                          className={cn(
                            "flex items-center gap-2.5 pl-8 pr-4 py-2 text-sm transition-colors group",
                            isActive
                              ? "bg-accent/10 text-foreground border-r-2 border-accent"
                              : "text-muted hover:text-foreground hover:bg-[var(--surface-2)]"
                          )}
                          onClick={onClose}
                        >
                          {isDone ? (
                            <CheckCircle2
                              size={13}
                              className="text-success shrink-0"
                            />
                          ) : (
                            <span
                              className={cn(
                                "w-3.5 h-3.5 rounded-full border shrink-0 flex items-center justify-center",
                                isActive
                                  ? "border-accent"
                                  : "border-[var(--border)]"
                              )}
                            >
                              <span className="text-[8px] font-mono text-muted">
                                {lesson.id}
                              </span>
                            </span>
                          )}
                          <span className="truncate leading-relaxed">
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
      <div className="px-4 py-3 border-t border-[var(--border)]">
        <Link
          href="/"
          className="text-xs text-muted hover:text-foreground transition-colors"
        >
          ← Back to home
        </Link>
      </div>
    </aside>
  );
}
