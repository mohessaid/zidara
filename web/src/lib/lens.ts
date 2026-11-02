// The pedagogy lens (ADR 0013): the learner's own evidence, judged by another school's rules.
// A throwaway kernel replays the same log under the framework asked for; nothing is run, saved
// or sent, and the class's real framework stays the teacher's choice.
import { Kernel, type Setup } from "./kernel/kernel.ts";
import type { Event } from "./kernel/types/Event.ts";
import type { FrameworkId } from "./kernel/types/FrameworkId.ts";
import type { LearnerView } from "./kernel/types/LearnerView.ts";

export const FRAMEWORKS = ["mastery", "cambridge-style"] as const;

export function otherFramework(framework: FrameworkId): FrameworkId {
  return framework === "mastery" ? "cambridge-style" : "mastery";
}

export function lensView(setup: Setup, events: Event[], framework: FrameworkId, user: string, locale: string, now: number): LearnerView {
  const kernel = new Kernel({ ...setup, framework });
  // A switch already in the log would override the lens; the lens asks "what if", so it goes.
  kernel.load(events.filter((e) => e.body.type !== "framework_changed"));
  return kernel.learnerView(user, locale, now);
}
