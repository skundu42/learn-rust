import { notFound } from "next/navigation";
import { getLessonBySlug, LESSONS } from "@/lib/lessons";
import LessonView from "@/components/lesson-view";
import { getUser, getCompletedLessonIds } from "@/app/auth/actions";

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
  const completedIds = user ? await getCompletedLessonIds() : [];

  return (
    <LessonView
      lesson={lesson}
      user={user ? { id: user.id, email: user.email ?? "", name: user.user_metadata?.full_name ?? "" } : null}
      initialCompleted={completedIds}
    />
  );
}
