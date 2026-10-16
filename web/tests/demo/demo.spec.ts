// The one-minute demo for Solve: the real app on the real server, recorded by the browser.
// Not a test of correctness (tests/e2e does that); a film of the product working.
// Run: npx playwright test --config playwright.demo.config.ts
import { expect, test } from "@playwright/test";
import { passQuizF1, signIn, stone } from "../e2e/helpers.ts";

const pause = (ms: number) => new Promise((r) => setTimeout(r, ms));

test("Zidara in one minute", async ({ browser }) => {
  const video = { dir: "test-results/demo", size: { width: 412, height: 915 } };
  const learner = await browser.newContext({ recordVideo: video, viewport: video.size, locale: "ar" });
  const amina = await learner.newPage();
  const startedAt = Date.now();

  // 1. Arabic first: a learner signs in and sees her path.
  await signIn(amina, "amina");
  await expect(stone(amina, "F1")).toBeVisible();
  await pause(2500);

  // 2. The network drops; she still learns. Her phone's kernel marks the quiz.
  await learner.setOffline(true);
  await stone(amina, "F1").click();
  await pause(800);
  await passQuizF1(amina);
  await pause(1500);
  await amina.getByRole("button", { name: /رجوع إلى الطريق/ }).first().click();
  await amina.getByRole("button", { name: /رجوع إلى الطريق/ }).first().click();
  await expect(stone(amina, "F1")).toHaveAttribute("data-state", "achieved");
  await pause(2500);

  // 3. She explains in her own words; the AI assistant will draft a mark for her teacher.
  await stone(amina, "F2").click();
  await amina.getByRole("button", { name: /اشرح بكلماتك/ }).click();
  await amina.getByRole("textbox").pressSequentially("هو نفس الشيء", { delay: 120 });
  await amina.getByRole("button", { name: "أرسل إلى معلّمي" }).click();
  await pause(1500);

  // 4. The network returns; everything syncs.
  await learner.setOffline(false);
  await expect(amina.getByTestId("sync")).toHaveText(/كل شيء محفوظ/, { timeout: 20_000 });
  await pause(1500);

  // 5. The teacher: who is where, and the assistant's draft waiting for a human decision.
  const studio = await browser.newContext({ recordVideo: video, viewport: video.size, locale: "ar" });
  const karim = await studio.newPage();
  const teacherAt = (Date.now() - startedAt) / 1000;
  await signIn(karim, "karim");
  const proposal = karim.getByTestId("proposal").first();
  await expect(proposal).toBeVisible({ timeout: 20_000 });
  await proposal.scrollIntoViewIfNeeded();
  await pause(2500);
  await proposal.getByRole("button", { name: "اقبل" }).click();
  await pause(1500);

  // 6. One switch changes the pedagogy: same work, Cambridge-style rules.
  await karim.getByRole("combobox", { name: "طريقة التدريس" }).scrollIntoViewIfNeeded();
  await karim.getByRole("combobox", { name: "طريقة التدريس" }).selectOption("cambridge-style");
  await karim.getByRole("button", { name: "طبّق" }).click();
  await expect(karim.locator("main")).toHaveAttribute("data-framework", "cambridge-style", { timeout: 15_000 });
  await pause(1500);
  await studio.close();

  await amina.reload();
  await expect(stone(amina, "F1")).toHaveAttribute("data-state", "in_progress", { timeout: 15_000 });
  await pause(3000);
  await learner.close();
  // Where the teacher's recording starts on the learner's timeline, for compose-demo.sh.
  const { writeFileSync } = await import("node:fs");
  writeFileSync("test-results/demo/timeline.json", JSON.stringify({ teacherAt, learner: await amina.video()!.path(), teacher: await karim.video()!.path() }));
});
