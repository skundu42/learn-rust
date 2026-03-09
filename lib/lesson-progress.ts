export type LessonProgressStatus = "in_progress" | "completed";

export interface LessonProgressRecord {
  lessonId: number;
  status: LessonProgressStatus;
  startedAt: string | null;
  lastViewedAt: string;
  completedAt: string | null;
  updatedAt: string;
  visitCount: number;
}

export interface LessonProgressRow {
  lesson_id: number;
  status: string | null;
  started_at: string | null;
  last_viewed_at: string | null;
  completed_at: string | null;
  updated_at: string | null;
  visit_count: number | null;
}

export function getProgressStatus(value: string | null | undefined): LessonProgressStatus {
  return value === "completed" ? "completed" : "in_progress";
}

export function normalizeProgressRow(row: LessonProgressRow): LessonProgressRecord {
  const lastViewedAt = row.last_viewed_at ?? row.updated_at ?? new Date(0).toISOString();
  const updatedAt = row.updated_at ?? lastViewedAt;

  return {
    lessonId: row.lesson_id,
    status: getProgressStatus(row.status),
    startedAt: row.started_at,
    lastViewedAt,
    completedAt: row.completed_at,
    updatedAt,
    visitCount: row.visit_count ?? 1,
  };
}

export function getCompletedLessonIdsFromProgress(progress: LessonProgressRecord[]) {
  return progress
    .filter((record) => record.status === "completed")
    .map((record) => record.lessonId)
    .sort((a, b) => a - b);
}

export function getLastViewedLessonId(progress: LessonProgressRecord[]) {
  if (progress.length === 0) return null;

  return [...progress]
    .sort(
      (left, right) =>
        new Date(right.lastViewedAt).getTime() - new Date(left.lastViewedAt).getTime()
    )[0].lessonId;
}

