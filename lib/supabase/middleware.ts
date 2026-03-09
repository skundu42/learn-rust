import { createServerClient, type SetAllCookies } from "@supabase/ssr";
import { NextResponse, type NextRequest } from "next/server";
import { getSupabasePublicEnv } from "@/lib/supabase/env";

export async function updateSession(request: NextRequest) {
  let supabaseResponse = NextResponse.next({ request });
  const env = getSupabasePublicEnv();

  let user = null;

  if (env) {
    const supabase = createServerClient(
      env.url,
      env.anonKey,
      {
        cookies: {
          getAll() {
            return request.cookies.getAll();
          },
          setAll(cookiesToSet: Parameters<SetAllCookies>[0]) {
            cookiesToSet.forEach(({ name, value }) =>
              request.cookies.set(name, value)
            );
            supabaseResponse = NextResponse.next({ request });
            cookiesToSet.forEach(({ name, value, options }) =>
              supabaseResponse.cookies.set(name, value, options)
            );
          },
        },
      }
    );

    // Must call getUser() to keep session alive — do not add code between
    // createServerClient and this call.
    const {
      data: { user: currentUser },
    } = await supabase.auth.getUser();

    user = currentUser;
  }

  // Protect all /learn/* routes beyond the first lesson for unauthenticated users.
  // The first lesson slug is "hello-world"; everything else requires sign-in.
  const isLearnRoute = request.nextUrl.pathname.startsWith("/learn/");
  const isFirstLesson = request.nextUrl.pathname === "/learn/hello-world";

  if (isLearnRoute && !isFirstLesson && !user) {
    const url = request.nextUrl.clone();
    url.pathname = "/auth/login";
    url.searchParams.set("next", request.nextUrl.pathname);
    return NextResponse.redirect(url);
  }

  return supabaseResponse;
}
