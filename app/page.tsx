import { LESSONS, TRACKS } from "@/lib/lessons";
import { getUser, getCompletedLessonIds } from "@/app/auth/actions";
import HomepageClient from "@/components/homepage-client";

export default async function Home() {
  const user = await getUser();
  const completedIds = user ? await getCompletedLessonIds() : [];

  const userInfo = user
    ? { id: user.id, email: user.email ?? "", name: user.user_metadata?.full_name ?? "" }
    : null;

  return (
    <HomepageClient
      lessons={LESSONS}
      tracks={TRACKS}
      user={userInfo}
      serverCompletedIds={completedIds}
    />
  );
}
