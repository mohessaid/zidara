import { beforeAll, describe, expect, it } from "vitest";
import { readFileSync } from "node:fs";
import { Kernel, loadKernel, type Setup } from "../../src/lib/kernel/kernel.ts";
import type { Actor } from "../../src/lib/kernel/types/Actor.ts";
import type { Envelope } from "../../src/lib/kernel/types/Envelope.ts";
import { lensView, otherFramework } from "../../src/lib/lens.ts";

const golden = JSON.parse(
  readFileSync(new URL("../../../crates/zidara-wasm/tests/golden/session.json", import.meta.url), "utf8"),
) as { setup: Setup; learner: string; steps: { actor: Actor; envelope: Envelope }[] };

beforeAll(async () => {
  await loadKernel(readFileSync(new URL("../../src/lib/kernel/pkg/zidara_wasm_bg.wasm", import.meta.url)));
});

/** The golden learner's work, without the step where the teacher switched the framework. */
function work() {
  const kernel = new Kernel(golden.setup);
  for (const step of golden.steps) {
    if (step.envelope.command.type === "change_framework") break;
    kernel.run(step.actor, step.envelope);
  }
  return kernel.events();
}

describe("the pedagogy lens", () => {
  it("names the other framework", () => {
    expect(otherFramework("mastery")).toBe("cambridge-style");
    expect(otherFramework("cambridge-style")).toBe("mastery");
  });

  it("judges the same work under another framework, and changes nothing", () => {
    const events = work();
    const at = golden.steps.at(-1)!.envelope.at;
    const mastery = lensView(golden.setup, events, "mastery", golden.learner, "en", at);
    const cambridge = lensView(golden.setup, events, "cambridge-style", golden.learner, "en", at);
    expect(mastery.framework).toBe("mastery");
    expect(cambridge.framework).toBe("cambridge-style");
    // Same evidence: the same attempts on every outcome.
    expect(cambridge.outcomes.map((o) => o.judgement.attempts)).toEqual(mastery.outcomes.map((o) => o.judgement.attempts));
    // A different school's rules: mastery achieves F1 on practice, Cambridge-style never does.
    expect(mastery.outcomes[0]!.judgement.state).toBe("achieved");
    expect(cambridge.outcomes[0]!.judgement.state).not.toBe("achieved");
    // The log it read is untouched.
    expect(events).toEqual(work());
  });

  it("ignores a framework switch already in the log, so the lens always shows the one asked for", () => {
    const kernel = new Kernel(golden.setup);
    for (const step of golden.steps) kernel.run(step.actor, step.envelope);
    const at = golden.steps.at(-1)!.envelope.at;
    expect(lensView(golden.setup, kernel.events(), "mastery", golden.learner, "en", at).framework).toBe("mastery");
  });
});
