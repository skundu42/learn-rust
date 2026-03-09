import { NextResponse, type NextRequest } from "next/server";
import { createClient } from "@/lib/supabase/server";
import { getSafeRedirectPath } from "@/lib/supabase/env";

export async function GET(request: NextRequest) {
  const code = request.nextUrl.searchParams.get("code");
  const nextPath = getSafeRedirectPath(request.nextUrl.searchParams.get("next"));

  if (!code) {
    return NextResponse.redirect(new URL("/auth/error", request.url));
  }

  const supabase = await createClient();
  if (!supabase) {
    return NextResponse.redirect(new URL("/auth/error?reason=config", request.url));
  }

  const { error } = await supabase.auth.exchangeCodeForSession(code);

  if (error) {
    return NextResponse.redirect(new URL("/auth/error", request.url));
  }

  return NextResponse.redirect(new URL(nextPath, request.url));
}

