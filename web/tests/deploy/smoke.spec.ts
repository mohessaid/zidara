import { expect, test } from "@playwright/test";
import { signIn, stone } from "../e2e/helpers.ts";

test("the API refuses a visitor who has not signed in", async ({ request }) => {
  expect((await request.get("/api/me")).status()).toBe(401);
});

test("a learner signs in and sees the path", async ({ page }) => {
  await signIn(page, "amina");
  await expect(stone(page, "F1")).toBeVisible();
  await expect(page.getByTestId("sync")).toHaveText(/كل شيء محفوظ/);
});

test("after one visit, the app opens with no network", async ({ page, context }) => {
  await signIn(page, "amina");
  await expect(stone(page, "F1")).toBeVisible();
  await page.evaluate(() => navigator.serviceWorker.ready);
  await context.setOffline(true);
  await page.reload();
  await expect(stone(page, "F1")).toBeVisible();
});
