<script lang="ts">
  import { app } from "../lib/app.svelte.ts";
  import { t } from "../lib/i18n/i18n.ts";
  import type { Activity } from "../lib/kernel/types/Activity.ts";
  import type { Response } from "../lib/kernel/types/Response.ts";

  let { activity, ondone }: { activity: Activity; ondone: () => void } = $props();

  const text = (l: Record<string, string>) => l[app.locale] ?? l.en ?? Object.values(l)[0] ?? "";
  let message: string | undefined = $state();
  let busy = $state(false);
  let videoMissing = $state(false);

  // Quiz answers: a choice index, or a fraction typed as two numbers.
  // One activity per component instance (the panel re-creates it), so the version and the
  // answer slots are fixed once, before the first render binds to them. The kernel says which
  // version of a quiz is due; it refuses any other, so retakes are never a memory test.
  // svelte-ignore state_referenced_locally
  const { kind, id: activityId } = activity;
  const variant = kind.type === "quiz" ? app.kernel!.nextVariant(app.me!.id, activityId) : 0;
  const questions = kind.type === "quiz" ? (variant === 0 ? kind.questions : (kind.variants[variant - 1] ?? kind.questions)) : [];
  let answers: { choice?: number; num?: number; den?: number }[] = $state(questions.map(() => ({})));
  const complete = $derived(
    answers.length === questions.length &&
      answers.every((a, i) => (questions[i]!.expected.type === "choice" ? a.choice !== undefined : Number.isInteger(a.num) && Number.isInteger(a.den))),
  );
  let explanation = $state("");

  async function submitQuiz() {
    busy = true;
    const responses: Response[] = answers.map((a, i) =>
      questions[i]!.expected.type === "choice" ? { type: "choice", index: a.choice! } : { type: "fraction", num: a.num!, den: a.den! },
    );
    const result = await app.run({ type: "submit_quiz", activity: activity.id, variant, responses });
    busy = false;
    message = result.status === "refused" ? t(app.locale, "error.generic") : t(app.locale, "quiz.done");
  }

  async function submitAnswer() {
    busy = true;
    const result = await app.run({ type: "submit_answer", activity: activity.id, answer: explanation.trim() });
    busy = false;
    message = result.status === "refused" ? t(app.locale, "error.generic") : t(app.locale, "open.sent");
  }
</script>

<article aria-labelledby="activity-title">
  <h3 id="activity-title">{text(activity.title)}</h3>

  {#if activity.kind.type === "video"}
    {#if videoMissing}
      <p>{t(app.locale, "video.missing")}</p>
    {:else}
      <!-- svelte-ignore a11y_media_has_caption: captions arrive with the video in G5 -->
      <video controls preload="none" src={`/${activity.kind.src}`} onerror={() => (videoMissing = true)}></video>
    {/if}
  {:else if activity.kind.type === "reading"}
    <p class="reading">{text(activity.kind.body)}</p>
    {#if activity.kind.audio}<audio controls preload="none" src={`/${activity.kind.audio}`}></audio>{/if}
  {:else if activity.kind.type === "quiz"}
    <form data-variant={variant} onsubmit={(e) => (e.preventDefault(), submitQuiz())}>
      {#each questions as q, i (q.id)}
        <fieldset>
          <legend>{text(q.prompt)}</legend>
          {#if q.expected.type === "choice"}
            {#each q.expected.options as option, k (k)}
              <label class="choice"><input type="radio" name={q.id} value={k} bind:group={answers[i]!.choice} /> <bdi>{text(option)}</bdi></label>
            {/each}
          {:else}
            <div class="fraction" dir="ltr">
              <label><span class="visually-hidden">{t(app.locale, "quiz.numerator")}</span><input type="number" inputmode="numeric" min="0" bind:value={answers[i]!.num} /></label>
              <span aria-hidden="true" class="bar">/</span>
              <label><span class="visually-hidden">{t(app.locale, "quiz.denominator")}</span><input type="number" inputmode="numeric" min="0" bind:value={answers[i]!.den} /></label>
            </div>
          {/if}
        </fieldset>
      {/each}
      <button class="primary" type="submit" disabled={!complete || busy || message !== undefined}>{t(app.locale, "quiz.check")}</button>
    </form>
  {:else if activity.kind.type === "open_response"}
    <form onsubmit={(e) => (e.preventDefault(), submitAnswer())}>
      <label for="explain">{text(activity.kind.prompt)}</label>
      <textarea id="explain" rows="4" placeholder={t(app.locale, "open.placeholder")} bind:value={explanation}></textarea>
      <button class="primary" type="submit" disabled={explanation.trim() === "" || busy || message !== undefined}>{t(app.locale, "open.send")}</button>
    </form>
  {/if}

  {#if message}
    <p role="status" class="done">{message}</p>
    <button onclick={ondone}>{t(app.locale, "activity.back")}</button>
  {/if}
</article>

<style>
  video { width: 100%; border-radius: var(--radius); }
  .reading { font-size: 20px; }
  fieldset { border: 2px solid #e6dcc8; border-radius: var(--radius); margin: 0 0 1rem; padding: 1rem; }
  legend { font-weight: 700; padding: 0 0.5rem; }
  .choice { display: flex; align-items: center; gap: 0.5rem; min-height: 48px; }
  .choice input { width: 24px; height: 24px; }
  .fraction { display: inline-flex; align-items: center; gap: 0.5rem; }
  .fraction input { width: 5rem; min-height: 48px; font-size: 24px; text-align: center; border: 2px solid var(--ink); border-radius: var(--radius); }
  .bar { font-size: 32px; }
  textarea { width: 100%; font-size: 20px; border: 2px solid var(--ink); border-radius: var(--radius); padding: 0.5rem; display: block; margin: 0.5rem 0; }
  .done { font-size: 22px; color: var(--achieved); font-weight: 700; }
</style>
