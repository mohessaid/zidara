import { expect, it } from "vitest";
import { stars } from "../../src/lib/stars.ts";

it("stars come from the band, never from effort alone", () => {
  expect(stars({ state: "not_started", score: null, band: null, attempts: 0 })).toEqual({ n: 0, of: 0 });
  expect(stars({ state: "in_progress", score: 3000, band: { label: "mastery.not_yet", rank: 0, of: 3 }, attempts: 1 })).toEqual({ n: 0, of: 2 });
  expect(stars({ state: "achieved", score: 9000, band: { label: "mastery.mastered", rank: 2, of: 3 }, attempts: 2 })).toEqual({ n: 2, of: 2 });
  expect(stars({ state: "achieved", score: 9000, band: { label: "cambridge.excelling", rank: 3, of: 4 }, attempts: 1 })).toEqual({ n: 3, of: 3 });
});
