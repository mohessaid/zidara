// Experience points and the streak, read from the log: nothing is stored besides the work
// itself, so phone and server always agree. XP: 10 for each question right the first time
// (marked by the kernel), 10 for each right answer in a live round, 5 for each explanation.
// The streak counts days in a row, ending today or yesterday, on which the learner did something
// themselves (a teacher's or the system's marks do not count).
import type { Event } from "../lib/kernel/types/Event.ts";
import type { Response } from "../lib/kernel/types/Response.ts";

export type Check = (activity: string, variant: number, index: number, response: Response) => boolean;

const DAY = 86_400_000;

export function xp(events: Event[], learner: string, check: Check): number {
  let total = 0;
  for (const { body } of events) {
    if (body.type === "quiz_submitted" && body.learner === learner) {
      body.responses.forEach((r, i) => (total += check(body.activity, body.variant, i, r) ? 10 : 0));
    } else if (body.type === "challenge_answered" && body.learner === learner && body.correct) total += 10;
    else if (body.type === "answer_submitted" && body.learner === learner) total += 5;
  }
  return total;
}

export function streak(events: Event[], learner: string, now: number): number {
  const days = new Set(
    events
      .filter((e) => e.actor.kind === "human" && e.actor.user === learner)
      .map((e) => Math.floor(e.at / DAY)),
  );
  const today = Math.floor(now / DAY);
  let day = days.has(today) ? today : today - 1;
  let n = 0;
  while (days.has(day)) {
    n++;
    day--;
  }
  return n;
}
