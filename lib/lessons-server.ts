import fs from "node:fs";
import path from "node:path";
import { LESSONS, getLessonBySlug as _getLessonBySlug, type Lesson } from "./lessons";

const PROJECT_ROOT = path.join(process.cwd());

/**
 * Reads the starter code for a lesson from its corresponding .rs file on disk.
 * Falls back to an empty string if the file doesn't exist.
 */
function readStarterCode(starterFile: string): string {
  try {
    return fs.readFileSync(path.join(PROJECT_ROOT, starterFile), "utf-8");
  } catch {
    return "// Could not load starter code.";
  }
}

/** Returns a single lesson with starterCode populated from disk. */
export function getLessonBySlug(slug: string): Lesson | undefined {
  const lesson = _getLessonBySlug(slug);
  if (!lesson) return undefined;
  return {
    ...lesson,
    starterCode: readStarterCode(lesson.starterFile),
  };
}

/** Returns all lessons with starterCode populated (used for static param generation). */
export function getAllLessonsWithCode(): Lesson[] {
  return LESSONS.map((lesson) => ({
    ...lesson,
    starterCode: readStarterCode(lesson.starterFile),
  }));
}
