"use client";

import { useState, useEffect } from "react";
import Link from "next/link";
import dynamic from "next/dynamic";
import {
  CheckCircle2,
  Circle,
  ChevronLeft,
  ChevronRight,
  Lightbulb,
  Tag,
  BookOpen,
} from "lucide-react";
import { type Lesson, TRACKS, getNextLesson, getPrevLesson } from "@/lib/lessons";
import { markLessonComplete, markLessonIncomplete, getCompletedLessons } from "@/lib/utils";
import { cn } from "@/lib/utils";

const RustIDE = dynamic(() => import("@/components/rust-ide"), {
  ssr: false,
  loading: () => (
    <div className="flex-1 flex items-center justify-center bg-[#0d1117]">
      <div className="flex flex-col items-center gap-3">
        <div className="w-6 h-6 rounded-full border-2 border-accent border-t-transparent animate-spin" />
        <span className="text-sm text-muted font-mono">Loading IDE...</span>
      </div>
    </div>
  ),
});

const DIFFICULTY_COLORS: Record<string, string> = {
  beginner: "text-success bg-success/10",
  intermediate: "text-warning bg-warning/10",
  advanced: "text-danger bg-danger/10",
};

const TRACK_COLORS: Record<string, string> = {
  fundamentals: "#e75a2b",
  advanced: "#3b82f6",
  dsa: "#a855f7",
  projects: "#22c55e",
};

interface LessonViewProps {
  lesson: Lesson;
}

