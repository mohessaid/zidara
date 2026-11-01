// Acting offline and syncing. The phone runs every command through its own kernel first, so
// the learner sees the result at once; the outbox carries the command to the server, which
// replays it through the same kernel. After a sync the server's log replaces the phone's.
import type { Kernel, RunResult } from "./kernel/kernel.ts";
import type { Actor } from "./kernel/types/Actor.ts";
import type { Command } from "./kernel/types/Command.ts";
import type { Envelope } from "./kernel/types/Envelope.ts";
import type { Event } from "./kernel/types/Event.ts";
import type { KernelError } from "./kernel/types/KernelError.ts";
import type { Outgoing, Store } from "./store.ts";

export interface SyncApi {
  commands(cohort: string, items: { actor: Actor; envelope: Envelope }[]): Promise<{ id: string; outcome: RunResult }[]>;
  events(cohort: string): Promise<Event[]>;
}

export type SyncReport = { sent: number; refused: { item: Outgoing; error: KernelError }[]; events: number; changed: boolean };

/** A live answer that reached the server after the teacher moved on (or ended the round): in a
 *  round led by the teacher this is normal, not an error to show a child. */
const MISSED: KernelError["type"][] = ["not_this_question", "already_answered", "no_challenge"];

/** Runs a command locally; when the kernel accepts it, it is saved and queued for the server. */
export async function act(
  store: Store,
  kernel: Kernel,
  cohort: string,
  actor: Actor,
  command: Command,
  now: () => number = Date.now,
): Promise<RunResult> {
  const envelope: Envelope = { id: crypto.randomUUID(), at: now(), command };
  const result = kernel.run(actor, envelope);
  if (result.status === "applied") {
    await store.enqueue(cohort, actor, envelope);
    await store.setLog(cohort, kernel.events());
  }
  return result;
}

/** Sends the outbox, then adopts the server's log. Throws when offline; nothing is lost. */
export async function sync(store: Store, api: SyncApi, kernel: Kernel, cohort: string): Promise<SyncReport> {
  const pending = await store.pending(cohort);
  const refused: SyncReport["refused"] = [];
  if (pending.length > 0) {
    const results = await api.commands(cohort, pending.map(({ actor, envelope }) => ({ actor, envelope })));
    for (const item of pending) {
      const outcome = results.find((r) => r.id === item.envelope.id)?.outcome;
      if (outcome?.status === "refused" && item.envelope.command.type === "answer_challenge" && MISSED.includes(outcome.error.type)) {
        await store.remove(item.id);
      } else if (outcome?.status === "refused") {
        await store.markRefused(item, outcome.error);
        refused.push({ item, error: outcome.error });
      } else if (outcome) {
        await store.remove(item.id);
      }
    }
  }
  const events = await api.events(cohort);
  // When the server's log is the phone's, an idle poll reloads nothing and redraws nothing (polls
  // run every few seconds). Whole logs are compared: the server may have adjusted an event's time.
  if (JSON.stringify(events) === JSON.stringify(kernel.events())) {
    return { sent: pending.length, refused, events: events.length, changed: false };
  }
  await store.setLog(cohort, events);
  kernel.load(events);
  return { sent: pending.length, refused, events: events.length, changed: true };
}
