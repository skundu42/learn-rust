"use client";

import { useState, useTransition } from "react";
import Link from "next/link";
import {
  BookOpen,
  Cpu,
  BarChart3,
  Wrench,
  ChevronRight,
  CheckCircle2,
  ArrowRight,
  Github,
  Play,
  LogOut,
  User,
} from "lucide-react";
import { type Lesson, type TrackMeta, getLessonsByTrack } from "@/lib/lessons";
import { signOut } from "@/app/auth/actions";
import { cn } from "@/lib/utils";

const TRACK_ICONS: Record<string, React.ReactNode> = {
  fundamentals: <BookOpen size={18} />,
  advanced: <Cpu size={18} />,
  dsa: <BarChart3 size={18} />,
  projects: <Wrench size={18} />,
};

const HERO_CODE = `fn main() {
    let message = greet("Rustacean");
    println!("{}", message);
}

fn greet(name: &str) -> String {
    format!("Hello, {}! 🦀", name)
}`;

interface UserInfo {
  id: string;
  email: string;
  name: string;
}

interface Props {
  lessons: Lesson[];
  tracks: TrackMeta[];
  user: UserInfo | null;
  serverCompletedIds: number[];
}

export default function HomepageClient({ lessons, tracks, user, serverCompletedIds }: Props) {
  const [isPending, startTransition] = useTransition();

  const completed = serverCompletedIds;
  const totalDone = completed.length;
  const totalLessons = lessons.length;
  const overallPct = totalLessons > 0 ? Math.round((totalDone / totalLessons) * 100) : 0;

  // Find current lesson (first incomplete)
  const currentLesson = lessons.find((l) => !completed.includes(l.id)) ?? lessons[0];

  const handleSignOut = () => {
    startTransition(async () => {
      await signOut();
    });
  };

  return (
    <div className="min-h-screen bg-background text-foreground">
      {/* Nav */}
      <header className="sticky top-0 z-10 border-b border-[var(--border)] bg-background/80 backdrop-blur-sm">
        <div className="max-w-6xl mx-auto px-4 sm:px-6 flex items-center justify-between h-14">
          <div className="flex items-center gap-2">
            <span className="font-mono font-bold text-lg text-accent">rust</span>
            <span className="font-mono font-bold text-lg text-foreground">learn</span>
          </div>
          <nav className="flex items-center gap-3">
            <a
              href="https://github.com/skundu42/learn-rust"
              target="_blank"
              rel="noopener noreferrer"
              className="flex items-center gap-1.5 text-xs text-muted hover:text-foreground transition-colors"
            >
              <Github size={14} />
              <span className="hidden sm:inline">Source</span>
            </a>
            {user ? (
              <div className="flex items-center gap-2">
                <div className="hidden sm:flex items-center gap-1.5 text-xs text-muted">
                  <User size={12} />
                  <span className="max-w-[120px] truncate">{user.name || user.email}</span>
                </div>
                <button
                  onClick={handleSignOut}
                  disabled={isPending}
                  className="flex items-center gap-1.5 text-xs text-muted hover:text-danger transition-colors disabled:opacity-50"
                  title="Sign out"
                >
                  <LogOut size={13} />
                  <span className="hidden sm:inline">{isPending ? "..." : "Sign out"}</span>
                </button>
              </div>
            ) : (
              <div className="flex items-center gap-2">
                <Link
                  href="/auth/login"
                  className="text-xs text-muted hover:text-foreground transition-colors"
                >
                  Sign in
                </Link>
                <Link
                  href="/auth/sign-up"
                  className="flex items-center gap-1.5 bg-accent text-white text-xs font-medium px-3 py-1.5 rounded hover:bg-accent/90 transition-colors"
                >
                  Sign up free
                </Link>
              </div>
            )}
            <Link
              href={`/learn/${currentLesson.slug}`}
              className="flex items-center gap-1.5 bg-accent/10 border border-accent/20 text-accent text-xs font-medium px-3 py-1.5 rounded hover:bg-accent/20 transition-colors"
            >
              <Play size={12} />
              {totalDone > 0 ? "Continue" : "Start Learning"}
            </Link>
          </nav>
        </div>
      </header>

      <main>
        {/* Hero */}
        <section className="max-w-6xl mx-auto px-4 sm:px-6 pt-16 pb-12 flex flex-col lg:flex-row gap-12 items-center">
          <div className="flex-1 min-w-0">
            <div className="inline-flex items-center gap-2 px-2.5 py-1 rounded-full bg-accent/10 border border-accent/20 text-accent text-xs font-medium mb-6">
              36 Lessons · Integrated IDE · Free
            </div>
            <h1 className="text-4xl sm:text-5xl font-bold text-foreground leading-tight mb-4 text-balance">
              Learn Rust from{" "}
              <span className="text-accent">Basics to Advanced</span>
            </h1>
            <p className="text-base text-muted leading-relaxed mb-8 max-w-lg">
              A hands-on, step-by-step Rust learning platform with an integrated
              online IDE. Write and run Rust directly in your browser — no
              installation required.
            </p>

            {/* Progress if signed in and started */}
            {user && totalDone > 0 && (
              <div className="mb-6 p-4 rounded-lg bg-surface border border-[var(--border)]">
                <div className="flex items-center justify-between mb-2">
                  <span className="text-sm font-medium">Your progress</span>
                  <span className="text-sm font-mono text-accent">
                    {totalDone}/{totalLessons} lessons
                  </span>
                </div>
                <div className="h-2 rounded-full bg-[var(--surface-2)] overflow-hidden">
                  <div
                    className="h-full rounded-full bg-accent transition-all duration-700"
                    style={{ width: `${overallPct}%` }}
                  />
                </div>
                <p className="text-xs text-muted mt-2">
                  {overallPct}% complete · Next:{" "}
                  <Link
                    href={`/learn/${currentLesson.slug}`}
                    className="text-accent hover:underline"
                  >
                    {currentLesson.title}
                  </Link>
                </p>
              </div>
            )}

            <div className="flex flex-wrap gap-3">
              <Link
                href={`/learn/${currentLesson.slug}`}
                className="flex items-center gap-2 bg-accent text-white px-5 py-2.5 rounded font-medium text-sm hover:bg-accent/90 transition-colors"
              >
                {totalDone > 0 ? "Continue Learning" : "Start Free"}
                <ArrowRight size={15} />
              </Link>
              <a
                href="https://github.com/skundu42/learn-rust"
                target="_blank"
                rel="noopener noreferrer"
                className="flex items-center gap-2 border border-[var(--border)] text-muted px-5 py-2.5 rounded font-medium text-sm hover:text-foreground hover:border-[var(--muted)] transition-colors"
              >
                <Github size={15} />
                View on GitHub
              </a>
            </div>
          </div>

          {/* Code preview */}
          <div className="w-full lg:w-96 shrink-0">
            <div className="rounded-lg overflow-hidden border border-[#21262d] bg-[#0d1117]">
              <div className="flex items-center gap-1.5 px-4 py-3 bg-[#161b22] border-b border-[#21262d]">
                <span className="w-3 h-3 rounded-full bg-[#ff5f57]" />
                <span className="w-3 h-3 rounded-full bg-[#febc2e]" />
                <span className="w-3 h-3 rounded-full bg-[#28c840]" />
                <span className="ml-3 text-xs text-[#484f58] font-mono">main.rs</span>
              </div>
              <pre className="p-4 text-sm font-mono leading-relaxed overflow-x-auto">
                <code>
                  {HERO_CODE.split("\n").map((line, i) => (
                    <div key={i} className="flex">
                      <span className="text-[#484f58] select-none w-6 shrink-0 text-right mr-4">
                        {i + 1}
                      </span>
                      <span
                        dangerouslySetInnerHTML={{
                          __html: highlightRust(line),
                        }}
                      />
                    </div>
                  ))}
                </code>
              </pre>
              <div className="px-4 py-3 bg-[#161b22] border-t border-[#21262d] font-mono text-xs text-[#3fb950]">
                $ Hello, Rustacean! 🦀
              </div>
            </div>
          </div>
        </section>

        {/* Stats */}
        <section className="border-y border-[var(--border)] bg-surface">
          <div className="max-w-6xl mx-auto px-4 sm:px-6 py-8 grid grid-cols-2 sm:grid-cols-4 gap-6">
            {[
              { value: "36", label: "Lessons" },
              { value: "4", label: "Learning tracks" },
              { value: "0", label: "Setup required" },
              { value: "∞", label: "Practice sessions" },
            ].map(({ value, label }) => (
              <div key={label} className="text-center">
                <div className="text-2xl font-bold text-accent font-mono">{value}</div>
                <div className="text-xs text-muted mt-1">{label}</div>
              </div>
            ))}
          </div>
        </section>

        {/* Tracks */}
        <section className="max-w-6xl mx-auto px-4 sm:px-6 py-14">
          <h2 className="text-2xl font-bold mb-2 text-balance">Learning Tracks</h2>
          <p className="text-muted text-sm mb-8">
            Progress through four curated tracks — each building on the last.
          </p>
          <div className="grid sm:grid-cols-2 gap-4">
            {tracks.map((track) => {
              const trackLessons = getLessonsByTrack(track.id);
              const doneLessons = trackLessons.filter((l) =>
                completed.includes(l.id)
              );
              const pct = Math.round((doneLessons.length / trackLessons.length) * 100);
              const firstLesson = trackLessons[0];

              return (
                <Link
                  key={track.id}
                  href={`/learn/${firstLesson.slug}`}
                  className="group block p-5 rounded-lg border border-[var(--border)] bg-surface hover:border-[var(--muted)] transition-colors"
                >
                  <div className="flex items-start justify-between mb-3">
                    <div className="flex items-center gap-3">
                      <div
                        className="w-8 h-8 rounded-lg flex items-center justify-center shrink-0"
                        style={{ background: `${track.color}18`, color: track.color }}
                      >
                        {TRACK_ICONS[track.id]}
                      </div>
                      <div>
                        <h3 className="font-semibold text-sm text-foreground">
                          {track.label}
                        </h3>
                        <p className="text-xs text-muted mt-0.5">
                          Steps {track.range[0]}–{track.range[1]} · {trackLessons.length} lessons
                        </p>
                      </div>
                    </div>
                    <ChevronRight
                      size={15}
                      className="text-muted group-hover:text-foreground transition-colors mt-0.5 shrink-0"
                    />
                  </div>
                  <p className="text-xs text-muted leading-relaxed mb-4">
                    {track.description}
                  </p>
                  <div>
                    <div className="flex justify-between items-center mb-1.5">
                      <span className="text-xs text-muted">
                        {doneLessons.length}/{trackLessons.length} complete
                      </span>
                      <span className="text-xs font-mono" style={{ color: track.color }}>
                        {pct}%
                      </span>
                    </div>
                    <div className="h-1 rounded-full bg-[var(--surface-2)] overflow-hidden">
                      <div
                        className="h-full rounded-full transition-all duration-700"
                        style={{ width: `${pct}%`, background: track.color }}
                      />
                    </div>
                  </div>
                </Link>
              );
            })}
          </div>
        </section>

        {/* Lesson list */}
        <section className="max-w-6xl mx-auto px-4 sm:px-6 pb-16">
          <h2 className="text-2xl font-bold mb-2">All Lessons</h2>
          <p className="text-muted text-sm mb-8">
            {totalLessons} lessons covering Rust from Hello World to production REST APIs.
          </p>
          <div className="space-y-8">
            {tracks.map((track) => {
              const trackLessons = getLessonsByTrack(track.id);
              return (
                <div key={track.id}>
                  <div className="flex items-center gap-2 mb-3">
                    <span style={{ color: track.color }}>
                      {TRACK_ICONS[track.id]}
                    </span>
                    <h3
                      className="text-sm font-semibold uppercase tracking-wide"
                      style={{ color: track.color }}
                    >
                      {track.label}
                    </h3>
                  </div>
                  <div className="grid sm:grid-cols-2 lg:grid-cols-3 gap-2">
                    {trackLessons.map((lesson) => {
                      const isDone = completed.includes(lesson.id);
                      const isCurrent = currentLesson.id === lesson.id && !isDone;
                      return (
                        <Link
                          key={lesson.id}
                          href={`/learn/${lesson.slug}`}
                          className={cn(
                            "flex items-center gap-3 p-3 rounded border transition-colors group",
                            isDone
                              ? "border-success/20 bg-success/5 hover:bg-success/10"
                              : isCurrent
                              ? "border-accent/40 bg-accent/5 hover:bg-accent/10"
                              : "border-[var(--border)] bg-surface hover:border-[var(--muted)]"
                          )}
                        >
                          <span className="font-mono text-xs shrink-0 w-6 text-center text-muted">
                            {String(lesson.id).padStart(2, "0")}
                          </span>
                          {isDone ? (
                            <CheckCircle2 size={14} className="text-success shrink-0" />
                          ) : (
                            <span
                              className={cn(
                                "w-3.5 h-3.5 rounded-full border shrink-0",
                                isCurrent ? "border-accent bg-accent/20" : "border-[var(--border)]"
                              )}
                            />
                          )}
                          <span
                            className={cn(
                              "text-xs font-medium truncate",
                              isDone
                                ? "text-success"
                                : isCurrent
                                ? "text-accent"
                                : "text-muted group-hover:text-foreground"
                            )}
                          >
                            {lesson.title}
                          </span>
                        </Link>
                      );
                    })}
                  </div>
                </div>
              );
            })}
          </div>
        </section>
      </main>

      {/* Footer */}
      <footer className="border-t border-[var(--border)] py-8">
        <div className="max-w-6xl mx-auto px-4 sm:px-6 flex flex-col sm:flex-row items-center justify-between gap-4">
          <div className="flex items-center gap-2">
            <span className="font-mono font-bold text-accent">rust</span>
            <span className="font-mono font-bold">learn</span>
            <span className="text-muted text-xs ml-2">36 lessons</span>
          </div>
          <div className="flex items-center gap-4 text-xs text-muted">
            <span>Code runs via the Rust Playground API</span>
            <a
              href="https://github.com/skundu42/learn-rust"
              target="_blank"
              rel="noopener noreferrer"
              className="hover:text-foreground transition-colors flex items-center gap-1"
            >
              <Github size={12} />
              GitHub
            </a>
          </div>
        </div>
      </footer>
    </div>
  );
}

// Minimal syntax highlighter for the hero code block
function highlightRust(line: string): string {
  return line
    .replace(/&/g, "&amp;")
    .replace(/</g, "&lt;")
    .replace(/>/g, "&gt;")
    .replace(
      /\b(fn|let|mut|pub|use|struct|enum|impl|match|if|else|for|while|return|true|false|mod|pub)\b/g,
      '<span style="color:#ff7b72">$1</span>'
    )
    .replace(
      /\b(String|str|i32|u32|bool|Option|Result|Vec|Some|None|Ok|Err)\b/g,
      '<span style="color:#ffa657">$1</span>'
    )
    .replace(
      /("[^"]*")/g,
      '<span style="color:#a5d6ff">$1</span>'
    )
    .replace(
      /(\/\/[^\n]*)/g,
      '<span style="color:#8b949e">$1</span>'
    )
    .replace(
      /\b([a-zA-Z_][a-zA-Z0-9_]*)(\()/g,
      '<span style="color:#d2a8ff">$1</span>$2'
    );
}
