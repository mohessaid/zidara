import { expect, test } from "@playwright/test";
import { arabic, backToPath, expectAccessible, explainF2, F3_OPENS, playF1, signIn, startAsGuest, stone } from "../e2e/helpers.ts";

async function signOut(page: import("@playwright/test").Page) {
  // A learner's sign-out is in the menu; a teacher's is in the header.
  const menu = page.getByRole("button", { name: "القائمة" });
  if (await menu.count()) await menu.click();
  await page.getByRole("button", { name: "خروج" }).click();
  await expect(page.getByRole("heading", { name: "من أنت؟" })).toBeVisible();
}

test("the whole school story runs in one browser, with no server", async ({ page }) => {
  const api: string[] = [];
  page.on("request", (r) => r.url().includes("/api/") && api.push(r.url()));

  await arabic(page);
  await page.goto("/#classes");
  await expect(page.getByTestId("demo-notice")).toContainText("يعمل كلّه في متصفّحك");
  await expectAccessible(page, "demo sign-in");

  // A learner masters F1, then writes an open answer the rubric is unsure of.
  await signIn(page, "amina");
  await playF1(page);
  await backToPath(page);
  await expect(stone(page, "F1")).toHaveAttribute("data-state", "achieved");
  await explainF2(page, "هو نفس الشيء");
  await expect(page.getByTestId("sync")).toHaveText(/كل شيء محفوظ/);
  await page.getByRole("button", { name: "أغلق وارجع إلى الطريق", exact: true }).click();
  await signOut(page);

  // The teacher accepts the queued answer, then switches the class to Cambridge-style.
  await signIn(page, "karim");
  const proposal = page.getByTestId("proposal").filter({ hasText: "هو نفس الشيء" });
  await expect(proposal).toBeVisible();
  await expectAccessible(page, "demo studio");
  await proposal.getByRole("button", { name: "اقبل" }).click();
  await expect(proposal).toHaveCount(0);
  await page.getByRole("combobox", { name: "طريقة التدريس" }).selectOption("cambridge-style");
  await page.getByRole("button", { name: "طبّق" }).click();
  await expect(page.locator("main")).toHaveAttribute("data-framework", "cambridge-style");
  await signOut(page);

  // The same evidence under another pedagogy redraws the learner's path.
  await signIn(page, "amina");
  await expect(stone(page, "F1")).toHaveAttribute("data-state", "in_progress");
  await expect(stone(page, "F3")).toHaveAttribute("data-state", Date.now() < F3_OPENS ? "locked" : "not_started");
  await signOut(page);

  // Another class sees none of it.
  await signIn(page, "sara");
  await expect(stone(page, "F1")).toHaveAttribute("data-state", "not_started");

  // No call reached a server: every API path was answered inside the page.
  expect(api).toEqual([]);
});

test("a guest learns in the static demo too, with no server", async ({ page }) => {
  const api: string[] = [];
  page.on("request", (r) => r.url().includes("/api/") && api.push(r.url()));
  await startAsGuest(page, "how-zidara-works");
  await expect(stone(page, "Z1")).toHaveAttribute("data-state", "not_started");
  await expectAccessible(page, "demo guest path");
  expect(api).toEqual([]);
});

test("after one visit, the demo opens and works with no network", async ({ page, context }) => {
  await signIn(page, "yacine");
  await expect(stone(page, "F1")).toBeVisible();
  await page.evaluate(() => navigator.serviceWorker.ready);
  await context.setOffline(true);
  await page.reload();
  await expect(stone(page, "F1")).toBeVisible();
  // The session's code, the toys and the sounds were cached on the first visit too.
  await playF1(page);
});

test("a teacher's unsaved choice of approach survives the studio's refresh", async ({ page }) => {
  // The studio refreshes every 4.5 s; it used to put the select back to the class's approach,
  // so a teacher who took a moment before pressing Apply lost the choice (found 5 November).
  await signIn(page, "karim");
  const approach = page.locator("form:has(button.primary) select").last();
  await approach.selectOption("cambridge-style");
  await page.waitForTimeout(6_000);
  await expect(approach).toHaveValue("cambridge-style");
  await expect(page.getByRole("button", { name: "طبّق" })).toBeEnabled();
});
