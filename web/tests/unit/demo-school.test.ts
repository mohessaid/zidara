import { beforeAll, describe, expect, it } from "vitest";
import { readFileSync } from "node:fs";
import { Api, ApiError } from "../../src/lib/api.ts";
import { DemoSchool, PEOPLE } from "../../src/lib/demo/school.ts";
import { loadKernel } from "../../src/lib/kernel/kernel.ts";
import type { Envelope } from "../../src/lib/kernel/types/Envelope.ts";
import type { LessonPack } from "../../src/lib/kernel/types/LessonPack.ts";

const pack = JSON.parse(readFileSync(new URL("../../../content/lessons/equivalent-fractions.json", import.meta.url), "utf8")) as LessonPack;
const NOW = 1_791_158_400_000 + 3_600_000;
const id = (name: string): string => PEOPLE.find((p) => p.name === name)!.id;
const [amina, yacine, sara, karim] = [id("amina"), id("yacine"), id("sara"), id("karim")];
const A = "00000000-0000-0000-0000-00000000c00a";

beforeAll(async () => {
  await loadKernel(readFileSync(new URL("../../src/lib/kernel/pkg/zidara_wasm_bg.wasm", import.meta.url)));
});

function memory() {
  const data = new Map<string, string>();
  return { getItem: (k: string) => data.get(k) ?? null, setItem: (k: string, v: string) => void data.set(k, v) };
}

let n = 0;
const envelope = (command: Envelope["command"], at = NOW): Envelope => ({ id: `00000000-0000-4000-8000-${String(++n).padStart(12, "0")}`, at, command });
const explain = (answer: string) => envelope({ type: "submit_answer", activity: "explain-f2", answer });

