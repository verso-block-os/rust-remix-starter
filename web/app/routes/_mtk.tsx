import { Outlet } from "@remix-run/react";

export default function Index() {
  return (
    <div className="flex justify-center items-center flex-1 relative p-4">
      <Outlet />
    </div>
  );
}
