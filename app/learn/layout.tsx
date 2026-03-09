import { getUser, getCompletedLessonIds } from "@/app/auth/actions";
import LearnLayoutClient from "@/components/learn-layout-client";

export default async function LearnLayout({ children }: { children: React.ReactNode }) {
  const user = await getUser();
  const completedIds = user ? await getCompletedLessonIds() : [];

  const userInfo = user
    ? { id: user.id, email: user.email ?? "", name: user.user_metadata?.full_name ?? "" }
    : null;

  return (
    <LearnLayoutClient user={userInfo} completedIds={completedIds}>
      {children}
    </LearnLayoutClient>
  );
}
