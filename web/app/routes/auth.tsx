import { Outlet } from "@remix-run/react";

export default function Auth() {
  return (
    <div className="flex justify-center items-center flex-1 relative p-4">
      <div className="absolute inset-0 bg-white">
        <div className="absolute h-full w-full bg-[radial-gradient(#e5e7eb_1px,transparent_1px)] [background-size:16px_16px] [mask-image:radial-gradient(ellipse_50%_50%_at_50%_50%,#000_70%,transparent_100%)]"></div>
      </div>
      <Outlet />
    </div>
  );
}
