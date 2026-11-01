// The server's JSON API. Same origin (vite proxies /api in dev and preview).
import type { RunResult } from "./kernel/kernel.ts";
import type { Actor } from "./kernel/types/Actor.ts";
import type { Envelope } from "./kernel/types/Envelope.ts";
import type { Event } from "./kernel/types/Event.ts";
import type { LessonPack } from "./kernel/types/LessonPack.ts";
import type { StudioView } from "./kernel/types/StudioView.ts";
import type { CohortMembership, Store } from "./store.ts";
import type { SyncApi } from "./sync.ts";
import { apiFetch } from "./transport.ts";

export type Me = { id: string; display_name: string; cohorts: (CohortMembership & { tenant_id: string; framework: string })[] };
export type Person = { id: string; display_name: string };

export class ApiError extends Error {
  constructor(
    readonly status: number,
    message: string,
  ) {
    super(message);
  }
}

const PAGE = 500;

export class Api implements SyncApi {
  constructor(
    readonly token: string,
    private readonly fetcher: typeof fetch = apiFetch,
  ) {}

  /** Demo sign-in by user id (the server allows it only when ZIDARA_DEMO_LOGIN=1). */
  static async signIn(user: string, fetcher: typeof fetch = apiFetch): Promise<Api> {
    const { token } = await request<{ token: string }>(fetcher, "POST", "/api/login", undefined, { user_id: user });
    return new Api(token, fetcher);
  }

  /** Starts as a guest: a new learner in every open class (ADR 0012). */
  static async guest(fetcher: typeof fetch = apiFetch): Promise<Api> {
    const { token } = await request<{ token: string }>(fetcher, "POST", "/api/guest", undefined, {});
    return new Api(token, fetcher);
  }

  me = () => this.#get<Me>("/api/me");
  studio = (cohort: string, locale: string) => this.#get<StudioView>(`/api/cohorts/${cohort}/studio?locale=${locale}`);
  roster = (cohort: string) => this.#get<Person[]>(`/api/cohorts/${cohort}/roster`);

  /** The lesson pack, from the network when possible, else from the offline copy. */
  async pack(id: string, store: Store): Promise<LessonPack> {
    try {
      const pack = await this.#get<LessonPack>(`/api/packs/${id}`);
      await store.savePack(id, pack);
      return pack;
    } catch (error) {
      const saved = (await store.pack(id)) as LessonPack | undefined;
      if (saved === undefined) throw error;
      return saved;
    }
  }

  commands(cohort: string, items: { actor: Actor; envelope: Envelope }[]) {
    const envelopes = items.map((i) => i.envelope);
    return request<{ id: string; outcome: RunResult }[]>(this.fetcher, "POST", `/api/cohorts/${cohort}/commands`, this.token, envelopes);
  }

  /** Every event this user may see, oldest first, fetched page by page. */
  async events(cohort: string): Promise<Event[]> {
    const all: Event[] = [];
    let cursor = "";
    for (;;) {
      const page = await this.#get<{ seq: number; event: Event }[]>(`/api/cohorts/${cohort}/events?limit=${PAGE}${cursor}`);
      all.push(...page.map((p) => p.event));
      const last = page.at(-1);
      if (page.length < PAGE || last === undefined) return all;
      cursor = `&after_ms=${last.event.at}&after_seq=${last.seq}`;
    }
  }

  #get<T>(path: string): Promise<T> {
    return request<T>(this.fetcher, "GET", path, this.token);
  }
}

async function request<T>(fetcher: typeof fetch, method: string, path: string, token?: string, body?: unknown): Promise<T> {
  const headers = new Headers();
  if (token) headers.set("authorization", `Bearer ${token}`);
  if (body !== undefined) headers.set("content-type", "application/json");
  const response = await fetcher(path, { method, headers, body: body === undefined ? undefined : JSON.stringify(body) });
  const data: unknown = await response.json().catch(() => null);
  if (!response.ok) {
    const message = (data as { error?: string } | null)?.error ?? response.statusText;
    throw new ApiError(response.status, message);
  }
  return data as T;
}
