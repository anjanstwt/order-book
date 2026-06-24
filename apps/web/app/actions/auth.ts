"use server";

import { signIn, signOut } from "@/auth";

// Used as a <form action={...}> target. signIn() throws a redirect to Google,
// which Next.js handles, so this never "returns" normally.
export async function googleSignIn() {
  await signIn("google", { redirectTo: "/" });
}

export async function appSignOut() {
  await signOut({ redirectTo: "/" });
}
