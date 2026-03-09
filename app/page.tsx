import { LESSONS, TRACKS } from "@/lib/lessons";
import HomepageClient from "@/components/homepage-client";

export default function Home() {
  return <HomepageClient lessons={LESSONS} tracks={TRACKS} />;
}
