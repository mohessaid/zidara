import { expect, type Page } from "@playwright/test";
import AxeBuilder from "@axe-core/playwright";

export const PEOPLE = { amina: "أمينة", yacine: "ياسين", sara: "سارة", karim: "كريم" } as const;

/** The app opens in English (ADR 0014); the school's story is told in Arabic, its first
 * language. Seeds the choice before the page loads, unless the page has already made one. */
export async function arabic(page: Page) {
  await page.addInitScript(() => {
    if (!localStorage.getItem("zidara.locale")) localStorage.setItem("zidara.locale", "ar");
  });
}

/** Signs in one of the demo classes' people (#classes); the game's front door is for guests. */
export async function signIn(page: Page, who: keyof typeof PEOPLE) {
  await arabic(page);
  await page.goto("/#classes");
  await page.getByRole("button", { name: new RegExp(`الدخول باسم ${PEOPLE[who]}`) }).click();
}

/** Zero WCAG 2.2 A/AA violations on what is on screen, and every colour measured (below). */
export async function expectAccessible(page: Page, screen: string) {
  const results = await new AxeBuilder({ page }).withTags(["wcag2a", "wcag2aa", "wcag21a", "wcag21aa", "wcag22aa"]).analyze();
  expect(results.violations.map((v) => `${v.id}: ${v.nodes.map((n) => n.target.join(" ")).join(", ")}`), screen).toEqual([]);
  expect(await lowContrast(page), `${screen}: measured contrast`).toEqual([]);
}

/**
 * Measures contrast the way a reader meets it (ADR 0016). axe leaves out text it cannot decide
 * (over gradients, under translucent layers) and never checks the edges of controls, which is
 * how low contrast kept slipping through. This walks every visible text and every control:
 * - text: its colour against what is really behind it (translucent layers composited, every stop
 *   of a gradient tried), 4.5:1, or 3:1 for large text (24 px, or 18.66 px bold);
 * - a control (button, input, select, textarea, radio, a link drawn as a button): if it draws a
 *   boundary (a border or a fill different from behind it), that boundary must reach 3:1. A
 *   faint outline is worse than none (WCAG 1.4.11; stricter on purpose). Disabled controls,
 *   which WCAG exempts, are skipped.
 * Returns one line per failure.
 */
