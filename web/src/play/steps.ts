// What a session on one outcome plays, in the pack's order. A quiz that covers several outcomes
// (the end-of-lesson checkpoint) comes once, in the session of the last outcome it covers, so it
// is never asked before its last outcome is open. Remediation comes only when the kernel assigned it.
import type { Activity } from "../lib/kernel/types/Activity.ts";
import type { LessonPack } from "../lib/kernel/types/LessonPack.ts";

export function stepsFor(pack: LessonPack, outcome: string, assigned: string[]): Activity[] {
  const order = pack.outcomes.map((o) => o.id);
  const last = (a: Activity) => [...a.outcomes].sort((x, y) => order.indexOf(y) - order.indexOf(x))[0];
  return pack.activities.filter((a) => {
    if (!a.outcomes.includes(outcome)) return false;
    if (a.role === "remediation") return assigned.includes(a.id);
    if (a.kind.type === "quiz" && a.outcomes.length > 1) return last(a) === outcome;
    return true;
  });
}
