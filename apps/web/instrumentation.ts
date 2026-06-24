// Runs once when the Next.js server runtime boots, before any route handler or
// component module is evaluated. We load the monorepo-root .env here (shared with
// the Rust backend) so Auth.js can read AUTH_SECRET / GOOGLE_CLIENT_ID /
// GOOGLE_CLIENT_SECRET. Doing this in next.config.ts does NOT work under
// Turbopack — its server runtime doesn't inherit that process's env mutations.
export async function register() {
  // register() also runs in the Edge runtime, which has no node:path/process.cwd,
  // so only do the filesystem env load in the Node.js runtime.
  if (process.env.NEXT_RUNTIME === "nodejs") {
    const { loadEnvConfig } = await import("@next/env");
    const { resolve } = await import("node:path");
    // process.cwd() is apps/web when running next, so ../../ is the repo root.
    loadEnvConfig(resolve(process.cwd(), "../.."));
  }
}
