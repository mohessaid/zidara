// The kernel, typed. The wasm Shell speaks JSON strings; this wrapper speaks the generated
// kernel types, so the app can never drift from the Rust contract.
import init, { Shell } from "./pkg/zidara_wasm.js";
import type { Actor } from "./types/Actor.ts";
import type { Assessment } from "./types/Assessment.ts";
import type { CohortId } from "./types/CohortId.ts";
import type { Envelope } from "./types/Envelope.ts";
import type { Event } from "./types/Event.ts";
import type { FrameworkId } from "./types/FrameworkId.ts";
import type { KernelError } from "./types/KernelError.ts";
import type { LearnerView } from "./types/LearnerView.ts";
import type { LessonPack } from "./types/LessonPack.ts";
import type { Role } from "./types/Role.ts";
import type { StudioView } from "./types/StudioView.ts";
import type { TenantId } from "./types/TenantId.ts";
import type { UserId } from "./types/UserId.ts";

export type Setup = { lesson: LessonPack; framework: FrameworkId; tenant: TenantId; cohort: CohortId };

export type RunResult =
  | { status: "applied"; events: Event[] }
  | { status: "duplicate" }
  | { status: "refused"; error: KernelError };

/** Loads the wasm once: a URL in the browser, bytes in tests. */
export async function loadKernel(source?: BufferSource | URL | string): Promise<void> {
  await init(source === undefined ? undefined : { module_or_path: source });
}

export class Kernel {
  readonly #shell: Shell;

  constructor(setup: Setup) {
    this.#shell = new Shell(JSON.stringify(setup));
  }

  run(actor: Actor, envelope: Envelope): RunResult {
    return JSON.parse(this.#shell.run(JSON.stringify(actor), JSON.stringify(envelope))) as RunResult;
  }

  /** The version of a quiz due for `user`: show it, and send its number with the answers. */
  nextVariant(user: UserId, activity: string): number {
    return this.#shell.nextVariant(user, activity);
  }

  learnerView(user: UserId, locale: string, now: number): LearnerView {
    return JSON.parse(this.#shell.learnerView(user, locale, now)) as LearnerView;
  }

  studioView(roster: UserId[], locale: string, now: number): StudioView {
    return JSON.parse(this.#shell.studioView(JSON.stringify(roster), locale, now)) as StudioView;
  }

  /** The log as `user` in `role` may read it: what the server's database would return. */
  visibleEvents(user: UserId, role: Role): Event[] {
    return JSON.parse(this.#shell.visibleEvents(user, role)) as Event[];
  }

  /** The rubric's mark for each open answer that has no proposal yet. */
  rubricMarks(): { answer: string; assessment: Assessment }[] {
    return JSON.parse(this.#shell.rubricMarks()) as { answer: string; assessment: Assessment }[];
  }

  events(): Event[] {
    return JSON.parse(this.#shell.events()) as Event[];
  }

  load(events: Event[]): void {
    this.#shell.load(JSON.stringify(events));
  }
}
