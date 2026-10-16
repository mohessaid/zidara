import { expect, it } from "vitest";
import { stonePositions } from "../../src/lib/path.ts";

it("lays stones in reading order and mirrors them for Arabic", () => {
  const ltr = stonePositions(3, false);
  expect(ltr.map((p) => p.x)).toEqual([0.14, 0.5, 0.86]);
  expect(ltr.map((p) => Number(p.y.toFixed(2)))).toEqual([0.66, 0.34, 0.66]);
  const rtl = stonePositions(3, true);
  expect(rtl.map((p) => Number(p.x.toFixed(2)))).toEqual([0.86, 0.5, 0.14]);
  expect(stonePositions(1, false)).toEqual([{ x: 0.5, y: 0.66 }]);
  expect(stonePositions(0, true)).toEqual([]);
});
