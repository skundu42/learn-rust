export type VerificationMethod = "hidden-tests" | "output-check";

export interface LessonVerificationResult {
  passed: boolean;
  method: VerificationMethod;
  summary: string;
  details: string;
  progressSaved: boolean;
}

