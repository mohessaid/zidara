import { readFileSync } from "node:fs";
import { describe, expect, it } from "vitest";
import { LANDING } from "../../src/lib/i18n/landing.ts";

/** Every string in a value, with its path. */
function strings(value: unknown, at = ""): [string, string][] {
  if (typeof value === "string") return [[at, value]];
  if (Array.isArray(value)) return value.flatMap((v, i) => strings(v, `${at}[${i}]`));
  if (value && typeof value === "object") return Object.entries(value).flatMap(([k, v]) => strings(v, `${at}.${k}`));
  return [];
}
const shape = (value: unknown) => strings(value).map(([at]) => at);

describe("the landing page's words", () => {
  it("say the same things in Arabic, French and English", () => {
    expect(shape(LANDING.ar)).toEqual(shape(LANDING.en));
    expect(shape(LANDING.fr)).toEqual(shape(LANDING.en));
    for (const locale of ["ar", "fr", "en"] as const) for (const [at, s] of strings(LANDING[locale])) expect(s.trim(), `${locale}${at}`).not.toBe("");
  });

  it("keep the same figures and placeholders in every language", () => {
    const tokens = (s: string) => [...s.matchAll(/\{\w+\}|\d+/g)].map((m) => m[0]).sort();
    const en = Object.fromEntries(strings(LANDING.en));
    for (const locale of ["ar", "fr"] as const) {
      for (const [at, s] of strings(LANDING[locale])) {
        if (at.startsWith(".sources") || at.includes(".src")) continue;
        expect(tokens(s), `${locale}${at}`).toEqual(tokens(en[at]!));
      }
    }
  });

  it("use no em or en dashes (house style for this page)", () => {
    for (const locale of ["ar", "fr", "en"] as const) for (const [at, s] of strings(LANDING[locale])) expect(s, `${locale}${at}`).not.toMatch(/[–—]/);
  });

  it("cite only sources and screens that exist, and link only to the app's own", () => {
    const n = LANDING.en.sources.length;
    for (const step of LANDING.en.why.steps) for (const c of step.cite) expect(c).toBeGreaterThanOrEqual(1), expect(c).toBeLessThanOrEqual(n);
    for (const locale of ["ar", "fr", "en"] as const) for (const item of LANDING[locale].made.items) expect(["#play", "#classes"]).toContain(item.href);
    for (const item of LANDING.en.made.items) expect(() => readFileSync(new URL(`../../public${item.shot}`, import.meta.url))).not.toThrow();
  });

  it("pitch the engine, not a game (ADR 0015)", () => {
    const en = LANDING.en;
    expect(`${en.hero.before}${en.hero.accent}`).toMatch(/learning engine/);
    expect(en.hero.lead).toMatch(/any teaching framework/);
    expect(en.hero.lead).toMatch(/any experience/);
    expect(en.faq.items[0]!.q).toBe("Is Zidara a game?");
    expect(en.faq.items[0]!.a).toMatch(/^No\./);
  });
});
