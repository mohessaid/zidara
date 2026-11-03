import { expect, test, type Page } from "@playwright/test";
import { arabic, button, expectAccessible, lowContrast, signIn, startAsGuest, stone } from "../e2e/helpers.ts";

// Every screen, measured (ADR 0016): axe plus our own walk of every text and every control edge
// (helpers.ts, lowContrast), in each state a reader meets: the front page in English and Arabic,
// the course cards, the path, each kind of lesson step, a right and a wrong answer, the end of a
// lesson, the sign-in and the teacher's studio. Runs on the static demo, so it needs no server.

/** The front page with every section rendered (content-visibility skips unseen ones). */
async function wholePage(page: Page) {
  await page.addStyleTag({ content: ".landing section { content-visibility: visible !important; }" });
  for (let y = 0; y < 14000; y += 800) await page.evaluate((top) => scrollTo(0, top), y);
  await page.evaluate(() => scrollTo(0, 0));
}

test("the front page, measured in English and Arabic, light and dark", async ({ page }) => {
  await page.emulateMedia({ reducedMotion: "reduce" });
  await page.goto("/");
  await expect(page.getByRole("heading", { level: 1 })).toBeVisible();
  await wholePage(page);
  await expectAccessible(page, "front page, English");
  await page.emulateMedia({ reducedMotion: "reduce", colorScheme: "dark" });
  await expectAccessible(page, "front page, English, dark");
  await page.getByRole("combobox").first().selectOption("ar");
  await expect(page.locator("html")).toHaveAttribute("dir", "rtl");
  await wholePage(page);
  await expectAccessible(page, "front page, Arabic");
});

test("the front page draws no gradients and no logo", async ({ page }) => {
  // The owner's call (2026-11-03): gradients and badges read as AI-made, and the page carries no logo.
  await page.goto("/");
  await expect(page.getByRole("heading", { level: 1 })).toBeVisible();
  await wholePage(page);
  const found = await page.evaluate(() =>
    [...document.querySelectorAll(".landing *")].flatMap((el) => {
      const cs = getComputedStyle(el);
      const hits = [cs.backgroundImage, cs.borderImageSource, cs.maskImage].filter((v) => v.includes("gradient"));
      const pseudo = ["::before", "::after"].filter((p) => getComputedStyle(el, p).backgroundImage.includes("gradient"));
      return hits.length || pseudo.length ? [`${el.tagName.toLowerCase()}.${[...el.classList].join(".")}`] : [];
    }),
  );
  expect(found).toEqual([]);
  await expect(page.locator(".landing .mark")).toHaveCount(0);
});

test("every learner screen, measured: cards, path, each step, right and wrong, the end", async ({ page }) => {
  await page.emulateMedia({ reducedMotion: "reduce" });
  await arabic(page);
  await page.goto("/#play");
  await expect(page.locator("[data-lesson]").first()).toBeVisible();
  await expectAccessible(page, "course cards");

  await startAsGuest(page, "equivalent-fractions");
  await expect(stone(page, "F1")).toBeVisible();
  await expectAccessible(page, "path");
  await page.getByRole("button", { name: /على طريقة كامبريدج/ }).click();
  await expectAccessible(page, "path, other school rules");
  await page.getByRole("button", { name: /الإتقان/ }).click();

  await stone(page, "F1").click();
  await expectAccessible(page, "lesson: watch");
  await button(page, "تابع").click();
  await expectAccessible(page, "lesson: read");
  await button(page, "تابع").click();
  await expectAccessible(page, "lesson: question");
  // A wrong answer first (the last option is never the right one in quiz-f1), then the next.
  await page.getByRole("radio").last().click();
  await expectAccessible(page, "lesson: answer picked");
  await button(page, "تحقّق").click();
  await expectAccessible(page, "lesson: not this time");
  // Answers already given stay solid tiles; only a button not ready yet wears the dashed edge.
  for (const style of await page.getByRole("radio").evaluateAll((els) => els.map((e) => getComputedStyle(e).borderTopStyle)))
    expect(style).toBe("solid");
  await button(page, "تابع").click();
  await page.getByRole("spinbutton", { name: "البسط" }).fill("1");
  await page.getByRole("spinbutton", { name: "المقام" }).fill("1");
  await expectAccessible(page, "lesson: typed fraction");
  await button(page, "تحقّق").click();
  await button(page, "تابع").click();
  await expect(page.locator(".finish")).toBeVisible();
  await expectAccessible(page, "lesson: the end");
});

test("a right answer, the sign-in and the teacher's studio, measured", async ({ page }) => {
  await page.emulateMedia({ reducedMotion: "reduce" });
  await signIn(page, "yacine");
  await stone(page, "F1").click();
  await button(page, "تابع").click();
  await button(page, "تابع").click();
  const variant = Number(await page.locator("[data-variant]").getAttribute("data-variant"));
  await page.getByRole("radio").nth([0, 1, 2][variant]!).click();
  await button(page, "تحقّق").click();
  await expectAccessible(page, "lesson: right");

  await button(page, "تابع").click();
  await page.getByRole("button", { name: /أغلق/ }).first().click();
  await page.getByRole("button", { name: "القائمة" }).click();
  await expectAccessible(page, "learner menu");
  await page.getByRole("button", { name: "خروج" }).click();
  await expect(page.getByRole("heading", { name: "من أنت؟" })).toBeVisible();
  await expectAccessible(page, "sign-in");
  await signIn(page, "karim");
  await expect(page.locator("main")).toHaveAttribute("data-framework", /.+/);
  await expectAccessible(page, "studio");
});

test("the measure itself catches faint text, faint edges, fades and gradients", async ({ page }) => {
  // The check is only worth something if it fails when it should: plant one of each mistake.
  await page.goto("/#play");
  await expect(page.locator("[data-lesson]").first()).toBeVisible();
  await page.evaluate(() => {
    const box = document.createElement("div");
    box.id = "planted";
    box.innerHTML = `
      <p style="color:#a1a1aa;background:#fff">faint words</p>
      <button style="border:2px solid #e4e4e7;background:#fff;color:#000">faint edge</button>
      <p style="color:#000;opacity:.3">faded words</p>
      <div style="background:linear-gradient(#fff,#111)"><span style="color:#000">words on a gradient</span></div>
      <button style="border:0;background:none;color:#000">plain text button</button>
      <button disabled style="border:0;box-shadow:none;background:#f4f4f5;color:#55555d">grey disabled</button>
      <div data-meter style="height:12px;background:#f4f4f5"></div>`;
    document.body.prepend(box);
  });
  const found = (await lowContrast(page)).join("\n");
  expect(found).toContain("faint words");
  expect(found).toContain("edge button");
  expect(found).toContain("faded words");
  expect(found).toContain("words on a gradient");
  expect(found).not.toContain("plain text button");
  expect(found).toContain("edge button \"grey disabled\"");
  expect(found).toContain("edge div");
});