describe("the demo school in the browser", () => {
  it("signs in only the demo people and says who they are, as the server does", async () => {
    const school = new DemoSchool(pack, memory(), () => NOW);
    await expect(Api.signIn("00000000-0000-0000-0000-000000000bad", school.fetch)).rejects.toEqual(new ApiError(401, "sign in first"));
    await expect(new Api("", school.fetch).me()).rejects.toMatchObject({ status: 401 });
    const me = await (await Api.signIn(amina, school.fetch)).me();
    expect(me).toEqual({
      id: amina,
      display_name: "Amina",
      cohorts: [{ id: A, tenant_id: "00000000-0000-0000-0000-000007e40001", name: "Class 4 A", lesson_id: "equivalent-fractions", framework: "mastery", role: "learner" }],
    });
  });

  it("applies commands as the signed-in person, marks open answers with the rubric, and keeps learners apart", async () => {
    const school = new DemoSchool(pack, memory(), () => NOW);
    const asAmina = await Api.signIn(amina, school.fetch);
    const sent = explain("You multiply the top and bottom by the same number");
    const [result] = await asAmina.commands(A, [{ actor: { kind: "system" }, envelope: sent }]);
    expect(result).toMatchObject({ id: sent.id, outcome: { status: "applied" } });
    // Sent again (a retry), it applies once.
    expect((await asAmina.commands(A, [{ actor: { kind: "system" }, envelope: sent }]))[0]!.outcome).toEqual({ status: "duplicate" });

    // The agent marked it at once: a confident practice mark applies.
    const types = (await asAmina.events(A)).map((e) => e.body.type);
    expect(types).toEqual(["answer_submitted", "proposal_made", "evidence_recorded"]);
    expect((await asAmina.events(A))[1]!.actor).toEqual({ kind: "agent", agent: "assessor", role: "assessment_agent" });

    // Another learner in the class sees none of it; the teacher sees all of it.
    expect(await (await Api.signIn(yacine, school.fetch)).events(A)).toEqual([]);
    const asKarim = await Api.signIn(karim, school.fetch);
    expect((await asKarim.events(A)).length).toBe(3);
    expect((await asKarim.studio(A, "en")).agent_actions.length).toBe(1);
    expect(await asKarim.roster(A)).toEqual([
      { id: amina, display_name: "Amina" },
      { id: yacine, display_name: "Yacine" },
    ]);

    // Learners cannot read the studio; nobody reaches another class.
    await expect(asAmina.studio(A, "en")).rejects.toMatchObject({ status: 403 });
    const asSara = await Api.signIn(sara, school.fetch);
    await expect(asSara.events(A)).rejects.toMatchObject({ status: 403 });
    await expect(asSara.commands(A, [{ actor: { kind: "system" }, envelope: explain("x") }])).rejects.toMatchObject({ status: 403 });
  });

  it("queues an unsure mark for the teacher and refuses what the kernel refuses", async () => {
    const school = new DemoSchool(pack, memory(), () => NOW);
    const asAmina = await Api.signIn(amina, school.fetch);
    await asAmina.commands(A, [{ actor: { kind: "system" }, envelope: explain("you multiply") }]);
    const studio = await (await Api.signIn(karim, school.fetch)).studio(A, "en");
    expect(studio.queue.length).toBe(1);
    const refused = await asAmina.commands(A, [{ actor: { kind: "system" }, envelope: envelope({ type: "change_framework", framework: "cambridge-style" }) }]);
    expect(refused[0]!.outcome.status).toBe("refused");
  });

  it("keeps the school across page loads, and times no command in the future", async () => {
    const storage = memory();
    const later = envelope({ type: "change_framework", framework: "cambridge-style" }, NOW + 86_400_000);
    await (await Api.signIn(karim, new DemoSchool(pack, storage, () => NOW).fetch)).commands(A, [{ actor: { kind: "system" }, envelope: later }]);
    const reloaded = new DemoSchool(pack, storage, () => NOW);
    const events = await (await Api.signIn(karim, reloaded.fetch)).events(A);
    expect(events.map((e) => [e.body.type, e.at])).toEqual([["framework_changed", NOW]]);
    // The phone's retry after the reload is still a duplicate.
    expect((await (await Api.signIn(karim, reloaded.fetch)).commands(A, [{ actor: { kind: "system" }, envelope: later }]))[0]!.outcome).toEqual({ status: "duplicate" });
  });

  it("serves the lesson pack and answers unknown paths with JSON errors", async () => {
    const school = new DemoSchool(pack, memory(), () => NOW);
    const api = await Api.signIn(amina, school.fetch);
    const response = await school.fetch("/api/packs/equivalent-fractions", { headers: { authorization: `Bearer ${api.token}` } });
    expect(((await response.json()) as LessonPack).id).toBe("equivalent-fractions");
    const missing = await school.fetch("/api/nothing", { headers: { authorization: `Bearer ${api.token}` } });
    expect([missing.status, await missing.json()]).toEqual([404, { error: "not found" }]);
  });

  it("plays a live round across two tabs that share the school", async () => {
    const storage = memory();
    const board = await Api.signIn(karim, new DemoSchool(pack, storage, () => NOW).fetch);
    const phone = await Api.signIn(amina, new DemoSchool(pack, storage, () => NOW).fetch);
    // The phone's tab loaded before the round started; it still sees it.
    await phone.events(A);
    await board.commands(A, [{ actor: { kind: "system" }, envelope: envelope({ type: "start_challenge", activity: "quiz-f1" }) }]);
    const seen = (await phone.events(A)).map((e) => e.body.type);
    expect(seen).toEqual(["challenge_started"]);
    const [answered] = await phone.commands(A, [{ actor: { kind: "system" }, envelope: envelope({ type: "answer_challenge", question: 0, response: { type: "choice", index: 0 } }) }]);
    expect(answered!.outcome.status).toBe("applied");
    expect((await board.studio(A, "en")).challenge).toMatchObject({ activity: "quiz-f1", question: 0, answered: 1, correct: 1, picks: [1, 0, 0] });
    await board.commands(A, [{ actor: { kind: "system" }, envelope: envelope({ type: "end_challenge" }) }]);
    const evidence = (await phone.events(A)).filter((e) => e.body.type === "evidence_recorded");
    expect(evidence).toHaveLength(1);
  });
});
