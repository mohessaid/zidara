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

/** Under Cambridge-style, the scheme of work opens F3 in the third week of term (19 October
 * 2026). Browser tests use the real clock, so they state the rule for any date. */
export const F3_OPENS = Date.UTC(2026, 9, 19);

export const stone = (page: Page, outcome: string) => page.locator(`[data-outcome="${outcome}"]`);

/** The right answers to each version of quiz-f1: the option to pick, and the fraction to type. */
const F1_ANSWERS = [
  { choice: 0, num: "3", den: "8" },
  { choice: 1, num: "2", den: "5" },
  { choice: 2, num: "4", den: "7" },
] as const;

/** Opens quiz-f1 from the F1 panel and answers whichever version the kernel shows, correctly. */
export async function passQuizF1(page: Page) {
  await page.getByRole("button", { name: /تمرين: أجزاء من كلّ/ }).click();
  const variant = Number(await page.locator("form[data-variant]").getAttribute("data-variant"));
  const answer = F1_ANSWERS[variant]!;
  await page.getByRole("radio").nth(answer.choice).check();
  await page.getByRole("spinbutton", { name: "البسط" }).fill(answer.num);
  await page.getByRole("spinbutton", { name: "المقام" }).fill(answer.den);
  await page.getByRole("button", { name: "تحقّق من إجاباتي" }).click();
  return variant;
}
