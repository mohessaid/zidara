// The landing page's live proof (ADR 0014): one learner answers the first fractions quiz once,
// and the real kernel, compiled to WebAssembly and running in the visitor's browser, judges that
// same work under each school's rules. Nothing is sent or saved; the ids and times are fixed so
// every visitor sees the same story.
import { Kernel, type Setup } from "./kernel/kernel.ts";
import type { FrameworkId } from "./kernel/types/FrameworkId.ts";
import type { LearnerView } from "./kernel/types/LearnerView.ts";
import type { LessonPack } from "./kernel/types/LessonPack.ts";
import { FRAMEWORKS, lensView } from "./lens.ts";

const LEARNER = "00000000-0000-0000-0000-0000005ca5e1";
/** Monday 12 October 2026, 09:00 UTC: early in term, before the scheme of work opens F3. */
const AT = Date.UTC(2026, 9, 12, 9);

export type Showcase = Record<(typeof FRAMEWORKS)[number], LearnerView>;

export function showcase(lesson: LessonPack, locale: string): Showcase {
  const setup: Setup = { lesson, framework: "mastery", tenant: "00000000-0000-0000-0000-0000005ca5e0", cohort: "00000000-0000-0000-0000-0000005ca5ec" };
  const kernel = new Kernel(setup);
  const result = kernel.run(
    { kind: "human", role: "learner", user: LEARNER },
    {
      id: "0193b000-0000-7000-8000-0000005ca5e1",
      at: AT,
      command: { type: "submit_quiz", activity: "quiz-f1", variant: 0, responses: [{ type: "choice", index: 0 }, { type: "fraction", num: 3, den: 8 }] },
    },
  );
  if (result.status !== "applied") throw new Error(`showcase: ${JSON.stringify(result)}`);
  const events = kernel.events();
  const view = (framework: FrameworkId) => lensView(setup, events, framework, LEARNER, locale, AT + 60_000);
  return { mastery: view("mastery"), "cambridge-style": view("cambridge-style") };
}
