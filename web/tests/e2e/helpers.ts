import { expect, type Page } from "@playwright/test";
import AxeBuilder from "@axe-core/playwright";

export const PEOPLE = { amina: "أمينة", yacine: "ياسين", sara: "سارة", karim: "كريم" } as const;

/** Signs in one of the demo classes' people (#classes); the front door is for guests. */
export async function signIn(page: Page, who: keyof typeof PEOPLE) {
  await page.goto("/#classes");
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

export const button = (page: Page, name: string) => page.getByRole("button", { name, exact: true });

/** Checks the answer on screen, then moves on. */
async function checkAndGo(page: Page) {
  await button(page, "تحقّق").click();
  await button(page, "تابع").click();
}

/** Plays the F1 session from its stone: the film, the reading, then whichever version of
 * quiz-f1 the kernel shows, answered right. Ends on the finish screen. Returns the version. */
export async function playF1(page: Page) {
  await stone(page, "F1").click();
  await button(page, "تابع").click();
  await button(page, "تابع").click();
  const variant = Number(await page.locator("[data-variant]").getAttribute("data-variant"));
  const answer = F1_ANSWERS[variant]!;
  await page.getByRole("radio").nth(answer.choice).click();
  await checkAndGo(page);
  await page.getByRole("spinbutton", { name: "البسط" }).fill(answer.num);
  await page.getByRole("spinbutton", { name: "المقام" }).fill(answer.den);
  await checkAndGo(page);
  await expect(page.getByRole("heading", { name: "أصبحت تستطيع هذا!" })).toBeVisible();
  return variant;
}

/** Starts as a guest on the course `lesson`, from the front door. */
export async function startAsGuest(page: Page, lesson = "equivalent-fractions") {
  await page.goto("/");
  await page.locator(`[data-lesson="${lesson}"]`).click();
}

export const backToPath = (page: Page) => button(page, "رجوع إلى الطريق").click();

/** Plays F2 up to its open answer (any answers to the quiz), and sends `text`. */
export async function explainF2(page: Page, text: string) {
  await stone(page, "F2").click();
  await button(page, "تابع").click();
  await button(page, "اطوِها إلى نصفين").click();
  await button(page, "اطوِها إلى نصفين").click();
  await button(page, "تابع").click();
  for (let q = 0; q < 3; q++) {
    const more = page.getByRole("button", { name: "أجزاء أكثر" });
    if (await more.count()) await more.click();
    await page.locator(".cell").first().click();
    await checkAndGo(page);
  }
  await page.getByRole("textbox").fill(text);
  await button(page, "أرسل إلى معلّمي").click();
  await expect(page.getByRole("status").filter({ hasText: "أُرسلت!" })).toBeVisible();
}
