"use client";

import { useEffect } from "react";
import { cn } from "@/lib/utils";
import { googleSignIn } from "@/app/actions/auth";

interface SignInModalProps {
    open: boolean;
    onClose: () => void;
}

export default function SignInModal({ open, onClose }: SignInModalProps) {
    // Close on Escape while the modal is open.
    useEffect(() => {
        if (!open) return;
        const onKey = (e: KeyboardEvent) => {
            if (e.key === "Escape") onClose();
        };
        document.addEventListener("keydown", onKey);
        return () => document.removeEventListener("keydown", onKey);
    }, [open, onClose]);

    if (!open) return null;

    return (
        // Backdrop: clicking it closes the modal.
        <div
            className="fixed inset-0 z-50 flex items-center justify-center bg-black/50 p-4"
            role="dialog"
            aria-modal="true"
            aria-labelledby="signin-title"
            onClick={onClose}
        >
            {/* Panel: stop propagation so inner clicks don't close it. */}
            <div
                className={cn(
                    "w-full max-w-sm rounded-2xl bg-background p-6 shadow-xl",
                    "border border-foreground/10"
                )}
                onClick={(e) => e.stopPropagation()}
            >
                <div className="flex items-center justify-between">
                    <h2 id="signin-title" className="text-lg font-semibold">
                        Sign in
                    </h2>
                    <button
                        type="button"
                        aria-label="Close"
                        onClick={onClose}
                        className="rounded-md p-1 text-foreground/60 hover:bg-foreground/5 hover:text-foreground"
                    >
                        ✕
                    </button>
                </div>

                {/* Real auth: Google OAuth via Auth.js server action. */}
                <form action={googleSignIn} className="mt-5">
                    <button
                        type="submit"
                        className="flex w-full items-center justify-center gap-2 rounded-lg border border-foreground/15 px-4 py-2 font-medium hover:bg-foreground/5"
                    >
                        <svg className="h-4 w-4" viewBox="0 0 24 24" aria-hidden="true">
                            <path
                                fill="#4285F4"
                                d="M22.56 12.25c0-.78-.07-1.53-.2-2.25H12v4.26h5.92a5.06 5.06 0 0 1-2.2 3.32v2.77h3.57c2.08-1.92 3.27-4.74 3.27-8.1z"
                            />
                            <path
                                fill="#34A853"
                                d="M12 23c2.97 0 5.46-.98 7.28-2.66l-3.57-2.77c-.98.66-2.23 1.06-3.71 1.06-2.86 0-5.29-1.93-6.16-4.53H2.18v2.84A11 11 0 0 0 12 23z"
                            />
                            <path
                                fill="#FBBC05"
                                d="M5.84 14.1a6.6 6.6 0 0 1 0-4.2V7.06H2.18a11 11 0 0 0 0 9.88l3.66-2.84z"
                            />
                            <path
                                fill="#EA4335"
                                d="M12 5.38c1.62 0 3.06.56 4.21 1.64l3.15-3.15C17.45 2.09 14.97 1 12 1A11 11 0 0 0 2.18 7.06l3.66 2.84C6.71 7.31 9.14 5.38 12 5.38z"
                            />
                        </svg>
                        Continue with Google
                    </button>
                </form>

                {/* Divider */}
                <div className="my-4 flex items-center gap-3 text-xs text-foreground/40">
                    <span className="h-px flex-1 bg-foreground/10" />
                    or
                    <span className="h-px flex-1 bg-foreground/10" />
                </div>

                <form
                    className="flex flex-col gap-4"
                    onSubmit={(e) => {
                        e.preventDefault();
                        // TODO: wire up real authentication here.
                        onClose();
                    }}
                >
                    <label className="flex flex-col gap-1 text-sm">
                        <span className="text-foreground/70">Email</span>
                        <input
                            type="email"
                            required
                            autoFocus
                            placeholder="you@example.com"
                            className="rounded-lg border border-foreground/15 bg-transparent px-3 py-2 outline-none focus:border-foreground/40"
                        />
                    </label>

                    <label className="flex flex-col gap-1 text-sm">
                        <span className="text-foreground/70">Password</span>
                        <input
                            type="password"
                            required
                            placeholder="••••••••"
                            className="rounded-lg border border-foreground/15 bg-transparent px-3 py-2 outline-none focus:border-foreground/40"
                        />
                    </label>

                    <button
                        type="submit"
                        className="mt-2 rounded-lg bg-foreground px-4 py-2 font-medium text-background hover:opacity-90"
                    >
                        Sign in
                    </button>
                </form>
            </div>
        </div>
    );
}
