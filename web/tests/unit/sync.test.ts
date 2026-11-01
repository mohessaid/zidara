import "fake-indexeddb/auto";
import { describe, expect, it, beforeAll } from "vitest";
import { readFileSync } from "node:fs";
import { Kernel, loadKernel, type Setup } from "../../src/lib/kernel/kernel.ts";
import { openStore } from "../../src/lib/store.ts";
import { act, sync, type SyncApi } from "../../src/lib/sync.ts";
import type { Actor } from "../../src/lib/kernel/types/Actor.ts";
import type { Envelope } from "../../src/lib/kernel/types/Envelope.ts";
import type { Event } from "../../src/lib/kernel/types/Event.ts";
import type { Command } from "../../src/lib/kernel/types/Command.ts";

const golden = JSON.parse(
  readFileSync(new URL("../../../crates/zidara-wasm/tests/golden/session.json", import.meta.url), "utf8"),
) as { setup: Setup; learner: string; steps: { actor: Actor; envelope: Envelope }[] };
const cohort = golden.setup.cohort;
const amina = golden.steps[0]!.actor;
const quiz = golden.steps[0]!.envelope.command;

beforeAll(async () => {
  await loadKernel(readFileSync(new URL("../../src/lib/kernel/pkg/zidara_wasm_bg.wasm", import.meta.url)));
});

/** A server that applies commands with the same kernel, like the real one. */
function fakeServer() {
  const server = new Kernel(golden.setup);
  let online = true;
  const posted: Envelope[][] = [];
  const api: SyncApi = {
    async commands(_cohort, items) {
      if (!online) throw new TypeError("Failed to fetch");
      posted.push(items.map((i) => i.envelope));
      return items.map(({ actor, envelope }) => ({ id: envelope.id, outcome: server.run(actor, envelope) }));
    },
    async events() {
      if (!online) throw new TypeError("Failed to fetch");
      return server.events();
    },
  };
  return { api, server, posted, setOnline: (value: boolean) => (online = value) };
}

let n = 0;
const fresh = () => openStore(`zidara-test-${n++}`);

describe("offline work and sync", () => {
  it("records offline, keeps the outbox across a reload, then syncs once", async () => {
    const { api, server, setOnline, posted } = fakeServer();
    setOnline(false);
    const store = await fresh();
    const kernel = new Kernel(golden.setup);
    const result = await act(store, kernel, cohort, amina, quiz, () => 1_000);
    expect(result.status).toBe("applied");
    expect(kernel.learnerView(golden.learner, "en", 1_000).outcomes[0]!.judgement.state).toBe("achieved");
    await expect(sync(store, api, kernel, cohort)).rejects.toThrow(/Failed to fetch/);
    expect(await store.pending(cohort)).toHaveLength(1);

    // The tab closes; a new one opens the same database and rebuilds the kernel from it.
    store.close();
    const reopened = await openStore(store.name);
    const again = new Kernel(golden.setup);
    again.load(await reopened.log(cohort));
    expect(again.learnerView(golden.learner, "en", 1_000).outcomes[0]!.judgement.state).toBe("achieved");

    setOnline(true);
    const report = await sync(reopened, api, again, cohort);
    expect(report).toEqual({ sent: 1, refused: [], events: server.events().length });
    expect(await reopened.pending(cohort)).toEqual([]);
    expect(await reopened.log(cohort)).toEqual(server.events());
    // Syncing again sends nothing new.
    await sync(reopened, api, again, cohort);
    expect(posted).toHaveLength(1);
  });

  it("does not record what the kernel refuses", async () => {
    const store = await fresh();
    const kernel = new Kernel(golden.setup);
    const forbidden: Command = { type: "change_framework", framework: "cambridge-style" };
    const result = await act(store, kernel, cohort, amina, forbidden, () => 1);
    expect(result).toMatchObject({ status: "refused", error: { type: "forbidden" } });
    expect(await store.pending(cohort)).toEqual([]);
  });

  it("keeps commands the server refuses, with the reason, and stops resending them", async () => {
    const { api, server } = fakeServer();
    const store = await fresh();
    const kernel = new Kernel(golden.setup);
    await act(store, kernel, cohort, amina, quiz, () => 1);
    // The server knows something the phone did not: this learner is now someone else's.
    const stranger: Actor = { kind: "human", user: "00000000-0000-0000-0000-000000000bad", role: "teacher" };
    await store.enqueue(cohort, stranger, { id: crypto.randomUUID(), at: 2, command: { type: "change_framework", framework: "nope" } });
    const report = await sync(store, api, kernel, cohort);
    expect(report.sent).toBe(2);
    expect(report.refused).toHaveLength(1);
    expect(report.refused[0]!.error).toEqual({ type: "unknown_framework", framework: "nope" });
    expect(await store.pending(cohort)).toEqual([]);
    expect(await store.refused(cohort)).toHaveLength(1);
    expect(server.events().length).toBe(2);
  });

  it("drops a live answer that came too late, quietly, and follows the board", async () => {
    const { api, server } = fakeServer();
    const store = await fresh();
    const kernel = new Kernel(golden.setup);
    const teacher: Actor = { kind: "human", user: "00000000-0000-0000-0000-0000007ea001", role: "teacher" };
    const env = (at: number, command: Command): Envelope => ({ id: crypto.randomUUID(), at, command });
    server.run(teacher, env(1, { type: "start_challenge", activity: "quiz-f1" }));
    await sync(store, api, kernel, cohort);
    // The phone answers question 1 while the teacher has already moved to question 2.
    await act(store, kernel, cohort, amina, { type: "answer_challenge", question: 0, response: { type: "choice", index: 0 } }, () => 2);
    server.run(teacher, env(3, { type: "next_question" }));
    const report = await sync(store, api, kernel, cohort);
    // Missing a question in a live round is not an error to show a child: nothing is kept.
    expect(report.refused).toEqual([]);
    expect(await store.refused(cohort)).toEqual([]);
    expect(await store.pending(cohort)).toEqual([]);
    expect(kernel.learnerView(golden.learner, "en", 4).challenge).toMatchObject({ question: 1, answered: null });
  });

  it("keeps cohorts apart", async () => {
    const store = await fresh();
    const kernel = new Kernel(golden.setup);
    await act(store, kernel, cohort, amina, quiz, () => 1);
    expect(await store.pending("00000000-0000-0000-0000-00000000c00b")).toEqual([]);
    expect(await store.log("00000000-0000-0000-0000-00000000c00b")).toEqual([] as Event[]);
  });

  it("remembers the session", async () => {
    const store = await fresh();
    expect(await store.session()).toBeUndefined();
    const session = { token: "t", user: golden.learner, displayName: "Amina", cohorts: [] };
    await store.saveSession(session);
    expect(await store.session()).toEqual(session);
    await store.clearSession();
    expect(await store.session()).toBeUndefined();
  });
});
