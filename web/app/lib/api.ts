import { Procedures } from "@/generated/bindings";
import { FetchTransport, createClient } from "@rspc/client";

export const api = createClient<Procedures>({
  transport: new FetchTransport("http://localtest.me:1337/rpc", (input, init) =>
    fetch(input, {
      ...init,
      cache: "no-store",
      credentials: "include",
      headers: {
        Connection: "keep-alive",
      },
    })
  ),
});
