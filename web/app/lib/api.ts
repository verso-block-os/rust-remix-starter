import { FetchTransport, createClient } from "@rspc/client";

import { Procedures } from "@/generated/bindings";

export const api = createClient<Procedures>({
  transport: new FetchTransport("http://127.0.0.1:1337/rpc", (input, init) =>
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
