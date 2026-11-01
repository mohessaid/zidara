import { describe, expect, it } from "vitest";
import { readFileSync } from "node:fs";
import { stepsFor } from "../../src/play/steps.ts";
import { streak, xp } from "../../src/play/xp.ts";
import type { Event } from "../../src/lib/kernel/types/Event.ts";
import type { LessonPack } from "../../src/lib/kernel/types/LessonPack.ts";

const pack = JSON.parse(readFileSync(new URL("../../../content/lessons/equivalent-fractions.json", import.meta.url), "utf8")) as LessonPack;
const ids = (outcome: string, assigned: string[] = []) => stepsFor(pack, outcome, assigned).map((a) => a.id);

describe("a session's steps", () => {
  it("plays an outcome's activities in the pack's order", () => {
    expect(ids("F1")).toEqual(["video-intro", "reading-halves", "quiz-f1"]);
    expect(ids("F2")).toEqual(["reading-halves", "explore-fold", "quiz-f2", "explain-f2"]);
  });

  it("asks the checkpoint once, with the last outcome it covers", () => {
    expect(ids("F3")).toEqual(["explore-wall", "quiz-f3", "checkpoint"]);
    expect(ids("F1")).not.toContain("checkpoint");
  });

  it("adds help only when the kernel assigned it", () => {
    expect(ids("F2", ["remedial-f2"])).toContain("remedial-f2");
    expect(ids("F2")).not.toContain("remedial-f2");
  });
});

const AMINA = "a";
const at = (day: number, hour = 9) => day * 86_400_000 + hour * 3_600_000;
let n = 0;
const ev = (body: Event["body"], when: number, actor: Event["actor"] = { kind: "human", user: AMINA, role: "learner" }): Event => ({
  id: String(++n),
  at: when,
  tenant: "t",
  cohort: "c",
  actor,
  body,
});

describe("XP and the streak", () => {
  const quiz = ev({ type: "quiz_submitted", learner: AMINA, activity: "quiz-f1", variant: 0, responses: [{ type: "choice", index: 0 }, { type: "fraction", num: 1, den: 2 }] }, at(10));
  const answer = ev({ type: "answer_submitted", learner: AMINA, activity: "explain-f2", answer: "x" }, at(11));
  const live = (correct: boolean) => ev({ type: "challenge_answered", challenge: "r", learner: AMINA, question: 0, response: { type: "choice", index: 0 }, correct }, at(12));
  const other = ev({ type: "answer_submitted", learner: "b", activity: "explain-f2", answer: "x" }, at(13), { kind: "human", user: "b", role: "learner" });

  it("counts what the kernel marks right, live answers and explanations, for this learner only", () => {
    // The kernel says the first response is right and the second is not.
    const check = (activity: string, variant: number, index: number) => activity === "quiz-f1" && variant === 0 && index === 0;
    expect(xp([quiz, answer, live(true), live(false), other], AMINA, check)).toBe(10 + 5 + 10);
    expect(xp([], AMINA, check)).toBe(0);
  });

  it("counts days in a row of the learner's own work, ending today or yesterday", () => {
    const work = (day: number) => ev({ type: "answer_submitted", learner: AMINA, activity: "explain-f2", answer: "x" }, at(day));
    const log = [work(10), work(11), work(11), work(12)];
    expect(streak(log, AMINA, at(12, 20))).toBe(3);
    expect(streak(log, AMINA, at(13, 8))).toBe(3);
    expect(streak(log, AMINA, at(14))).toBe(0);
    expect(streak([work(10), work(12)], AMINA, at(12))).toBe(1);
    // The system's and the teacher's marks, and other learners' work, do not keep a streak alive.
    const assigned = ev({ type: "activity_assigned", learner: AMINA, outcome: "F2", activity: "remedial-f2" }, at(13), { kind: "system" });
    const marked = ev({ type: "evidence_recorded", evidence: { learner: AMINA, outcome: "F1", activity: "quiz-f1", score: 9000, kind: "formative" } }, at(13), { kind: "human", user: "t", role: "teacher" });
    expect(streak([...log, assigned, marked, other], AMINA, at(13, 20))).toBe(3);
  });
});
