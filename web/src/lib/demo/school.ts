// The demo school, in the browser: the server's API for the demo build, so the prototype can be
// tried from a static link. The rules are the kernel's (the same wasm as the phone's), as on the
// server: commands run as the signed-in person, each person reads what the server's database
// would show them, and the rubric marks open answers as the assessment agent. What it cannot
// show is the server itself: the database, row-level security, many devices, a model marking.
import { Kernel } from "../kernel/kernel.ts";
import type { Actor } from "../kernel/types/Actor.ts";
import type { Envelope } from "../kernel/types/Envelope.ts";
import type { Event } from "../kernel/types/Event.ts";
import type { LessonPack } from "../kernel/types/LessonPack.ts";
import type { Role } from "../kernel/types/Role.ts";

const TENANT = "00000000-0000-0000-0000-000007e40001";
const CLASSES = [
  { id: "00000000-0000-0000-0000-00000000c00a", name: "Class 4 A", lesson: "equivalent-fractions" },
  { id: "00000000-0000-0000-0000-00000000c00b", name: "Class 4 B", lesson: "equivalent-fractions" },
  { id: "00000000-0000-0000-0000-00000000c00c", name: "Discover Zidara", lesson: "how-zidara-works" },
] as const;
const [A, B, C] = [CLASSES[0].id, CLASSES[1].id, CLASSES[2].id];

/** The server's seed (`crates/zidara-server/src/seed.rs`). */
export const PEOPLE = [
  { name: "amina", id: "00000000-0000-0000-0000-0000001ea001", display_name: "Amina", cohort: A, role: "learner" },
  { name: "yacine", id: "00000000-0000-0000-0000-0000001ea002", display_name: "Yacine", cohort: A, role: "learner" },
  { name: "sara", id: "00000000-0000-0000-0000-0000001eb001", display_name: "Sara", cohort: B, role: "learner" },
  { name: "karim", id: "00000000-0000-0000-0000-0000007ea001", display_name: "Karim", cohort: A, role: "teacher" },
  { name: "nadia", id: "00000000-0000-0000-0000-0000007eb001", display_name: "Nadia", cohort: B, role: "teacher" },
  { name: "lina", id: "00000000-0000-0000-0000-0000001ec001", display_name: "Lina", cohort: C, role: "learner" },
  { name: "omar", id: "00000000-0000-0000-0000-0000007ec001", display_name: "Omar", cohort: C, role: "teacher" },
] as const satisfies readonly { name: string; id: string; display_name: string; cohort: string; role: Role }[];

const AGENT: Actor = { kind: "agent", agent: "assessor", role: "assessment_agent" };
const KEY = "zidara.demo-school";

type Saved = Record<string, { log: Event[]; applied: string[] }>;
type Storage = { getItem(key: string): string | null; setItem(key: string, value: string): void };

class Refusal {
  constructor(
    readonly status: number,
    readonly error: string,
  ) {}
}
const unauthorized = () => new Refusal(401, "sign in first");
const notMember = () => new Refusal(403, "not a member of this cohort");
const forbidden = () => new Refusal(403, "your role cannot see this");
const notFound = () => new Refusal(404, "not found");

export class DemoSchool {
  #saved: Saved;
  readonly #packs: LessonPack[];
  #seen: string | null;
  /** One kernel per class for the page's life: each holds the lesson and log in wasm memory. */
  readonly #kernels = new Map<string, Kernel>();

  constructor(
    packs: LessonPack | LessonPack[],
    private readonly storage: Storage,
    private readonly now: () => number = Date.now,
  ) {
    this.#packs = Array.isArray(packs) ? packs : [packs];
    this.#seen = storage.getItem(KEY);
    this.#saved = JSON.parse(this.#seen ?? "{}") as Saved;
  }

  /** Another tab (a teacher's board, a learner's phone) may have changed the school since:
   *  read it again, so two tabs play one live round together. */
  #refresh() {
    const now = this.storage.getItem(KEY);
    if (now === this.#seen) return;
    this.#seen = now;
    this.#saved = JSON.parse(now ?? "{}") as Saved;
    this.#kernels.clear();
  }