export async function lowContrast(page: Page): Promise<string[]> {
  return page.evaluate(() => {
    const ctx = document.createElement("canvas").getContext("2d", { willReadFrequently: true })!;
    type Rgba = [number, number, number, number];
    const cache = new Map<string, Rgba>();
    // Any CSS colour (rgb, oklch, color-mix…) through the canvas, which speaks them all.
    const rgba = (css: string): Rgba => {
      const hit = cache.get(css);
      if (hit) return hit;
      ctx.clearRect(0, 0, 1, 1);
      ctx.fillStyle = "#000";
      ctx.fillStyle = css;
      ctx.fillRect(0, 0, 1, 1);
      const d = ctx.getImageData(0, 0, 1, 1).data;
      const out: Rgba = [d[0]!, d[1]!, d[2]!, d[3]! / 255];
      cache.set(css, out);
      return out;
    };
    const over = (top: Rgba, under: Rgba): Rgba => {
      const a = top[3];
      return [top[0] * a + under[0] * (1 - a), top[1] * a + under[1] * (1 - a), top[2] * a + under[2] * (1 - a), 1];
    };
    const lum = (c: Rgba) => {
      const f = (v: number) => ((v /= 255) <= 0.03928 ? v / 12.92 : ((v + 0.055) / 1.055) ** 2.4);
      return 0.2126 * f(c[0]) + 0.7152 * f(c[1]) + 0.0722 * f(c[2]);
    };
    const ratio = (a: Rgba, b: Rgba) => {
      const [x, y] = [lum(a), lum(b)].sort((p, q) => q - p) as [number, number];
      return (x + 0.05) / (y + 0.05);
    };
    const hex = (c: Rgba) => "#" + c.slice(0, 3).map((v) => Math.round(v).toString(16).padStart(2, "0")).join("");
    const STOP = /(?:rgba?|hsla?|oklch|oklab|lab|lch|color)\([^()]*(?:\([^()]*\)[^()]*)*\)|#[0-9a-f]{3,8}\b/gi;

    /** Every colour that may sit behind `el`: its own and its ancestors' backgrounds, composited. */
    function behind(el: Element | null, self: boolean): Rgba[] {
      const layers: string[][] = [];
      for (let n = self ? el : (el?.parentElement ?? null); n; n = n.parentElement) {
        const cs = getComputedStyle(n);
        const stops = cs.backgroundImage.includes("gradient") ? (cs.backgroundImage.match(STOP) ?? []) : [];
        const colour = cs.backgroundColor;
        if (stops.length) layers.push(stops);
        else if (rgba(colour)[3] > 0) layers.push([colour]);
        if (rgba(colour)[3] >= 1 && !stops.length) break;
      }
      let under: Rgba[] = [[255, 255, 255, 1]];
      for (const layer of layers.reverse()) {
        const next: Rgba[] = [];
        for (const c of layer) for (const u of under) next.push(over(rgba(c), u));
        under = next;
      }
      return under;
    }
    const shown = (el: Element) => {
      const r = el.getBoundingClientRect();
      if (r.width < 2 || r.height < 2) return false;
      for (let n: Element | null = el; n; n = n.parentElement) {
        const cs = getComputedStyle(n);
        if (cs.visibility === "hidden" || cs.display === "none" || Number(cs.opacity) === 0) return false;
        if (cs.clipPath.startsWith("inset(50%")) return false;
      }
      return true;
    };
    const fade = (el: Element) => {
      let a = 1;
      for (let n: Element | null = el; n; n = n.parentElement) a *= Number(getComputedStyle(n).opacity);
      return a;
    };
    const name = (el: Element) => {
      const cls = [...el.classList].slice(0, 2).join(".");
      const words = (el.textContent ?? "").trim().replace(/\s+/g, " ").slice(0, 40);
      return `${el.tagName.toLowerCase()}${cls ? "." + cls : ""} "${words}"`;
    };
    const inert = (el: Element) => (el as HTMLButtonElement).disabled || el.closest("[disabled], [inert]") !== null;
    const out: string[] = [];

    for (const el of document.body.querySelectorAll("*")) {
      if (el.closest("svg, script, style, noscript, video, audio, canvas, img")) continue;
      const text = [...el.childNodes].some((c) => c.nodeType === 3 && c.textContent!.trim() !== "");
      if (!text || !shown(el) || inert(el)) continue;
      const cs = getComputedStyle(el);
      const size = parseFloat(cs.fontSize);
      const need = size >= 24 || (size >= 18.66 && Number(cs.fontWeight) >= 700) ? 3 : 4.5;
      const fg = rgba(cs.color);
      const a = fg[3] * fade(el);
      for (const bg of behind(el, true)) {
        const r = ratio(over([fg[0], fg[1], fg[2], a], bg), bg);
        if (r < need) {
          out.push(`text ${name(el)}: ${r.toFixed(2)} < ${need} (${hex(over([fg[0], fg[1], fg[2], a], bg))} on ${hex(bg)})`);
          break;
        }
      }
    }

    const CONTROLS = "button, input, select, textarea, [role=radio], [role=tab], [role=switch], a";
    for (const el of document.body.querySelectorAll(CONTROLS)) {
      if (!shown(el) || inert(el) || el.closest("svg")) continue;
      const cs = getComputedStyle(el);
      const parents = behind(el, false);
      const fill = rgba(cs.backgroundColor);
      const width = Math.max(...["Top", "Right", "Bottom", "Left"].map((s) => parseFloat(cs.getPropertyValue(`border-${s.toLowerCase()}-width`)) || 0));
      const border = cs.borderStyle !== "none" && width >= 1 ? rgba(cs.borderTopColor) : null;
      for (const bg of parents) {
        const edges: number[] = [];
        if (fill[3] > 0) edges.push(ratio(over(fill, bg), bg));
        if (border && border[3] > 0) edges.push(ratio(over(border, fill[3] > 0 ? over(fill, bg) : bg), bg), ratio(over(border, bg), bg));
        const drawn = edges.filter((r) => r > 1.05);
        if (drawn.length && Math.max(...drawn) < 3) {
          out.push(`edge ${name(el)}: ${Math.max(...drawn).toFixed(2)} < 3 (on ${hex(bg)})`);
          break;
        }
      }
    }
    return out;
  });
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

/** Starts as a guest on the course `lesson`, from the game's front door (#play). */
export async function startAsGuest(page: Page, lesson = "equivalent-fractions") {
  await arabic(page);
  await page.goto("/#play");
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
