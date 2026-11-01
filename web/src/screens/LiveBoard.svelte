<script lang="ts">
  import { lookOf } from "../play/look.ts";
  // The teacher's board for a live round, made to be projected: the question, how many have
  // answered, then (when the teacher shows it) how many picked each option and the right answer.
  // The teacher sets the pace. The class plays together: no names, no ranking, no timer.
  // Ending the round turns every learner's answers into practice evidence, in the kernel.
  import { app } from "../lib/app.svelte.ts";
  import { t } from "../lib/i18n/i18n.ts";
  import type { Command } from "../lib/kernel/types/Command.ts";
  import type { LiveTally } from "../lib/kernel/types/LiveTally.ts";
  import Picture from "../play/Picture.svelte";
  import { sound } from "../play/sound.svelte.ts";

  let { tally, learners, busy, run }: { tally: LiveTally | null; learners: number; busy: boolean; run: (c: Command) => Promise<void> } = $props();

  const text = (l: Record<string, string>) => l[app.locale] ?? l.en ?? Object.values(l)[0] ?? "";
  const quizzes = $derived((app.pack?.activities ?? []).filter((a) => a.kind.type === "quiz" && a.role !== "remediation"));
  let pick = $state("");
  $effect(() => {
    if (!pick && quizzes[0]) pick = quizzes[0].id;
  });
  let revealed = $state(false);
  let board: HTMLElement | undefined = $state();
  /** What each question ended with, for the class's result. */
  let seen: Record<number, { answered: number; correct: number }> = $state({});
  let result: { answered: number; correct: number } | undefined = $state();

  const activity = $derived(tally ? app.pack?.activities.find((a) => a.id === tally.activity) : undefined);
  const question = $derived(tally && activity?.kind.type === "quiz" ? activity.kind.questions[tally.question] : undefined);
  const SHAPES = ["●", "▲", "■", "◆"];

  $effect(() => {
    if (tally) seen[tally.question] = { answered: tally.answered, correct: tally.correct };
  });

  async function start() {
    seen = {};
    result = undefined;
    revealed = false;
    sound.play("live");
    await run({ type: "start_challenge", activity: pick });
  }

  async function next() {
    revealed = false;
    sound.play("tap");
    await run({ type: "next_question" });
  }

  async function end() {
    const all = Object.values(seen);
    result = { answered: all.reduce((n, s) => n + s.answered, 0), correct: all.reduce((n, s) => n + s.correct, 0) };
    revealed = false;
    sound.play("achieved");
    await run({ type: "end_challenge" });
  }

  function reveal() {
    revealed = true;
    sound.play("correct");
  }

  const correctIndex = $derived(question?.expected.type === "choice" ? question.expected.correct : -1);
  const most = $derived(Math.max(1, ...(tally?.picks ?? [])));
</script>

<section class="card live" aria-labelledby="live-title" bind:this={board}>
  <div class="head">
    <h2 id="live-title"><span aria-hidden="true">⚡</span> {t(app.locale, "studio.live")}</h2>
    {#if tally && board?.requestFullscreen}
      <button onclick={() => board?.requestFullscreen()}>{t(app.locale, "studio.live.board")}</button>
    {/if}
  </div>

  {#if !tally}
    {#if result}
      <div class="result" role="status">
        <p class="big" aria-hidden="true">🎉</p>
        <p class="headline">{t(app.locale, "studio.live.together", { n: result.correct, of: result.answered })}</p>
        <p>{t(app.locale, "studio.live.ended")}</p>
      </div>
    {/if}
    <div class="row">
      <label>{t(app.locale, "studio.live.pick")}
        <select bind:value={pick}>
          {#each quizzes as q (q.id)}<option value={q.id}>{text(q.title)}</option>{/each}
        </select>
      </label>
      <button class="primary" onclick={start} disabled={busy || !pick}>{t(app.locale, "studio.live.start")}</button>
    </div>
  {:else if question}
    <p class="meta">{t(app.locale, "live.question", { n: tally.question + 1, of: tally.questions })} · <strong aria-live="polite">{t(app.locale, "studio.live.answered", { n: tally.answered, of: learners })}</strong></p>
    <h3 class="prompt">{text(question.prompt)}</h3>
    <Picture visual={lookOf(question).visual} />
    {#if question.expected.type === "choice"}
      <ul class="options">
        {#each question.expected.options as option, k (k)}
          <li class="c{k % 4}" class:right={revealed && k === correctIndex} class:dim={revealed && k !== correctIndex}>
            <span class="shape" aria-hidden="true">{SHAPES[k % 4]}</span>
            <bdi>{text(option)}</bdi>
            {#if revealed}
              <span class="count">{tally.picks[k] ?? 0}</span>
              <span class="meter" style:inline-size={`${(100 * (tally.picks[k] ?? 0)) / most}%`}></span>
            {/if}
          </li>
        {/each}
      </ul>
    {:else if revealed}
      <p class="answer">{t(app.locale, "feedback.answer")} <bdi dir="ltr">{question.expected.num}/{question.expected.den}</bdi></p>
    {/if}
    {#if revealed}<p class="meta">{t(app.locale, "studio.live.right", { n: tally.correct, of: tally.answered })}</p>{/if}
    <div class="row">
      {#if !revealed}<button class="primary" onclick={reveal}>{t(app.locale, "studio.live.reveal")}</button>{/if}
      {#if tally.question + 1 < tally.questions}
        <button onclick={next} disabled={busy}>{t(app.locale, "studio.live.next")}</button>
      {/if}
      <button onclick={end} disabled={busy}>{t(app.locale, "studio.live.end")}</button>
    </div>
  {/if}
</section>

<style>
  .live { display: grid; gap: 0.75rem; }
  .live:fullscreen { background: var(--ink); color: #fff; padding: 3rem; font-size: 28px; align-content: center; }
  .live:fullscreen .prompt { font-size: 48px; }
  .head { display: flex; justify-content: space-between; align-items: center; gap: 1rem; }
  h2 { margin: 0; }
  .meta { margin: 0; }
  .prompt { font-size: 28px; margin: 0; }
  .options { list-style: none; padding: 0; margin: 0; display: grid; grid-template-columns: repeat(auto-fit, minmax(220px, 1fr)); gap: 0.75rem; }
  .options li { position: relative; overflow: hidden; display: flex; align-items: center; gap: 0.75rem; padding: 1rem; border-radius: var(--radius-studio); color: #fff; font-size: 24px; font-weight: 700; min-height: 72px; transition: opacity 200ms ease; }
  .c0 { background: var(--in-progress); } .c1 { background: var(--achieved); } .c2 { background: var(--needs-support); } .c3 { background: var(--agent); }
  /* Not chosen as right: slate, still readable (white on slate 6.0), never faded. */
  .options li.dim { background: var(--locked); }
  .options li.right { outline: 4px solid var(--star); outline-offset: 3px; }
  .options li.right::after { content: "✓"; margin-inline-start: 0.25rem; }
  .count { margin-inline-start: auto; font-size: 28px; }
  .meter { position: absolute; inset-block-end: 0; inset-inline-start: 0; height: 8px; background: rgba(255, 255, 255, 0.85); transition: inline-size 300ms ease; }
  .shape { font-size: 26px; }
  .answer { font-size: 26px; font-weight: 700; }
  .result { text-align: center; }
  .result .big { font-size: 56px; margin: 0; }
  .headline { font-size: 24px; font-weight: 700; margin: 0.25rem 0; }
  select { min-height: 48px; border: 2px solid var(--ink); border-radius: var(--radius-studio); padding: 0 0.5rem; margin-inline-start: 0.5rem; }
</style>