  /** A `fetch` for the app's `Api`: same paths, same JSON, same status codes as the server. */
  fetch: typeof fetch = async (input, init) => {
    const url = new URL(String(input), "http://demo");
    const body: unknown = typeof init?.body === "string" ? JSON.parse(init.body) : undefined;
    const token = new Headers(init?.headers).get("authorization")?.replace(/^Bearer /, "");
    try {
      this.#refresh();
      return json(this.#route(init?.method ?? "GET", url, token, body));
    } catch (error) {
      if (error instanceof Refusal) return json({ error: error.error }, error.status);
      throw error;
    }
  };

  #route(method: string, url: URL, token: string | undefined, body: unknown): unknown {
    const path = url.pathname;
    if (method === "POST" && path === "/api/login") {
      const person = PEOPLE.find((p) => p.id === (body as { user_id?: string }).user_id);
      if (!person) throw unauthorized();
      return { token: person.id };
    }
    const person = PEOPLE.find((p) => p.id === token);
    if (!person) throw unauthorized();
    if (method === "GET" && path === "/api/me") {
      const cohort = CLASSES.find((c) => c.id === person.cohort)!;
      return {
        id: person.id,
        display_name: person.display_name,
        cohorts: [{ id: cohort.id, tenant_id: TENANT, name: cohort.name, lesson_id: cohort.lesson, framework: "mastery", role: person.role }],
      };
    }
    const pack = this.#packs.find((p) => path === `/api/packs/${p.id}`);
    if (method === "GET" && pack) return pack;
    const [, cohort, action] = /^\/api\/cohorts\/([^/]+)\/([a-z]+)$/.exec(path) ?? [];
    if (!cohort || !CLASSES.some((c) => c.id === cohort)) throw notFound();
    if (person.cohort !== cohort) throw notMember();
    const actor: Actor = { kind: "human", user: person.id, role: person.role };
    const kernel = this.#kernel(cohort);
    if (method === "POST" && action === "commands") return this.#commands(cohort, kernel, actor, body as Envelope[]);
    if (method === "GET" && action === "events") {
      const after = Number(url.searchParams.get("after_seq") ?? -1);
      const limit = Number(url.searchParams.get("limit") ?? 500);
      const seq = new Map(kernel.events().map((e, i) => [e.id, i]));
      return kernel
        .visibleEvents(person.id, person.role)
        .map((event) => ({ seq: seq.get(event.id)!, event }))
        .filter((row) => row.seq > after)
        .slice(0, limit);
    }
    if (person.role !== "teacher") throw forbidden();
    const learners = PEOPLE.filter((p) => p.cohort === cohort && p.role === "learner").sort((a, b) => a.display_name.localeCompare(b.display_name));
    if (method === "GET" && action === "studio") {
      return kernel.studioView(learners.map((p) => p.id), url.searchParams.get("locale") ?? "ar", this.now());
    }
    if (method === "GET" && action === "roster") return learners.map((p) => ({ id: p.id, display_name: p.display_name }));
    throw notFound();
  }

  #commands(cohort: string, kernel: Kernel, actor: Actor, envelopes: Envelope[]) {
    const saved = this.#cohort(cohort);
    const results = envelopes.map((envelope) => {
      if (saved.applied.includes(envelope.id)) return { id: envelope.id, outcome: { status: "duplicate" } };
      // As on the server: when the person acted, no later than now and no earlier than their
      // latest event.
      const latest = Math.max(...saved.log.filter((e) => sameActor(e.actor, actor)).map((e) => e.at));
      const at = Math.min(Math.max(envelope.at, latest), this.now());
      const outcome = kernel.run(actor, { ...envelope, at });
      if (outcome.status === "applied") {
        saved.applied.push(envelope.id);
        saved.log = kernel.events();
      }
      return { id: envelope.id, outcome };
    });
    // The assessment agent marks new open answers, with the rubric (no model in a browser).
    for (const { answer, assessment } of kernel.rubricMarks()) {
      kernel.run(AGENT, { id: crypto.randomUUID(), at: this.now(), command: { type: "propose_assessment", answer, assessment } });
    }
    saved.log = kernel.events();
    this.#seen = JSON.stringify(this.#saved);
    this.storage.setItem(KEY, this.#seen);
    return results;
  }

  #cohort(cohort: string) {
    return (this.#saved[cohort] ??= { log: [], applied: [] });
  }

  #kernel(cohort: string): Kernel {
    let kernel = this.#kernels.get(cohort);
    if (!kernel) {
      const lesson = CLASSES.find((c) => c.id === cohort)!.lesson;
      kernel = new Kernel({ lesson: this.#packs.find((p) => p.id === lesson)!, framework: "mastery", tenant: TENANT, cohort });
      kernel.load(this.#cohort(cohort).log);
      this.#kernels.set(cohort, kernel);
    }
    return kernel;
  }
}

function sameActor(a: Actor, b: Actor): boolean {
  return JSON.stringify(a) === JSON.stringify(b);
}

function json(body: unknown, status = 200): Response {
  return new Response(JSON.stringify(body), { status, headers: { "content-type": "application/json" } });
}
