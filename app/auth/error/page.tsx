import Link from "next/link";
import { AlertTriangle } from "lucide-react";

interface AuthErrorPageProps {
  searchParams?: Promise<{ reason?: string }>;
}

export default async function AuthErrorPage({ searchParams }: AuthErrorPageProps) {
  const params = searchParams ? await searchParams : undefined;
  const message =
    params?.reason === "config"
      ? "Supabase auth is not configured correctly. Check your public env vars and callback URL."
      : "Something went wrong during sign in. The link may have expired or already been used.";

  return (
    <div className="min-h-screen bg-background flex items-center justify-center px-4">
      <div className="w-full max-w-sm text-center">
        <Link href="/" className="inline-flex items-center gap-1 font-mono font-bold text-2xl mb-8">
          <span className="text-accent">rust</span>
          <span className="text-foreground">learn</span>
        </Link>

        <div className="bg-surface border border-danger/20 rounded-lg p-8">
          <div className="w-12 h-12 rounded-full bg-danger/10 flex items-center justify-center mx-auto mb-4">
            <AlertTriangle size={24} className="text-danger" />
          </div>

          <h1 className="text-lg font-semibold text-foreground mb-2">Authentication error</h1>
          <p className="text-sm text-muted leading-relaxed mb-6">
            {message}
          </p>

          <Link
            href="/auth/login"
            className="inline-flex items-center justify-center w-full py-2.5 rounded bg-accent text-white text-sm font-medium hover:bg-accent/90 transition-colors"
          >
            Try again
          </Link>
        </div>
      </div>
    </div>
  );
}
