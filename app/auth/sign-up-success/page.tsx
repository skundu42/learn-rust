import Link from "next/link";
import { MailCheck } from "lucide-react";

export default function SignUpSuccessPage() {
  return (
    <div className="min-h-screen bg-background flex items-center justify-center px-4">
      <div className="w-full max-w-sm text-center">
        <Link href="/" className="inline-flex items-center gap-1 font-mono font-bold text-2xl mb-8">
          <span className="text-accent">rust</span>
          <span className="text-foreground">learn</span>
        </Link>

        <div className="bg-surface border border-[var(--border)] rounded-lg p-8">
          <div className="w-12 h-12 rounded-full bg-success/10 flex items-center justify-center mx-auto mb-4">
            <MailCheck size={24} className="text-success" />
          </div>

          <h1 className="text-lg font-semibold text-foreground mb-2">Check your email</h1>
          <p className="text-sm text-muted leading-relaxed mb-6">
            We sent a confirmation link to your email address. Click it to
            activate your account and unlock all 36 lessons.
          </p>

          <Link
            href="/auth/login"
            className="inline-flex items-center justify-center w-full py-2.5 rounded bg-accent text-white text-sm font-medium hover:bg-accent/90 transition-colors"
          >
            Back to sign in
          </Link>
        </div>

        <p className="text-xs text-muted mt-4">
          While you wait,{" "}
          <Link href="/learn/hello-world" className="text-accent hover:underline">
            try lesson 1 as a guest
          </Link>
        </p>
      </div>
    </div>
  );
}
