import { expect, it } from "vitest";
import { startsLight } from "../../src/lib/device.ts";

it("data saver or 2 GB of memory or less starts the light experience", () => {
  expect(startsLight({ connection: { saveData: true }, deviceMemory: 8 })).toBe(true);
  expect(startsLight({ deviceMemory: 2 })).toBe(true);
  expect(startsLight({ deviceMemory: 1 })).toBe(true);
});

it("an economy phone, or a browser that says nothing, gets the map", () => {
  expect(startsLight({ connection: { saveData: false }, deviceMemory: 4 })).toBe(false);
  expect(startsLight({})).toBe(false);
});
