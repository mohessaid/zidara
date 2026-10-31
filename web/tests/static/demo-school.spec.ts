import { expect, test } from "@playwright/test";
import { expectAccessible, F3_OPENS, passQuizF1, signIn, stone } from "../e2e/helpers.ts";

async function signOut(page: import("@playwright/test").Page) {
  await page.getByRole("button", { name: "خروج" }).click();
  await expect(page.getByRole("heading", { name: "من أنت؟" })).toBeVisible();
}

test("the whole school story runs in one browser, with no server", async ({ page }) => {
  const api: string[] = [];
  page.on("request", (r) => r.url().includes("/api/") && api.push(r.url()));

  await page.goto("/");
  await expect(page.getByTestId("demo-notice")).toContainText("يعمل كلّه في متصفّحك");
  await expectAccessible(page, "demo sign-in");

  // A learner masters F1, then writes an open answer the rubric is unsure of.
  await signIn(page, "amina");
  await stone(page, "F1").click();
  await passQuizF1(page);
  await page.getByRole("button", { name: /رجوع إلى الطريق/ }).first().click();
  await page.getByRole("button", { name: /رجوع إلى الطريق/ }).first().click();
  await expect(stone(page, "F1")).toHaveAttribute("data-state", "achieved");
  await stone(page, "F2").click();
  await page.getByRole("button", { name: /اشرح بكلماتك/ }).click();
  await page.getByRole("textbox").fill("هو نفس الشيء");
  await page.getByRole("button", { name: "أرسل إلى معلّمي" }).click();
  await expect(page.getByRole("status").filter({ hasText: "وصلت إجابتك" })).toBeVisible();
  await expect(page.getByTestId("sync")).toHaveText(/كل شيء محفوظ/);
  await page.getByRole("button", { name: /رجوع إلى الطريق/ }).first().click();
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

  expect(api.filter((url) => !url.startsWith("http://127.0.0.1:4175/"))).toEqual([]);
});
