// A question's look with the kernel's defaults filled in. The kernel reads packs with serde
// defaults (no look means a plain question answered on a keypad); the browser reads the same
// JSON as it is, so it applies the same defaults here.
import type { Input } from "../lib/kernel/types/Input.ts";
import type { Localized } from "../lib/kernel/types/Localized.ts";
import type { Question } from "../lib/kernel/types/Question.ts";
import type { Visual } from "../lib/kernel/types/Visual.ts";

export type FullLook = { visual: Visual | null; input: Input; explain: Localized | null };

export function lookOf(question: Question): FullLook {
  const look = (question as { look?: Partial<FullLook> }).look;
  return { visual: look?.visual ?? null, input: look?.input ?? { type: "keypad" }, explain: look?.explain ?? null };
}
