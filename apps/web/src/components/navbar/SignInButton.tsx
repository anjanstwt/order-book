"use client";

import { useState } from "react";
import SignInModal from "./SignInModal";

export default function SignInButton() {
    const [open, setOpen] = useState(false);

    return (
        <>
            <button
                type="button"
                onClick={() => setOpen(true)}
                className="rounded-lg bg-foreground px-4 py-2 text-sm font-medium text-background hover:opacity-90"
            >
                Sign in
            </button>
            <SignInModal open={open} onClose={() => setOpen(false)} />
        </>
    );
}
