"use server";

import { revalidatePath } from "next/cache";
import { redirect } from "next/navigation";
import {
  getCompletedLessonIdsFromProgress,
  type LessonProgressRecord,
  type LessonProgressRow,
  normalizeProgressRow,
} from "@/lib/lesson-progress";
import { verifyLessonCode } from "@/lib/lesson-verification";
import type { LessonVerificationResult } from "@/lib/lesson-verification-types";
import { createClient } from "@/lib/supabase/server";

async function getAuthenticatedContext() {
  const supabase = await createClient();
  if (!supabase) return { supabase: null, user: null };

  const {
    data: { user },
  } = await supabase.auth.getUser();

  return { supabase, user };
}

async function getExistingLessonProgress(
  supabase: NonNullable<Awaited<ReturnType<typeof createClient>>>,
  userId: string,
  lessonId: number
) {
  const { data, error } = await supabase
    .from("lesson_progress")
    .select(
      "lesson_id,status,started_at,last_viewed_at,completed_at,updated_at,visit_count"
    )
    .eq("user_id", userId)
    .eq("lesson_id", lessonId)
    .maybeSingle();

  if (error || !data) return null;

  return normalizeProgressRow(data as LessonProgressRow);
}

function revalidateProgressViews() {
  revalidatePath("/", "layout");
  revalidatePath("/learn", "layout");
}

export async function signOut() {
  const supabase = await createClient();
  if (!supabase) redirect("/");

  await supabase.auth.signOut();
  revalidatePath("/", "layout");
  redirect("/");
}

export async function getUser() {
  const supabase = await createClient();
  if (!supabase) return null;

  const {
    data: { user },
  } = await supabase.auth.getUser();
  return user;
}

export async function getLessonProgress(): Promise<LessonProgressRecord[]> {
  const { supabase, user } = await getAuthenticatedContext();
  if (!supabase || !user) return [];

  const { data, error } = await supabase
    .from("lesson_progress")
    .select(
      "lesson_id,status,started_at,last_viewed_at,completed_at,updated_at,visit_count"
    )
    .eq("user_id", user.id)
    .order("lesson_id", { ascending: true });

  if (error || !data) return [];

  return (data as LessonProgressRow[]).map(normalizeProgressRow);
}

export async function getCompletedLessonIds(): Promise<number[]> {
  const progress = await getLessonProgress();
  return getCompletedLessonIdsFromProgress(progress);
}

export async function trackLessonViewServer(lessonId: number) {
  const { supabase, user } = await getAuthenticatedContext();
  if (!supabase) return { error: "Supabase is not configured" };
  if (!user) return { error: "Not authenticated" };

  const existing = await getExistingLessonProgress(supabase, user.id, lessonId);
  const timestamp = new Date().toISOString();

  if (!existing) {
    const { error } = await supabase.from("lesson_progress").insert({
      user_id: user.id,
      lesson_id: lessonId,
      status: "in_progress",
      started_at: timestamp,
      last_viewed_at: timestamp,
      updated_at: timestamp,
      visit_count: 1,
    });

    return { error: error?.message ?? null };
  }

  const { error } = await supabase
    .from("lesson_progress")
    .update({
      started_at: existing.startedAt ?? timestamp,
      last_viewed_at: timestamp,
      updated_at: timestamp,
      visit_count: existing.visitCount + 1,
    })
    .eq("user_id", user.id)
    .eq("lesson_id", lessonId);

  return { error: error?.message ?? null };
}

export async function markLessonCompleteServer(lessonId: number) {
  const { supabase, user } = await getAuthenticatedContext();
  if (!supabase) return { error: "Supabase is not configured" };
  if (!user) return { error: "Not authenticated" };

  const existing = await getExistingLessonProgress(supabase, user.id, lessonId);
  const timestamp = new Date().toISOString();

  const { error } = existing
    ? await supabase
        .from("lesson_progress")
        .update({
          status: "completed",
          started_at: existing.startedAt ?? timestamp,
          last_viewed_at: timestamp,
          completed_at: timestamp,
          updated_at: timestamp,
          visit_count: existing.visitCount,
        })
        .eq("user_id", user.id)
        .eq("lesson_id", lessonId)
    : await supabase.from("lesson_progress").insert({
        user_id: user.id,
        lesson_id: lessonId,
        status: "completed",
        started_at: timestamp,
        last_viewed_at: timestamp,
        completed_at: timestamp,
        updated_at: timestamp,
        visit_count: 1,
      });

  if (error) return { error: error.message };
  revalidateProgressViews();
  return { error: null };
}

export async function markLessonIncompleteServer(lessonId: number) {
  const { supabase, user } = await getAuthenticatedContext();
  if (!supabase) return { error: "Supabase is not configured" };
  if (!user) return { error: "Not authenticated" };

  const existing = await getExistingLessonProgress(supabase, user.id, lessonId);
  const timestamp = new Date().toISOString();

  const { error } = existing
    ? await supabase
        .from("lesson_progress")
        .update({
          status: "in_progress",
          started_at: existing.startedAt ?? timestamp,
          last_viewed_at: timestamp,
          completed_at: null,
          updated_at: timestamp,
          visit_count: existing.visitCount,
        })
        .eq("user_id", user.id)
        .eq("lesson_id", lessonId)
    : await supabase.from("lesson_progress").insert({
        user_id: user.id,
        lesson_id: lessonId,
        status: "in_progress",
        started_at: timestamp,
        last_viewed_at: timestamp,
        completed_at: null,
        updated_at: timestamp,
        visit_count: 1,
      });

  if (error) return { error: error.message };
  revalidateProgressViews();
  return { error: null };
}

export async function verifyLessonSolutionServer(
  lessonId: number,
  code: string
): Promise<LessonVerificationResult> {
  const result = await verifyLessonCode(lessonId, code);

  if (!result.passed) {
    return result;
  }

  const user = await getUser();
  if (!user) {
    return {
      ...result,
      summary: `${result.summary} Sign in to save this completion to your progress.`,
      progressSaved: false,
    };
  }

  const completion = await markLessonCompleteServer(lessonId);
  if (completion.error) {
    return {
      ...result,
      summary: `${result.summary} The solution passed, but progress could not be saved.`,
      details: [result.details, `Progress save error: ${completion.error}`]
        .filter(Boolean)
        .join("\n\n"),
      progressSaved: false,
    };
  }

  return {
    ...result,
    summary: `${result.summary} Marked complete.`,
    progressSaved: true,
  };
}
