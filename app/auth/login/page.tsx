"use client";

import { useState, useEffect } from "react";
import Link from "next/link";
import { useRouter, useSearchParams } from "next/navigation";
import { Eye, EyeOff, Loader2, AlertCircle } from "lucide-react";
import { createClient } from "@/lib/supabase/client";
import { cn } from "@/lib/utils";

export default function LoginPage() {
  const router = useRouter();
  const searchParams = useSearchParams();
  const next = searchParams.get("next") ?? "/learn/hello-world";

  const [email, setEmail] = useState("");
  const [password, setPassword] = useState("");
  const [showPassword, setShowPassword] = useState(false);
  const [loading, setLoading] = useState(false);
  const [error, setError] = useState<string | null>(null);

  // If already signed in, redirect
  useEffect(() => {
    const supabase = createClient();
    supabase.auth.getUser().then(({ data: { user } }) => {
      if (user) router.replace(next);
    });
  }, [next, router]);

  const handleSubmit = async (e: React.FormEvent) => {
    e.preventDefault();
    setError(null);
    setLoading(true);

    const supabase = createClient();
    const { error: signInError } = await supabase.auth.signInWithPassword({
      email,
      password,
    });

    if (signInError) {
      setError(signInError.message);
      setLoading(false);
      return;
    }

    router.push(next);
    router.refresh();
  };

  return (
    <div className="min-h-screen bg-background flex items-center justify-center px-4">
      <div className="w-full max-w-sm">
        {/* Logo */}
        <div className="text-center mb-8">
          <Link href="/" className="inline-flex items-center gap-1 font-mono font-bold text-2xl">
            <span className="text-accent">rust</span>
            <span className="text-foreground">learn</span>
          </Link>
          <p className="text-sm text-muted mt-2">Sign in to access all 36 lessons</p>
        </div>

        {/* Card */}
        <div className="bg-surface border border-[var(--border)] rounded-lg p-6">
          <h1 className="text-lg font-semibold text-foreground mb-6">Sign in</h1>

          <form onSubmit={handleSubmit} className="space-y-4">
            <div>
              <label htmlFor="email" className="block text-xs font-medium text-muted mb-1.5">
                Email address
              </label>
              <input
                id="email"
                type="email"
                autoComplete="email"
                required
                value={email}
                onChange={(e) => setEmail(e.target.value)}
                placeholder="you@example.com"
                className={cn(
                  "w-full bg-[var(--surface-2)] border border-[var(--border)] rounded px-3 py-2",
                  "text-sm text-foreground placeholder:text-muted",
                  "focus:outline-none focus:border-accent transition-colors"
                )}
              />
            </div>

            <div>
              <label htmlFor="password" className="block text-xs font-medium text-muted mb-1.5">
                Password
              </label>
              <div className="relative">
                <input
                  id="password"
                  type={showPassword ? "text" : "password"}
                  autoComplete="current-password"
                  required
                  value={password}
                  onChange={(e) => setPassword(e.target.value)}
                  placeholder="••••••••"
                  className={cn(
                    "w-full bg-[var(--surface-2)] border border-[var(--border)] rounded px-3 py-2 pr-9",
                    "text-sm text-foreground placeholder:text-muted",
                    "focus:outline-none focus:border-accent transition-colors"
                  )}
                />
                <button
                  type="button"
                  onClick={() => setShowPassword(!showPassword)}
                  className="absolute right-2.5 top-1/2 -translate-y-1/2 text-muted hover:text-foreground transition-colors"
                  aria-label={showPassword ? "Hide password" : "Show password"}
                >
                  {showPassword ? <EyeOff size={14} /> : <Eye size={14} />}
                </button>
              </div>
            </div>

            {error && (
              <div className="flex items-start gap-2 p-3 rounded bg-danger/10 border border-danger/20">
                <AlertCircle size={14} className="text-danger shrink-0 mt-0.5" />
                <p className="text-xs text-danger leading-relaxed">{error}</p>
              </div>
            )}

            <button
              type="submit"
              disabled={loading}
              className={cn(
                "w-full flex items-center justify-center gap-2 py-2.5 rounded",
                "bg-accent text-white text-sm font-medium",
                "hover:bg-accent/90 transition-colors",
                "disabled:opacity-60 disabled:cursor-not-allowed"
              )}
            >
              {loading && <Loader2 size={14} className="animate-spin" />}
              {loading ? "Signing in..." : "Sign in"}
            </button>
          </form>
        </div>

        <p className="text-center text-xs text-muted mt-4">
          Don&apos;t have an account?{" "}
          <Link
            href={`/auth/sign-up${next !== "/learn/hello-world" ? `?next=${encodeURIComponent(next)}` : ""}`}
            className="text-accent hover:underline"
          >
            Create one free
          </Link>
        </p>

        <p className="text-center text-xs text-muted mt-3">
          <Link href="/learn/hello-world" className="hover:text-foreground transition-colors">
            Continue as guest (lesson 1 only)
          </Link>
        </p>
      </div>
    </div>
  );
}
