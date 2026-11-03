import { beforeAll, describe, expect, it } from "vitest";
import { readFileSync } from "node:fs";
import { loadKernel } from "../../src/lib/kernel/kernel.ts";
import type { LessonPack } from "../../src/lib/kernel/types/LessonPack.ts";
import { showcase } from "../../src/lib/showcase.ts";

const fractions = JSON.parse(readFileSync(new URL("../../../content/lessons/equivalent-fractions.json", import.meta.url), "utf8")) as LessonPack;

beforeAll(async () => {
  await loadKernel(readFileSync(new URL("../../src/lib/kernel/pkg/zidara_wasm_bg.wasm", import.meta.url)));
});

describe("the landing page's showcase", () => {
  it("judges one learner's same first quiz under each school's rules, with the real kernel", () => {
    const views = showcase(fractions, "en");
    const [m, c] = [views.mastery, views["cambridge-style"]];
    expect(m.framework).toBe("mastery");
    expect(c.framework).toBe("cambridge-style");
    expect(m.outcomes.map((o) => o.outcome)).toEqual(["F1", "F2", "F3"]);
    // The same evidence: one attempt at F1, nothing else.
    expect(c.outcomes.map((o) => o.judgement.attempts)).toEqual(m.outcomes.map((o) => o.judgement.attempts));
    expect(m.outcomes[0]!.judgement.attempts).toBe(1);
    // Mastery: F1 is achieved and F2 opens. Cambridge-style: a band, not mastery.
    expect(m.outcomes[0]!.judgement.state).toBe("achieved");
    expect(m.outcomes[1]!.lock).toBeNull();
    expect(c.outcomes[0]!.judgement.state).not.toBe("achieved");
    expect(c.outcomes[0]!.judgement.band).not.toBeNull();
  });

  it("speaks the visitor's language", () => {
    expect(showcase(fractions, "ar").mastery.outcomes[0]!.title).not.toBe(showcase(fractions, "en").mastery.outcomes[0]!.title);
  });
});
