import NextAuth from "next-auth";
import Google from "next-auth/providers/google";

// AUTH_SECRET is picked up automatically from the (root) .env.
// Credentials are passed explicitly because the env vars are named
// GOOGLE_CLIENT_ID / GOOGLE_CLIENT_SECRET rather than Auth.js's
// auto-inferred AUTH_GOOGLE_ID / AUTH_GOOGLE_SECRET.
export const { handlers, signIn, signOut, auth } = NextAuth({
  providers: [
    Google({
      clientId: process.env.GOOGLE_CLIENT_ID,
      clientSecret: process.env.GOOGLE_CLIENT_SECRET,
    }),
  ],
  // Required when not deploying on Vercel (e.g. local dev / self-host) so
  // Auth.js trusts the incoming Host header instead of a fixed AUTH_URL.
  trustHost: true,
});
