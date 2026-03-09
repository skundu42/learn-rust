import { clsx, type ClassValue } from "clsx";
import { twMerge } from "tailwind-merge";

export function cn(...inputs: ClassValue[]) {
  return twMerge(clsx(inputs));
}

export function getCompletedLessons(): number[] {
  if (typeof window === "undefined") return [];
  try {
    const stored = localStorage.getItem("rustlearn_completed");
    return stored ? JSON.parse(stored) : [];
  } catch {
    return [];
  }
}

export function markLessonComplete(id: number): void {
  if (typeof window === "undefined") return;
  const completed = getCompletedLessons();
  if (!completed.includes(id)) {
    completed.push(id);
    localStorage.setItem("rustlearn_completed", JSON.stringify(completed));
  }
}

export function markLessonIncomplete(id: number): void {
  if (typeof window === "undefined") return;
  const completed = getCompletedLessons().filter((c) => c !== id);
  localStorage.setItem("rustlearn_completed", JSON.stringify(completed));
}

export function formatDifficulty(d: string): string {
  return d.charAt(0).toUpperCase() + d.slice(1);
}
