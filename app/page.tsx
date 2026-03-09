import { LESSONS, TRACKS } from "@/lib/lessons";
import { getLessonProgress, getUser } from "@/app/auth/actions";
import {
  getCompletedLessonIdsFromProgress,
  getLastViewedLessonId,
} from "@/lib/lesson-progress";
import HomepageClient from "@/components/homepage-client";

export default async function Home() {
  const user = await getUser();
  const progress = user ? await getLessonProgress() : [];
  const completedIds = getCompletedLessonIdsFromProgress(progress);
  const inProgressCount = progress.filter((record) => record.status === "in_progress").length;
  const lastViewedLessonId = getLastViewedLessonId(progress);

  const userInfo = user
    ? { id: user.id, email: user.email ?? "", name: user.user_metadata?.full_name ?? "" }
    : null;

  return (
    <HomepageClient
      lessons={LESSONS}
      tracks={TRACKS}
      user={userInfo}
      serverCompletedIds={completedIds}
      inProgressCount={inProgressCount}
      lastViewedLessonId={lastViewedLessonId}
    />
  );
}
