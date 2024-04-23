import { Button } from "@/components/ui/button";
import { Input } from "@/components/ui/input";
import { Link } from "@remix-run/react";
import { RiDiscordFill, RiGoogleFill } from "react-icons/ri";

export default function Login() {
  return (
    <form className="rounded-md shadow-lg">
      <div className="rounded-md p-8 flex flex-col items-center border border-solid">
        <img className="h-8 mb-8" src="/logo.svg" alt="my logo" />
        <h1 className="font-semibold mb-2">Login to Rust Remix Starter</h1>
        <p className="text-muted-foreground text-sm mb-8">Welcome back! Please login to continue</p>
        <div className="flex gap-2 w-full mb-8">
          <Button variant="outline" className="h-8 flex-1 gap-4" size="sm">
            <RiGoogleFill />
            Google
          </Button>
          <Button variant="outline" className="h-8 flex-1 gap-4" size="sm">
            <RiDiscordFill />
            Discord
          </Button>
        </div>
        <div className="w-full mb-8">
          <Input className="h-8" />
        </div>
        <Button className="h-8 gap-4 w-full" size="sm">
          Continue
        </Button>
      </div>
      <div className="bg-muted p-4 text-center text-sm">
        <span className="text-muted-foreground">Don't have an account? </span>
        <Link to="/auth/register">Register</Link>
      </div>
    </form>
  );
}
