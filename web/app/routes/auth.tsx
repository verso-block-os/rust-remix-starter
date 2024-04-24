import { getSession } from "@/lib/get-session";
import { LoaderFunction, redirect } from "@remix-run/node";
import { Outlet } from "@remix-run/react";

export const loader: LoaderFunction = async ({ request }) => {
  const cookies = request.headers.get("Cookie");
  const session = await getSession(cookies);

  if (session) {
    throw redirect("/");
  }

  return null;
};

export default function Auth() {
  return (
    <div className="flex justify-center items-center flex-1 relative p-4">
      <div className="absolute inset-0 ">
        <div className="absolute h-full w-full bg-[radial-gradient(#e5e7eb_1px,transparent_1px)] dark:bg-[radial-gradient(#424242_1px,transparent_1px)] [background-size:16px_16px] [mask-image:radial-gradient(ellipse_50%_50%_at_50%_50%,#000_70%,transparent_100%)]"></div>
      </div>
      <Outlet />
    </div>
  );
}
