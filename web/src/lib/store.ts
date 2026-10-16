// What the phone keeps between visits: who is signed in, each cohort's log, and the outbox of
// commands not yet accepted by the server. IndexedDB, so it survives closing the tab.
import { openDB, type DBSchema, type IDBPDatabase } from "idb";
import type { Actor } from "./kernel/types/Actor.ts";
import type { Envelope } from "./kernel/types/Envelope.ts";
import type { Event } from "./kernel/types/Event.ts";
import type { KernelError } from "./kernel/types/KernelError.ts";

export type CohortMembership = { id: string; name: string; lesson_id: string; role: "learner" | "teacher" | "assessment_agent" };
export type Session = { token: string; user: string; displayName: string; cohorts: CohortMembership[] };
export type Outgoing = { id: string; cohort: string; actor: Actor; envelope: Envelope; refused?: KernelError };

interface Schema extends DBSchema {
  session: { key: string; value: Session };
  logs: { key: string; value: Event[] };
  outbox: { key: string; value: Outgoing; indexes: { cohort: string } };
  packs: { key: string; value: unknown };
}

export type Store = Awaited<ReturnType<typeof openStore>>;

export async function openStore(name = "zidara") {
  const db: IDBPDatabase<Schema> = await openDB<Schema>(name, 1, {
    upgrade(db) {
      db.createObjectStore("session");
      db.createObjectStore("logs");
      db.createObjectStore("outbox", { keyPath: "id" }).createIndex("cohort", "cohort");
      db.createObjectStore("packs");
    },
  });
  const inCohort = (cohort: string) => db.getAllFromIndex("outbox", "cohort", cohort);
  return {
    name,
    close: () => db.close(),
    session: () => db.get("session", "current"),
    saveSession: async (session: Session) => void (await db.put("session", session, "current")),
    clearSession: () => db.delete("session", "current"),
    log: async (cohort: string) => (await db.get("logs", cohort)) ?? [],
    setLog: async (cohort: string, events: Event[]) => void (await db.put("logs", events, cohort)),
    enqueue: async (cohort: string, actor: Actor, envelope: Envelope) =>
      void (await db.put("outbox", { id: envelope.id, cohort, actor, envelope })),
    /** Commands to send, oldest first: the order the learner acted in. */
    pending: async (cohort: string) =>
      (await inCohort(cohort)).filter((o) => !o.refused).sort((a, b) => a.envelope.at - b.envelope.at),
    refused: async (cohort: string) => (await inCohort(cohort)).filter((o) => o.refused),
    pack: (id: string) => db.get("packs", id),
    savePack: async (id: string, pack: unknown) => void (await db.put("packs", pack, id)),
    remove: (id: string) => db.delete("outbox", id),
    markRefused: async (item: Outgoing, error: KernelError) => void (await db.put("outbox", { ...item, refused: error })),
  };
}
