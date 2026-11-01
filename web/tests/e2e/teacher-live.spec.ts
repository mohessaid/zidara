import { expect, test } from "@playwright/test";
import { button, expectAccessible, signIn, stone } from "./helpers.ts";

test("a live round: the teacher sets the pace, every phone follows, and the answers become evidence", async ({ browser }) => {
  const [board, phoneA, phoneB] = await Promise.all([browser.newContext(), browser.newContext(), browser.newContext()]);
  const teacher = await board.newPage();
  const amina = await phoneA.newPage();
  const yacine = await phoneB.newPage();
  await signIn(amina, "amina");
  await signIn(yacine, "yacine");
  await expect(stone(amina, "F1")).toBeVisible();
  await expect(stone(yacine, "F1")).toBeVisible();

  await signIn(teacher, "karim");
  await teacher.getByRole("combobox", { name: "التمرين" }).selectOption("quiz-f1");
  await button(teacher, "ابدأ الجولة").click();
  await expect(teacher.getByText("أجاب 0 من 2")).toBeVisible();

  // Each phone shows the board's question within seconds, with no reload.
  await expect(amina.getByText("جولة مباشرة!")).toBeVisible({ timeout: 15_000 });
  await expect(yacine.getByText("جولة مباشرة!")).toBeVisible({ timeout: 15_000 });
  await expectAccessible(amina, "live phone");
  // quiz-f1, question 1: the first option (1/4) is right.
  await amina.getByRole("radio").nth(0).click();
  await yacine.getByRole("radio").nth(1).click();
  await expect(amina.getByText("صحيح!")).toBeVisible({ timeout: 10_000 });
  await expect(yacine.getByText("ليس هذه المرة")).toBeVisible({ timeout: 10_000 });

  // The board counts the answers; nobody is named or ranked.
  await expect(teacher.getByText("أجاب 2 من 2")).toBeVisible({ timeout: 10_000 });
  await button(teacher, "أظهر الإجابة").click();
  await expect(teacher.getByText("1 إجابات صحيحة من 2", { exact: true })).toBeVisible();
  await expectAccessible(teacher, "live board");
  await button(teacher, "السؤال التالي").click();
  await expect(amina.getByText("السؤال 2 من 2")).toBeVisible({ timeout: 15_000 });
  await button(teacher, "أنهِ الجولة").click();
  await expect(teacher.getByText(/القسم معًا/)).toBeVisible({ timeout: 10_000 });

  // The round is over on every phone, and the class's framework judged the answers.
  await expect(amina.getByText("جولة مباشرة!")).toHaveCount(0, { timeout: 15_000 });
  await expect(stone(amina, "F1")).toHaveAttribute("data-state", /in_progress|achieved/);
  await Promise.all([board.close(), phoneA.close(), phoneB.close()]);
});
