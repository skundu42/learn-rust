"use client";

import { useEffect, useRef } from "react";
import { trackLessonViewServer } from "@/app/auth/actions";

interface ProgressTrackerProps {
  enabled: boolean;
  lessonId: number;
}

export default function ProgressTracker({ enabled, lessonId }: ProgressTrackerProps) {
  const hasTrackedRef = useRef(false);

  useEffect(() => {
    if (!enabled || hasTrackedRef.current) return;

    hasTrackedRef.current = true;
    void trackLessonViewServer(lessonId);
  }, [enabled, lessonId]);

  return null;
}

