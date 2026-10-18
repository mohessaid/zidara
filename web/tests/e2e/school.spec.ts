import { expect, test } from "@playwright/test";
import { expectAccessible, passQuizF1, signIn, stone } from "./helpers.ts";

test("sign-in is in Arabic, right to left, and accessible", async ({ page }) => {
  await page.goto("/");
  await expect(page.locator("html")).toHaveAttribute("dir", "rtl");
  await expect(page.getByRole("heading", { name: "من أنت؟" })).toBeVisible();
  await expectAccessible(page, "sign-in");
  await page.getByRole("combobox").selectOption("fr");
  await expect(page.locator("html")).toHaveAttribute("dir", "ltr");
  await expect(page.getByRole("heading", { name: "Qui es-tu ?" })).toBeVisible();
  await page.getByRole("combobox").selectOption("ar");
});

test("a learner sees the path, mirrored for Arabic, with locks explained", async ({ page }) => {
  await signIn(page, "amina");
  await expect(stone(page, "F1")).toHaveAttribute("data-state", "not_started");
  await expect(stone(page, "F2")).toHaveAttribute("data-state", "locked");
  await expect(stone(page, "F2")).toContainText("تُفتح بعد إتقان");
  const [first, last] = [await stone(page, "F1").boundingBox(), await stone(page, "F3").boundingBox()];
  expect(first!.x).toBeGreaterThan(last!.x);
  await expectAccessible(page, "quest");
  await page.getByRole("button", { name: "الجدول" }).click();
  await expect(page.getByRole("table")).toBeVisible();
  await expectAccessible(page, "grid");
});

test("with data saver on, the light experience starts without the game engine", async ({ page }) => {
  await page.addInitScript(() => Object.defineProperty(navigator, "connection", { value: { saveData: true } }));
  await signIn(page, "amina");
  await expect(page.getByRole("table")).toBeVisible();
  await expect(page.getByRole("button", { name: "الجدول" })).toHaveAttribute("aria-pressed", "true");
  await expect(page.locator("canvas")).toHaveCount(0);
  await expectAccessible(page, "light");
  // The map is one tap away, and loads only then.
  await page.getByRole("button", { name: "الطريق" }).click();
  await expect(stone(page, "F1")).toBeVisible();
  await expect(page.locator("canvas")).toHaveCount(1);
});

test("if the map cannot load, the learner gets the table and can try again", async ({ page }) => {
  // The connection drops between the screen and the map: the map's code never arrives.
  await page.route("**/assets/Quest-*.js", (route) => route.abort());
  await signIn(page, "amina");
  await expect(page.getByRole("table")).toBeVisible();
  await expect(page.getByRole("status").filter({ hasText: "ستظهر الخريطة" })).toBeVisible();
  await expectAccessible(page, "map unavailable");
  await page.unroute("**/assets/Quest-*.js");
  await page.getByRole("button", { name: "الطريق" }).click();
  await expect(stone(page, "F1")).toBeVisible();
});

test("offline work is kept, then syncs to the server when the network returns", async ({ page, context, browser }) => {
  await signIn(page, "amina");
  await expect(page.getByTestId("sync")).toHaveText(/كل شيء محفوظ/);
  await expect(stone(page, "F1")).toBeVisible();
  await context.setOffline(true);
  await stone(page, "F1").click();
  await page.getByRole("button", { name: /تمرين: أجزاء من كلّ/ }).click();
  await expectAccessible(page, "quiz");
  await page.getByRole("button", { name: /رجوع إلى الطريق/ }).first().click();
  await passQuizF1(page);
  await expect(page.getByRole("status").filter({ hasText: "أحسنت المحاولة" })).toBeVisible();
  await expect(page.getByTestId("sync")).toHaveText(/غير متصل|في انتظار/);
  await page.getByRole("button", { name: /رجوع إلى الطريق/ }).first().click();
  await page.getByRole("button", { name: /رجوع إلى الطريق/ }).first().click();
  // The phone's kernel already knows: F1 is mastered and F2 opens, with no network.
  await expect(stone(page, "F1")).toHaveAttribute("data-state", "achieved");
  await expect(stone(page, "F2")).toHaveAttribute("data-state", "not_started");

  await context.setOffline(false);
  await expect(page.getByTestId("sync")).toHaveText(/كل شيء محفوظ/, { timeout: 15_000 });

  // The server has it: the teacher's studio counts Amina.
  const teacher = await browser.newContext();
  const studio = await teacher.newPage();
  await signIn(studio, "karim");
  await expect(studio.getByRole("img", { name: "1 من 2 أتقنوا" }).first()).toBeVisible();
  await teacher.close();
});

