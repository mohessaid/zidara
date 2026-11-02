import { expect, test } from "@playwright/test";
import { backToPath, button, expectAccessible, explainF2, F3_OPENS, playF1, signIn, startAsGuest, stone } from "./helpers.ts";

test("the front door is in Arabic, right to left, accessible, and lists no one", async ({ page }) => {
  await page.goto("/");
  await expect(page.locator("html")).toHaveAttribute("dir", "rtl");
  await expect(page.getByRole("heading", { name: "تعلّم وأنت تلعب." })).toBeVisible();
  await expect(page.getByText("أمينة")).toHaveCount(0);
  await expectAccessible(page, "front door");
  await page.getByRole("combobox").selectOption("fr");
  await expect(page.locator("html")).toHaveAttribute("dir", "ltr");
  await expect(page.getByRole("heading", { name: "Apprends en jouant." })).toBeVisible();
  await page.getByRole("combobox").selectOption("ar");
  // The demo classes are one small link away, for teachers.
  await page.getByRole("link", { name: "للمعلّمين: الأقسام التجريبية" }).click();
  await expect(page.getByRole("heading", { name: "من أنت؟" })).toBeVisible();
  await expectAccessible(page, "demo classes");
});

test("a guest starts a course in one tap, learns, and can switch to the other course", async ({ page }) => {
  await startAsGuest(page);
  await expect(page.getByRole("heading", { name: "الكسور المتكافئة" }).first()).toBeVisible();
  await expect(stone(page, "F1")).toHaveAttribute("data-state", "not_started");
  await expectAccessible(page, "guest path");
  await playF1(page);
  await backToPath(page);
  await expect(stone(page, "F1")).toHaveAttribute("data-state", "achieved");
  // Points come from the log at once, not after a reload.
  await expect(page.getByRole("group", { name: "نقاطك وأيامك المتتالية" })).toContainText("النقاط 20");
  // The pedagogy lens: the same work judged by Cambridge-style rules, on this phone only.
  await page.getByRole("button", { name: /على طريقة كامبريدج/ }).click();
  await expect(stone(page, "F1")).not.toHaveAttribute("data-state", "achieved");
  await expect(page.getByRole("status").filter({ hasText: "نفس إجاباتك" })).toBeVisible();
  await expectAccessible(page, "pedagogy lens");
  await page.getByRole("button", { name: /الإتقان/ }).click();
  await expect(stone(page, "F1")).toHaveAttribute("data-state", "achieved");
  await page.getByRole("button", { name: "القائمة" }).click();
  await page.getByRole("button", { name: /كيف تعمل زيدارا/ }).click();
  await expect(stone(page, "Z1")).toHaveAttribute("data-state", "not_started");
  // Back on fractions, the work is still there.
  await page.getByRole("button", { name: "القائمة" }).click();
  await page.getByRole("button", { name: /الكسور المتكافئة/ }).click();
  await expect(stone(page, "F1")).toHaveAttribute("data-state", "achieved");
});

test("a learner sees the path, mirrored for Arabic, with locks explained", async ({ page }) => {
  await signIn(page, "amina");
  await expect(stone(page, "F1")).toHaveAttribute("data-state", "not_started");
  await expect(stone(page, "F2")).toHaveAttribute("data-state", "locked");
  await expect(stone(page, "F2")).toContainText("تُفتح بعد إتقان");
  const [first, last] = [await stone(page, "F1").boundingBox(), await stone(page, "F3").boundingBox()];
  expect(first!.x).toBeGreaterThan(last!.x);
  await expectAccessible(page, "path");
  // A locked level is aria-disabled, but a tap still says why it is locked.
  await stone(page, "F2").click({ force: true });
  await expect(page.getByRole("status").filter({ hasText: "تُفتح بعد إتقان" })).toBeVisible();
});

test("offline work is kept, then syncs to the server when the network returns", async ({ page, context, browser }) => {
  await signIn(page, "amina");
  await expect(page.getByTestId("sync")).toHaveText(/كل شيء محفوظ/);
  await expect(stone(page, "F1")).toBeVisible();
  await context.setOffline(true);
  await playF1(page);
  await expectAccessible(page, "finish");
  await backToPath(page);
  await expect(page.getByTestId("sync")).toHaveText(/غير متصل|في انتظار/);
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

test("a session is one step at a time, checked at once, and a retake shows another version", async ({ page }) => {
  await signIn(page, "yacine");
  await stone(page, "F1").click();
  await expect(page.getByRole("dialog")).toBeVisible();
  await expectAccessible(page, "session: film");
  await button(page, "تابع").click();
  await button(page, "تابع").click();
  // A wrong first try: the kernel says so at once, gently, with the answer and why.
  await page.getByRole("radio").nth(1).click();
  await button(page, "تحقّق").click();
  await expect(page.getByRole("status").filter({ hasText: "ليس تمامًا" })).toContainText("1/4");
  await expectAccessible(page, "session: feedback");
  // Leaving mid-quiz records nothing: the first version is still due.
  await button(page, "أغلق وارجع إلى الطريق").click();
  expect(await playF1(page)).toBe(0);
  await backToPath(page);
  expect(await playF1(page)).toBe(1);
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
  await playF1(page);
  await backToPath(page);
  await explainF2(page, "هو نفس الشيء");

  const proposal = studio.getByTestId("proposal").filter({ hasText: "هو نفس الشيء" });
  await expect(proposal).toBeVisible({ timeout: 30_000 });
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
  if (Date.now() < F3_OPENS) {
    await expect(stone(page, "F3")).toHaveAttribute("data-state", "locked");
    await expect(stone(page, "F3")).toContainText("تُفتح يوم");
  } else {
    await expect(stone(page, "F3")).toHaveAttribute("data-state", "not_started");
  }
});
