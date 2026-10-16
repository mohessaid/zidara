import { describe, expect, it, beforeAll } from "vitest";
import { readFileSync } from "node:fs";
import { Kernel, loadKernel, type Setup } from "../../src/lib/kernel/kernel.ts";
import type { Actor } from "../../src/lib/kernel/types/Actor.ts";
import type { Envelope } from "../../src/lib/kernel/types/Envelope.ts";

const golden = JSON.parse(
  readFileSync(new URL("../../../crates/zidara-wasm/tests/golden/session.json", import.meta.url), "utf8"),
) as { setup: Setup; learner: string; steps: { actor: Actor; envelope: Envelope }[]; views_ar: unknown[] };

beforeAll(async () => {
  await loadKernel(readFileSync(new URL("../../src/lib/kernel/pkg/zidara_wasm_bg.wasm", import.meta.url)));
});

describe("Kernel", () => {
  it("runs the golden session with typed results", () => {
    const kernel = new Kernel(golden.setup);
    golden.steps.forEach((step, i) => {
      const result = kernel.run(step.actor, step.envelope);
      expect(result.status).toBe("applied");
      expect(kernel.learnerView(golden.learner, "ar", step.envelope.at)).toEqual(golden.views_ar[i]);
    });
    expect(kernel.events().length).toBeGreaterThan(golden.steps.length);
  });

  it("reports duplicates and refusals as results", () => {
    const kernel = new Kernel(golden.setup);
    const [first] = golden.steps;
    kernel.run(first!.actor, first!.envelope);
    expect(kernel.run(first!.actor, first!.envelope)).toEqual({ status: "duplicate" });
    const switchFramework: Envelope = { id: crypto.randomUUID(), at: 0, command: { type: "change_framework", framework: "cambridge-style" } };
    const refused = kernel.run(first!.actor, switchFramework);
    expect(refused).toEqual({ status: "refused", error: { type: "forbidden", role: "learner", permission: "change_framework" } });
  });

  it("loads a log and draws the studio", () => {
    const kernel = new Kernel(golden.setup);
    for (const step of golden.steps) kernel.run(step.actor, step.envelope);
    const copy = new Kernel(golden.setup);
    copy.load(kernel.events());
    expect(copy.events()).toEqual(kernel.events());
    const studio = copy.studioView([golden.learner], "en", golden.steps.at(-1)!.envelope.at);
    // The last step switched to Cambridge-style, where practice shows a band but never achieves.
    expect(studio.framework).toBe("cambridge-style");
    expect(studio.outcomes[0]).toMatchObject({ outcome: "F1", achieved: 0, of: 1 });
    expect(studio.flags[0]!.reason).toBe("struggling");
  });

  it("throws readable errors", () => {
    expect(() => new Kernel({ ...golden.setup, framework: "ib" })).toThrow(/unknown framework/);
  });
});
