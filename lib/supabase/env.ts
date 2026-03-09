const DEFAULT_NEXT_PATH = "/learn/hello-world";

export interface SupabasePublicEnv {
  url: string;
  anonKey: string;
  siteUrl: string | null;
}

function parseUrl(name: string, value: string) {
  try {
    return new URL(value).toString().replace(/\/$/, "");
  } catch {
    throw new Error(`${name} must be a valid absolute URL.`);
  }
}

export function getSupabasePublicEnv(): SupabasePublicEnv | null {
  const url = process.env.NEXT_PUBLIC_SUPABASE_URL?.trim() ?? "";
  const anonKey = process.env.NEXT_PUBLIC_SUPABASE_ANON_KEY?.trim() ?? "";
  const siteUrl = process.env.NEXT_PUBLIC_SITE_URL?.trim() ?? "";

  if (!url && !anonKey) {
    return null;
  }

  if (!url || !anonKey) {
    throw new Error(
      "Supabase is partially configured. Set both NEXT_PUBLIC_SUPABASE_URL and NEXT_PUBLIC_SUPABASE_ANON_KEY."
    );
  }

  return {
    url: parseUrl("NEXT_PUBLIC_SUPABASE_URL", url),
    anonKey,
    siteUrl: siteUrl ? parseUrl("NEXT_PUBLIC_SITE_URL", siteUrl) : null,
  };
}

export function getSafeRedirectPath(nextPath: string | null | undefined) {
  if (!nextPath || !nextPath.startsWith("/") || nextPath.startsWith("//")) {
    return DEFAULT_NEXT_PATH;
  }

  return nextPath;
}

export function buildAuthCallbackUrl(nextPath: string, originFallback?: string) {
  const env = getSupabasePublicEnv();
  const baseUrl = env?.siteUrl ?? originFallback;

  if (!baseUrl) {
    throw new Error(
      "NEXT_PUBLIC_SITE_URL is required for email auth redirects outside the browser."
    );
  }

  const url = new URL("/auth/callback", baseUrl);
  url.searchParams.set("next", getSafeRedirectPath(nextPath));
  return url.toString();
}

