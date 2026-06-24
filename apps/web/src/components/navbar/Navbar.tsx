import { cn } from "@/lib/utils";
import { auth } from "@/auth";
import { appSignOut } from "@/app/actions/auth";
import SignInButton from "./SignInButton";


export default async function Navbar() {
    const session = await auth();
    const user = session?.user;

    return (
        <div className={cn(
            "w-full h-16 bg-red-200",
            "flex items-center justify-end gap-3 px-6"
        )}>

            {user ? (
                <>
                    {user.image && (
                        // eslint-disable-next-line @next/next/no-img-element -- avatar from Google CDN; avoids configuring next/image remotePatterns
                        <img
                            src={user.image}
                            alt=""
                            className="h-8 w-8 rounded-full"
                        />
                    )}
                    <span className="text-sm font-medium">{user.name}</span>
                    <form action={appSignOut}>
                        <button
                            type="submit"
                            className="rounded-lg border border-foreground/15 px-4 py-2 text-sm font-medium hover:bg-foreground/5"
                        >
                            Sign out
                        </button>
                    </form>
                </>
            ) : (
                <SignInButton />
            )}

        </div>
    )
}