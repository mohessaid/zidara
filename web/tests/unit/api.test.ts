import "fake-indexeddb/auto";
import { describe, expect, it } from "vitest";
import { Api, ApiError } from "../../src/lib/api.ts";
import { openStore } from "../../src/lib/store.ts";

type Call = { url: string; init: RequestInit | undefined };

function fakeFetch(routes: Record<string, (init?: RequestInit) => Response>) {
  const calls: Call[] = [];
  const fetch = async (url: string | URL | Request, init?: RequestInit) => {
    const key = `${init?.method ?? "GET"} ${String(url)}`;
    calls.push({ url: String(url), init });
    const route = routes[key];
    if (!route) throw new Error(`unexpected ${key}`);
    return route(init);
  };
  return { fetch: fetch as typeof globalThis.fetch, calls };
}

const json = (body: unknown, status = 200) => new Response(JSON.stringify(body), { status, headers: { "content-type": "application/json" } });

describe("Api", () => {
  it("signs in and sends the token after", async () => {
    const { fetch, calls } = fakeFetch({
      "POST /api/login": () => json({ token: "tok" }),
      "GET /api/me": () => json({ id: "u", display_name: "Amina", cohorts: [] }),
    });
    const api = await Api.signIn("u", fetch);
    expect(api.token).toBe("tok");
    expect(JSON.parse(String(calls[0]!.init!.body))).toEqual({ user_id: "u" });
    expect((await api.me()).display_name).toBe("Amina");
    expect(new Headers(calls[1]!.init!.headers).get("authorization")).toBe("Bearer tok");
  });

  it("posts commands and pages through events", async () => {
    const page = (n: number, from: number) => Array.from({ length: n }, (_, i) => ({ seq: from + i, event: { at: from + i } }));
    const { fetch, calls } = fakeFetch({
      "POST /api/cohorts/c/commands": () => json([{ id: "e", outcome: { status: "duplicate" } }]),
      "GET /api/cohorts/c/events?limit=500": () => json(page(500, 1)),
      "GET /api/cohorts/c/events?limit=500&after_ms=500&after_seq=500": () => json(page(3, 501)),
      "GET /api/cohorts/c/events?limit=500&after_ms=503&after_seq=503": () => json([]),
    });
    const api = new Api("tok", fetch);
    const results = await api.commands("c", [{ actor: { kind: "system" }, envelope: { id: "e", at: 1, command: { type: "undo", target: "x" } } }]);
    expect(results).toEqual([{ id: "e", outcome: { status: "duplicate" } }]);
    expect(JSON.parse(String(calls[0]!.init!.body))).toEqual([{ id: "e", at: 1, command: { type: "undo", target: "x" } }]);
    const events = await api.events("c");
    expect(events).toHaveLength(503);
    expect(events[502]).toEqual({ at: 503 });
  });

  it("turns HTTP errors into ApiError with the server's message", async () => {
    const { fetch } = fakeFetch({ "GET /api/cohorts/c/studio?locale=en": () => json({ error: "your role cannot see this" }, 403) });
    const error = await new Api("tok", fetch).studio("c", "en").catch((e: unknown) => e);
    expect(error).toBeInstanceOf(ApiError);
    expect(error).toMatchObject({ status: 403, message: "your role cannot see this" });
  });

  it("caches lesson packs for offline use", async () => {
    const store = await openStore("api-pack-test");
    let served = 0;
    const { fetch } = fakeFetch({ "GET /api/packs/p": () => (served++, json({ id: "p" })) });
    const api = new Api("tok", fetch);
    expect(await api.pack("p", store)).toEqual({ id: "p" });
    const offline = new Api("tok", (async () => { throw new TypeError("Failed to fetch"); }) as typeof globalThis.fetch);
    expect(await offline.pack("p", store)).toEqual({ id: "p" });
    expect(served).toBe(1);
    await expect(offline.pack("q", store)).rejects.toThrow(/Failed to fetch/);
  });
});
