import stylesheet from "@/index.css?url";
import { LinksFunction } from "@remix-run/node";
import { Links, Meta, Outlet, Scripts, ScrollRestoration } from "@remix-run/react";
import { Toaster } from "./components/ui/sonner";

export const links: LinksFunction = () => [{ rel: "stylesheet", href: stylesheet }];

export function Layout({ children }: { children: React.ReactNode }) {
  return (
    <html lang="en">
      <head>
        <meta charSet="utf-8" />
        <meta name="viewport" content="width=device-width, initial-scale=1" />
        <Meta />
        <Links />
      </head>
      <body className="absolute inset-0 flex flex-col">
        {children}
        <ScrollRestoration />
        <Scripts />
        <Toaster position="bottom-center" />
      </body>
    </html>
  );
}

export default function App() {
  return <Outlet />;
}