export default function LessonView({ lesson }: LessonViewProps) {
  const [completed, setCompleted] = useState(false);
  const [showHint, setShowHint] = useState(false);
  const [panelView, setPanelView] = useState<"info" | "ide">("info");

  const nextLesson = getNextLesson(lesson.id);
  const prevLesson = getPrevLesson(lesson.id);
  const track = TRACKS.find((t) => t.id === lesson.track);

  useEffect(() => {
    const ids = getCompletedLessons();
    setCompleted(ids.includes(lesson.id));
  }, [lesson.id]);

  const toggleComplete = () => {
    if (completed) {
      markLessonIncomplete(lesson.id);
      setCompleted(false);
    } else {
      markLessonComplete(lesson.id);
      setCompleted(true);
    }
    window.dispatchEvent(new Event("lesson-completed"));
  };

  const trackColor = TRACK_COLORS[lesson.track];

  return (
    <div className="h-full flex flex-col">
      {/* Mobile tab toggle */}
      <div className="flex md:hidden border-b border-[var(--border)] bg-surface shrink-0">
        <button
          onClick={() => setPanelView("info")}
          className={cn(
            "flex-1 py-2 text-xs font-medium transition-colors",
            panelView === "info"
              ? "text-foreground border-b-2 border-accent"
              : "text-muted"
          )}
        >
          Lesson
        </button>
        <button
          onClick={() => setPanelView("ide")}
          className={cn(
            "flex-1 py-2 text-xs font-medium transition-colors",
            panelView === "ide"
              ? "text-foreground border-b-2 border-accent"
              : "text-muted"
          )}
        >
          Editor
        </button>
      </div>

      {/* Content — split pane on desktop, tabs on mobile */}
      <div className="flex-1 flex min-h-0 overflow-hidden">
        {/* Left: Lesson info */}
        <div
          className={cn(
            "w-full md:w-[42%] md:border-r md:border-[var(--border)] flex flex-col overflow-y-auto",
            panelView === "ide" ? "hidden md:flex" : "flex"
          )}
        >
          {/* Lesson header */}
          <div className="px-6 pt-6 pb-4 border-b border-[var(--border)] shrink-0">
            <div className="flex items-center gap-2 mb-3 flex-wrap">
              {track && (
                <span
                  className="text-xs font-semibold px-2 py-0.5 rounded-full"
                  style={{
                    color: trackColor,
                    background: `${trackColor}18`,
                  }}
                >
                  {track.label}
                </span>
              )}
              <span
                className={cn(
                  "text-xs px-2 py-0.5 rounded-full font-medium",
                  DIFFICULTY_COLORS[lesson.difficulty]
                )}
              >
                {lesson.difficulty}
              </span>
              <span className="text-xs text-muted ml-auto font-mono">
                Step {lesson.id} of 36
              </span>
            </div>

            <h1 className="text-xl font-bold text-foreground text-balance leading-tight mb-2">
              {lesson.title}
            </h1>
            <p className="text-sm text-muted leading-relaxed">
              {lesson.description}
            </p>
          </div>

          {/* Concepts */}
          <div className="px-6 py-4 border-b border-[var(--border)] shrink-0">
            <div className="flex items-center gap-2 mb-3">
              <Tag size={13} className="text-muted" />
              <span className="text-xs font-semibold text-muted uppercase tracking-wide">
                Key Concepts
              </span>
            </div>
            <div className="flex flex-wrap gap-1.5">
              {lesson.concepts.map((concept) => (
                <span
                  key={concept}
                  className="text-xs px-2 py-1 rounded bg-[var(--surface-2)] text-muted border border-[var(--border)] font-mono"
                >
                  {concept}
                </span>
              ))}
            </div>
          </div>

          {/* Hint */}
          <div className="px-6 py-4 border-b border-[var(--border)] shrink-0">
            <button
              onClick={() => setShowHint(!showHint)}
              className="flex items-center gap-2 text-xs font-semibold text-warning/80 hover:text-warning transition-colors"
            >
              <Lightbulb size={13} />
              {showHint ? "Hide hint" : "Show hint"}
            </button>
            {showHint && (
              <div className="mt-3 p-3 rounded bg-warning/5 border border-warning/20">
                <p className="text-xs text-[var(--foreground)] leading-relaxed font-mono">
                  {lesson.solutionHint}
                </p>
              </div>
            )}
          </div>

          {/* Source code link */}
          <div className="px-6 py-4 border-b border-[var(--border)] shrink-0">
            <div className="flex items-center gap-2">
              <BookOpen size={13} className="text-muted" />
              <span className="text-xs text-muted">
                Full source:{" "}
                <a
                  href={`https://github.com/skundu42/learn-rust/tree/main/step_${String(lesson.id).padStart(2, "0")}_${lesson.slug.replace(/-/g, "_")}`}
                  target="_blank"
                  rel="noopener noreferrer"
                  className="text-accent hover:underline font-mono"
                >
                  step_{String(lesson.id).padStart(2, "0")}/main.rs
                </a>
              </span>
            </div>
          </div>

          {/* Spacer */}
          <div className="flex-1" />

          {/* Navigation & complete */}
          <div className="px-6 py-4 border-t border-[var(--border)] shrink-0 space-y-3">
            <button
              onClick={toggleComplete}
              className={cn(
                "w-full flex items-center justify-center gap-2 py-2.5 px-4 rounded font-medium text-sm transition-colors",
                completed
                  ? "bg-success/10 text-success border border-success/20 hover:bg-success/20"
                  : "bg-accent text-white hover:bg-accent/90"
              )}
            >
              {completed ? (
                <>
                  <CheckCircle2 size={15} />
                  Completed
                </>
              ) : (
                <>
                  <Circle size={15} />
                  Mark as Complete
                </>
              )}
            </button>

            <div className="flex gap-2">
              {prevLesson ? (
                <Link
                  href={`/learn/${prevLesson.slug}`}
                  className="flex-1 flex items-center justify-center gap-1.5 py-2 px-3 rounded border border-[var(--border)] text-xs text-muted hover:text-foreground hover:border-[var(--muted)] transition-colors"
                >
                  <ChevronLeft size={13} />
                  Previous
                </Link>
              ) : (
                <div className="flex-1" />
              )}
              {nextLesson ? (
                <Link
                  href={`/learn/${nextLesson.slug}`}
                  className="flex-1 flex items-center justify-center gap-1.5 py-2 px-3 rounded border border-[var(--border)] text-xs text-muted hover:text-foreground hover:border-[var(--muted)] transition-colors"
                >
                  Next
                  <ChevronRight size={13} />
                </Link>
              ) : (
                <Link
                  href="/"
                  className="flex-1 flex items-center justify-center gap-1.5 py-2 px-3 rounded bg-success/10 border border-success/20 text-xs text-success hover:bg-success/20 transition-colors"
                >
                  Finish!
                </Link>
              )}
            </div>
          </div>
        </div>

        {/* Right: IDE */}
        <div
          className={cn(
            "flex-1 flex flex-col min-h-0",
            panelView === "info" ? "hidden md:flex" : "flex"
          )}
        >
          <RustIDE initialCode={lesson.starterCode} lessonId={lesson.id} />
        </div>
      </div>
    </div>
  );
}
