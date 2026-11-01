// The demo film for Solve: the real app on the real server, recorded by the browser.
// Not a test of correctness (tests/e2e does that); a film of the product working. Each step
// records its time and caption, so scripts/compose-demo.py places the captions exactly.
// Run: npx playwright test --config playwright.demo.config.ts
import { expect, test } from "@playwright/test";
import { backToPath, button, signIn, stone } from "../e2e/helpers.ts";

const pause = (ms: number) => new Promise((r) => setTimeout(r, ms));

test("Zidara in ninety seconds", async ({ browser }) => {
  const video = { dir: "test-results/demo", size: { width: 412, height: 915 } };
  const learner = await browser.newContext({ recordVideo: video, viewport: video.size, locale: "ar" });
  const amina = await learner.newPage();
  const startedAt = Date.now();
  const marks: { t: number; caption: string }[] = [];
  const mark = (caption: string) => marks.push({ t: (Date.now() - startedAt) / 1000, caption });

  // 1. Arabic first: a learner signs in and sees her path.
  mark("Amina signs in. Arabic first, right to left.");
  await signIn(amina, "amina");
  await expect(stone(amina, "F1")).toBeVisible();
  await pause(2000);

  // 2. The network drops; she still learns, a step at a time, marked by her phone's kernel.
  await learner.setOffline(true);
  mark("Offline. A lesson plays like a game, one step at a time.");
  await stone(amina, "F1").click();
  await pause(2500);
  await button(amina, "تابع").click();
  await pause(1800);
  await button(amina, "تابع").click();
  mark("Pictures and instant feedback, marked by the kernel on her phone.");
  // A fresh school: her first try is the first version (the pizza, then 3/8).
  await pause(1500);
  await amina.getByRole("radio").nth(0).click();
  await pause(600);
  await button(amina, "تحقّق").click();
  await pause(2200);
  await button(amina, "تابع").click();
  await pause(1200);
  await amina.getByRole("spinbutton", { name: "البسط" }).pressSequentially("3", { delay: 200 });
  await amina.getByRole("spinbutton", { name: "المقام" }).pressSequentially("8", { delay: 200 });
  await button(amina, "تحقّق").click();
  await pause(2200);
  await button(amina, "تابع").click();
  await expect(amina.getByRole("heading", { name: "أصبحت تستطيع هذا!" })).toBeVisible();
  mark("F1 mastered: the class's teaching approach says so, not the game.");
  await pause(2500);
  await backToPath(amina);
  await pause(1200);

  // 3. A toy before practice, then an answer given by shading a bar.
  await stone(amina, "F2").click();
  await button(amina, "تابع").click();
  mark("Brilliant-style: fold the paper. 1/2 = 2/4 = 4/8.");
  for (let i = 0; i < 3; i++) {
    await pause(800);
    await button(amina, "اطوِها إلى نصفين").click();
  }
  await pause(1200);
  await button(amina, "تابع").click();
  mark("She answers by shading the bar: 1/2 = 2/4.");
  // The first version: 2 of 4, 4 of 6, then any fraction equal to 3/4 (cut in 4, shade 3).
  for (const [cuts, shade] of [[0, 2], [0, 4], [3, 3]] as const) {
    for (let i = 0; i < cuts; i++) await amina.getByRole("button", { name: "أجزاء أكثر" }).click();
    for (let i = 0; i < shade; i++) {
      await pause(250);
      await amina.locator(".cell").nth(i).click();
    }
    await button(amina, "تحقّق").click();
    await pause(shade === 2 ? 2000 : 900);
    await button(amina, "تابع").click();
  }

  // 4. She explains in her own words; the AI assistant drafts a mark for her teacher.
  mark("She explains in her own words. The AI assistant will draft a mark.");
  await amina.getByRole("textbox").pressSequentially("هو نفس الشيء", { delay: 110 });
  await button(amina, "أرسل إلى معلّمي").click();
  await pause(1800);
  await button(amina, "أغلق وارجع إلى الطريق").click();

  // 5. The network returns; everything syncs.
  await learner.setOffline(false);
  mark("Back online: her work syncs.");
  await expect(amina.getByTestId("sync")).toHaveText(/كل شيء محفوظ/, { timeout: 20_000 });
  await pause(1200);

  // 6. The teacher: the assistant's unsure draft waits for a human decision.
  const studio = await browser.newContext({ recordVideo: video, viewport: video.size, locale: "ar" });
  const karim = await studio.newPage();
  const teacherAt = (Date.now() - startedAt) / 1000;
  await signIn(karim, "karim");
  mark("The teacher: the AI's unsure mark waits for her decision.");
  const proposal = karim.getByTestId("proposal").first();
  await expect(proposal).toBeVisible({ timeout: 30_000 });
  await proposal.scrollIntoViewIfNeeded();
  await pause(2200);
  await proposal.getByRole("button", { name: "اقبل" }).click();
  await pause(1200);

  // 7. A live round: the board sets the pace, every phone follows.
  await karim.locator("main").evaluate((m) => m.scrollIntoView());
  await karim.getByRole("combobox", { name: "التمرين" }).selectOption("quiz-f1");
  await button(karim, "ابدأ الجولة").click();
  mark("A live round, like Kahoot: every phone follows the board.");
  await expect(amina.getByText("جولة مباشرة!")).toBeVisible({ timeout: 15_000 });
  await pause(1500);
  await amina.getByRole("radio").nth(0).click();
  mark("Each answer marked at once. No timer, no names, no ranking.");
  await expect(amina.getByText("صحيح!")).toBeVisible({ timeout: 10_000 });
  await pause(1500);
  await button(karim, "أظهر الإجابة").click();
  await pause(2200);
  await button(karim, "أنهِ الجولة").click();
  mark("The round ends: every answer becomes evidence.");
  await expect(karim.getByText(/القسم معًا/)).toBeVisible({ timeout: 10_000 });
  await pause(2500);

  // 8. One switch changes the pedagogy: the same work, judged by Cambridge-style rules.
  await karim.getByRole("combobox", { name: "طريقة التدريس" }).scrollIntoViewIfNeeded();
  await karim.getByRole("combobox", { name: "طريقة التدريس" }).selectOption("cambridge-style");
  await button(karim, "طبّق").click();
  mark("One switch: Cambridge-style. The same evidence, judged again.");
  await expect(karim.locator("main")).toHaveAttribute("data-framework", "cambridge-style", { timeout: 15_000 });
  await pause(1200);
  await amina.reload();
  await expect(stone(amina, "F1")).toHaveAttribute("data-state", "in_progress", { timeout: 15_000 });
  await pause(3000);
  await studio.close();
  await learner.close();
  const { writeFileSync } = await import("node:fs");
  writeFileSync(
    "test-results/demo/timeline.json",
    JSON.stringify({ teacherAt, end: (Date.now() - startedAt) / 1000, marks, learner: await amina.video()!.path(), teacher: await karim.video()!.path() }),
  );
});
