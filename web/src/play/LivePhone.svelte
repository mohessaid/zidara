<script lang="ts">
  import { lookOf } from "./look.ts";
  // A learner's phone during a live round: the board's current question, big tiles, one answer.
  // The kernel marks it at once; the phone says so and waits for the teacher's next question.
  // No timer and no ranking (art bible): the class plays together at the teacher's pace.
  import { app } from "../lib/app.svelte.ts";
  import { t } from "../lib/i18n/i18n.ts";
  import type { LiveQuestion } from "../lib/kernel/types/LiveQuestion.ts";
  import type { Response } from "../lib/kernel/types/Response.ts";
  import Answer from "./Answer.svelte";
  import Picture from "./Picture.svelte";
  import { sound } from "./sound.svelte.ts";

  let { live }: { live: LiveQuestion } = $props();

  const text = (l: Record<string, string>) => l[app.locale] ?? l.en ?? Object.values(l)[0] ?? "";
  const activity = $derived(app.pack?.activities.find((a) => a.id === live.activity));
  const questions = $derived(activity?.kind.type === "quiz" ? activity.kind.questions : []);
  const question = $derived(questions[live.question]);
  let response: Response | undefined = $state();
  let busy = $state(false);
  let refused = $state(false);

  $effect(() => {
    void live.question;
    response = undefined;
    refused = false;
  });

  let announced = false;
  $effect(() => {
    if (!announced) sound.play("live");
    announced = true;
  });

  let last: boolean | null | undefined;
  $effect(() => {
    if (live.answered !== null && last === null) sound.play(live.answered ? "correct" : "nudge");
    last = live.answered;
  });

  async function send() {
    if (!response || busy) return;
    busy = true;
    const result = await app.run({ type: "answer_challenge", question: live.question, response: $state.snapshot(response) });
    busy = false;
    refused = result.status === "refused";
  }
</script>

<div class="live" role="dialog" aria-modal="true" aria-labelledby="live-title">
  <header>
    <p id="live-title" class="title"><span class="pulse" aria-hidden="true"></span>{t(app.locale, "live.title")}</p>
    <p>{t(app.locale, "live.question", { n: live.question + 1, of: questions.length })}</p>
  </header>
  {#if question}
    {#key live.question}
      <section class="q">
        <h2>{text(question.prompt)}</h2>
        <Picture visual={lookOf(question).visual} />
        {#if live.answered === null}
          <Answer {question} bind:response live locked={busy} onchoose={() => void send()} />
          {#if question.expected.type !== "choice"}
            <button class="primary big" onclick={send} disabled={!response || busy}>{t(app.locale, "session.check")}</button>
          {/if}
          {#if refused}<p role="status">{t(app.locale, "live.wait")}</p>{/if}
        {:else}
          <div class="result" class:right={live.answered} role="status">
            <p class="big">{live.answered ? "✓" : "↻"}</p>
            <p class="headline">{t(app.locale, live.answered ? "live.right" : "live.again")}</p>
            <p>{t(app.locale, "live.wait")}</p>
          </div>
        {/if}
      </section>
    {/key}
  {/if}
</div>

<style>
  .live { position: fixed; inset: 0; z-index: 20; background: var(--ink); color: #fff; display: flex; flex-direction: column; overflow-y: auto; }
  header { display: flex; justify-content: space-between; align-items: center; padding: 0.75rem 1rem; }
  header p { margin: 0; font-weight: 700; }
  .title { font-size: 22px; display: flex; align-items: center; gap: 0.5rem; }
  .pulse { width: 14px; height: 14px; border-radius: 50%; background: #ff8a65; animation: pulse 1.2s ease-in-out infinite; }
  .q { flex: 1; width: 100%; max-width: 720px; margin: 0 auto; padding: 1rem; display: grid; gap: 1rem; align-content: start; }
  h2 { font-size: 28px; margin: 0; }
  .q :global(.picture rect), .q :global(.picture path) { stroke: #fff; fill: transparent; }
  .q :global(.picture .on) { fill: #8ab4f8; }
  .q :global(.cell) { background: transparent; border-color: #fff; }
  .q :global(.cell.on) { background: #8ab4f8; }
  .q :global(.count), .q :global(.stepper output) { color: #fff; }
  .result { text-align: center; padding: 2rem 1rem; border-radius: var(--radius); background: rgba(255,255,255,0.08); }
  .result .big { font-size: 80px; margin: 0; }
  .result .headline { font-size: 28px; font-weight: 700; margin: 0.5rem 0; }
  .result.right .big { color: #7ee2a8; }
  @keyframes pulse { 50% { transform: scale(1.5); opacity: 0.5; } }
</style>
