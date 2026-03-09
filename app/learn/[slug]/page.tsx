import { notFound } from "next/navigation";
import { LESSONS } from "@/lib/lessons";
import { getLessonBySlug } from "@/lib/lessons-server";
import LessonView from "@/components/lesson-view";
import { getLessonProgress, getUser } from "@/app/auth/actions";
import { getCompletedLessonIdsFromProgress } from "@/lib/lesson-progress";

interface PageProps {
  params: Promise<{ slug: string }>;
}

export async function generateStaticParams() {
  return LESSONS.map((l) => ({ slug: l.slug }));
}

export async function generateMetadata({ params }: PageProps) {
  const { slug } = await params;
  const lesson = getLessonBySlug(slug);
  if (!lesson) return {};
  return {
    title: `${lesson.title} — RustLearn`,
    description: lesson.description,
  };
}

export default async function LessonPage({ params }: PageProps) {
  const { slug } = await params;
  const lesson = getLessonBySlug(slug);
  if (!lesson) notFound();

  const user = await getUser();
  const progress = user ? await getLessonProgress() : [];
  const completedIds = getCompletedLessonIdsFromProgress(progress);

  return (
    <LessonView
      lesson={lesson}
      user={user ? { id: user.id, email: user.email ?? "", name: user.user_metadata?.full_name ?? "" } : null}
      initialCompleted={completedIds}
    />
  );
}