test("after one visit, the app opens with no network at all", async ({ page, context }) => {
  await signIn(page, "amina");
  await expect(stone(page, "F1")).toHaveAttribute("data-state", "achieved");
  await page.evaluate(() => navigator.serviceWorker.ready);
  await page.reload();
  await expect(stone(page, "F1")).toBeVisible();
  await context.setOffline(true);
  await page.reload();
  // The shell, the kernel and the lesson come from the device: the path is still Amina's.
  await expect(stone(page, "F1")).toHaveAttribute("data-state", "achieved");
  await expect(page.getByTestId("sync")).toHaveText(/غير متصل/);
  await context.setOffline(false);
});

test("a retake shows another version of the quiz", async ({ page }) => {
  await signIn(page, "yacine");
  await stone(page, "F1").click();
  expect(await passQuizF1(page)).toBe(0);
  await page.getByRole("button", { name: /رجوع إلى الطريق/ }).first().click();
  const first = await page.getByRole("button", { name: /تمرين: أجزاء من كلّ/ }).count();
  expect(first).toBe(1);
  expect(await passQuizF1(page)).toBe(1);
  await expect(page.locator("form[data-variant] legend").first()).toContainText("كعكة");
});

test("a learner of another class sees none of it", async ({ page }) => {
  await signIn(page, "sara");
  await expect(stone(page, "F1")).toBeVisible();
  // Sara's own sign-in, from her device: the server still refuses class A's data.
  const token = await page.evaluate(
    () =>
      new Promise<string>((resolve) => {
        const open = indexedDB.open("zidara");
        open.onsuccess = () => {
          const get = open.result.transaction("session").objectStore("session").get("current");
          get.onsuccess = () => resolve(get.result.token);
        };
      }),
  );
  const classA = "/api/cohorts/00000000-0000-0000-0000-00000000c00a";
  for (const path of ["/learner-view", "/events", "/studio"]) {
    const response = await page.request.get(classA + path, { headers: { authorization: `Bearer ${token}` } });
    expect(response.status(), path).toBe(403);
  }
  expect((await page.request.get(classA + "/learner-view")).status()).toBe(401);
});

test("an unsure answer waits for the teacher, who accepts it", async ({ page, browser }) => {
  // The teacher's studio is already open when the answer arrives: her local kernel has not
  // seen the agent's proposal yet, and accepting it must still work.
  const teacher = await browser.newContext();
  const studio = await teacher.newPage();
  await signIn(studio, "karim");
  await expect(studio.getByRole("heading", { name: "إجابات تنتظر قرارك" })).toBeVisible();

  await signIn(page, "yacine");
  await stone(page, "F1").click();
  await passQuizF1(page);
  await page.getByRole("button", { name: /رجوع إلى الطريق/ }).first().click();
  await page.getByRole("button", { name: /رجوع إلى الطريق/ }).first().click();
  await stone(page, "F2").click();
  await page.getByRole("button", { name: /اشرح بكلماتك/ }).click();
  await page.getByRole("textbox").fill("هو نفس الشيء");
  await page.getByRole("button", { name: "أرسل إلى معلّمي" }).click();
  await expect(page.getByRole("status").filter({ hasText: "وصلت إجابتك" })).toBeVisible();

  const proposal = studio.getByTestId("proposal").filter({ hasText: "هو نفس الشيء" });
  await expect(proposal).toBeVisible({ timeout: 20_000 });
  await expectAccessible(studio, "studio");
  await proposal.getByRole("button", { name: "اقبل" }).click();
  await expect(proposal).toHaveCount(0, { timeout: 10_000 });
  await teacher.close();
});

test("switching the framework redraws the same learner's path", async ({ page, browser }) => {
  await signIn(page, "amina");
  await expect(stone(page, "F1")).toHaveAttribute("data-state", "achieved");

  const teacher = await browser.newContext();
  const studio = await teacher.newPage();
  await signIn(studio, "karim");
  await studio.getByRole("combobox", { name: "طريقة التدريس" }).selectOption("cambridge-style");
  await studio.getByRole("button", { name: "طبّق" }).click();
  // Wait for the server's answer, not the button: the studio is redrawn from the server's view.
  await expect(studio.locator("main")).toHaveAttribute("data-framework", "cambridge-style", { timeout: 15_000 });
  await teacher.close();

  await page.reload();
  // Same evidence, another pedagogy: practice no longer counts as mastery, and the scheme of
  // work, not mastery, opens outcomes: F3 waits for its week (the term began on 5 October).
  await expect(stone(page, "F1")).toHaveAttribute("data-state", "in_progress", { timeout: 15_000 });
  await expect(stone(page, "F3")).toHaveAttribute("data-state", "locked");
  await expect(stone(page, "F3")).toContainText("تُفتح يوم");
});
