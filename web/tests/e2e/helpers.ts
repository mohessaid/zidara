import { expect, type Page } from "@playwright/test";
import AxeBuilder from "@axe-core/playwright";

export const PEOPLE = { amina: "أمينة", yacine: "ياسين", sara: "سارة", karim: "كريم" } as const;

export async function signIn(page: Page, who: keyof typeof PEOPLE) {
  await page.goto("/");
  await page.getByRole("button", { name: new RegExp(`الدخول باسم ${PEOPLE[who]}`) }).click();
}

/** Zero WCAG 2.2 A/AA violations on what is on screen. */
export async function expectAccessible(page: Page, screen: string) {
  const results = await new AxeBuilder({ page }).withTags(["wcag2a", "wcag2aa", "wcag21a", "wcag21aa", "wcag22aa"]).analyze();
  expect(results.violations.map((v) => `${v.id}: ${v.nodes.map((n) => n.target.join(" ")).join(", ")}`), screen).toEqual([]);
}

export const stone = (page: Page, outcome: string) => page.locator(`[data-outcome="${outcome}"]`);
