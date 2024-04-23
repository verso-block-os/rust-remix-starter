import { Outlet } from "@remix-run/react";

export default function Auth() {
  return (
    <div className="flex justify-center items-center flex-1">
      <Outlet />
    </div>
  );
}
