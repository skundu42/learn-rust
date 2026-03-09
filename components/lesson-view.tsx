"use client";

import { useEffect, useState, useTransition } from "react";
import Link from "next/link";
import { useRouter } from "next/navigation";
import dynamic from "next/dynamic";
import {
  CheckCircle2,
  Circle,
  ChevronLeft,
  ChevronRight,
  Lightbulb,
  Tag,
  BookOpen,
  Lock,
  ArrowRight,
} from "lucide-react";
import { type Lesson, TRACKS, getNextLesson, getPrevLesson } from "@/lib/lessons";
import {
  markLessonIncompleteServer,
  verifyLessonSolutionServer,
} from "@/app/auth/actions";
import type { LessonVerificationResult } from "@/lib/lesson-verification-types";
import { cn } from "@/lib/utils";
import ProgressTracker from "@/components/progress-tracker";

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

interface UserInfo {
  id: string;
  email: string;
  name: string;
}

interface LessonViewProps {
  lesson: Lesson;
  user: UserInfo | null;
  initialCompleted: number[];
}

// Lesson 1 (id === 1) is free for everyone
const FREE_LESSON_ID = 1;

export default function LessonView({ lesson, user, initialCompleted }: LessonViewProps) {
  const router = useRouter();
  const [isPending, startTransition] = useTransition();
  const [completed, setCompleted] = useState(initialCompleted.includes(lesson.id));
  const [code, setCode] = useState(lesson.starterCode);
  const [showHint, setShowHint] = useState(false);
  const [panelView, setPanelView] = useState<"info" | "ide">("info");
  const [verification, setVerification] = useState<LessonVerificationResult | null>(null);

  const nextLesson = getNextLesson(lesson.id);
  const prevLesson = getPrevLesson(lesson.id);
  const track = TRACKS.find((t) => t.id === lesson.track);
  const isLocked = !user && lesson.id !== FREE_LESSON_ID;
  const trackColor = TRACK_COLORS[lesson.track];

  useEffect(() => {
    setCompleted(initialCompleted.includes(lesson.id));
    setCode(lesson.starterCode);
    setShowHint(false);
    setPanelView("info");
    setVerification(null);
  }, [initialCompleted, lesson.id, lesson.starterCode]);

  const updateCode = (nextCode: string) => {
    setCode(nextCode);
    setVerification(null);
  };

  const verifyAndComplete = () => {
    startTransition(async () => {
      const result = await verifyLessonSolutionServer(lesson.id, code);
      setVerification(result);

      if (result.progressSaved) {
        setCompleted(true);
        router.refresh();
      }
    });
  };

  const markIncomplete = () => {
    if (!user) return;

    startTransition(async () => {
      const result = await markLessonIncompleteServer(lesson.id);

      if (result.error) {
        setVerification({
          passed: false,
          method: "hidden-tests",
          summary: "Could not update lesson progress.",
          details: result.error,
          progressSaved: false,
        });
        return;
      }

      setCompleted(false);
      setVerification(null);
      router.refresh();
    });
  };

  // Auth gate — shown when a non-signed-in user tries to open a locked lesson
  if (isLocked) {
    return (
      <div className="h-full flex items-center justify-center bg-background px-4">
        <div className="w-full max-w-md text-center">
          <div className="w-14 h-14 rounded-full bg-accent/10 border border-accent/20 flex items-center justify-center mx-auto mb-5">
            <Lock size={24} className="text-accent" />
          </div>

          <h2 className="text-xl font-bold text-foreground mb-2 text-balance">
            Sign in to unlock this lesson
          </h2>
          <p className="text-sm text-muted leading-relaxed mb-6 max-w-sm mx-auto">
            <span className="text-foreground font-medium">{lesson.title}</span> is
            part of the full curriculum. Create a free account to access all 36
            lessons and track your progress.
          </p>

          <div className="flex flex-col sm:flex-row gap-3 justify-center">
            <Link
              href={`/auth/sign-up?next=/learn/${lesson.slug}`}
              className="flex items-center justify-center gap-2 bg-accent text-white px-5 py-2.5 rounded font-medium text-sm hover:bg-accent/90 transition-colors"
            >
              Create free account
              <ArrowRight size={14} />
            </Link>
            <Link
              href={`/auth/login?next=/learn/${lesson.slug}`}
              className="flex items-center justify-center gap-2 border border-[var(--border)] text-muted px-5 py-2.5 rounded font-medium text-sm hover:text-foreground hover:border-[var(--muted)] transition-colors"
            >
              Sign in
            </Link>
          </div>

          <p className="text-xs text-muted mt-6">
            <Link href="/learn/hello-world" className="hover:text-foreground transition-colors">
              ← Back to lesson 1 (free)
            </Link>
          </p>
        </div>
      </div>
    );
  }

  return (
    <div className="h-full flex flex-col">
      <ProgressTracker enabled={Boolean(user)} lessonId={lesson.id} />

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
                  style={{ color: trackColor, background: `${trackColor}18` }}
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
            <p className="text-sm text-muted leading-relaxed">{lesson.description}</p>
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
                <p className="text-xs text-foreground leading-relaxed font-mono">
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

          <div className="flex-1" />

          {/* Navigation & complete */}
          <div className="px-6 py-4 border-t border-[var(--border)] shrink-0 space-y-3">
            {user && completed ? (
              <button
                onClick={markIncomplete}
                disabled={isPending}
                className={cn(
                  "w-full flex items-center justify-center gap-2 py-2.5 px-4 rounded font-medium text-sm transition-colors",
                  "disabled:opacity-60 disabled:cursor-not-allowed",
                  "bg-success/10 text-success border border-success/20 hover:bg-success/20"
                )}
              >
                <CheckCircle2 size={15} />
                {isPending ? "Updating..." : "Mark as Incomplete"}
              </button>
            ) : (
              <button
                onClick={verifyAndComplete}
                disabled={isPending}
                className="w-full flex items-center justify-center gap-2 py-2.5 px-4 rounded font-medium text-sm bg-accent text-white hover:bg-accent/90 transition-colors disabled:opacity-60 disabled:cursor-not-allowed"
              >
                <Circle size={15} />
                {isPending
                  ? user
                    ? "Verifying..."
                    : "Checking..."
                  : user
                  ? "Verify & Mark Complete"
                  : "Verify Solution"}
              </button>
            )}

            {!user && (
              <Link
                href={`/auth/login?next=/learn/${lesson.slug}`}
                className="w-full flex items-center justify-center gap-2 py-2.5 px-4 rounded font-medium text-sm border border-[var(--border)] text-muted hover:text-foreground hover:border-[var(--muted)] transition-colors"
              >
                Sign in to save progress
              </Link>
            )}

            {verification && (
              <div
                className={cn(
                  "rounded border p-3",
                  verification.passed
                    ? "bg-success/5 border-success/20"
                    : "bg-danger/5 border-danger/20"
                )}
              >
                <div className="flex items-center gap-2 mb-1.5">
                  <span
                    className={cn(
                      "text-[10px] uppercase tracking-wide font-semibold",
                      verification.passed ? "text-success" : "text-danger"
                    )}
                  >
                    {verification.method === "hidden-tests" ? "Hidden tests" : "Output check"}
                  </span>
                  <span
                    className={cn(
                      "text-[10px] uppercase tracking-wide font-semibold",
                      verification.passed ? "text-success" : "text-danger"
                    )}
                  >
                    {verification.passed ? "Passed" : "Failed"}
                  </span>
                </div>
                <p className="text-xs text-foreground leading-relaxed">
                  {verification.summary}
                </p>
                {verification.details && (
                  <pre className="mt-2 max-h-40 overflow-auto whitespace-pre-wrap text-[11px] leading-relaxed text-muted font-mono">
                    {verification.details}
                  </pre>
                )}
              </div>
            )}

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
          <RustIDE
            code={code}
            initialCode={lesson.starterCode}
            onCodeChange={updateCode}
          />
        </div>
      </div>
    </div>
  );
}
