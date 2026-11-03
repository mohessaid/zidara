<script lang="ts">
  import { lookOf } from "./look.ts";
  // A session on one outcome, a step at a time. Watching, reading and exploring move on with
  // Continue; a quiz is one question per screen, checked at once by the kernel's own marking,
  // and the first try at each question is what is submitted as evidence. The finish screen
  // shows what the class's framework now says, so the game never claims more than the learner
  // can do (art bible: honesty).
  import { app } from "../lib/app.svelte.ts";
  import { t } from "../lib/i18n/i18n.ts";
  import type { Activity } from "../lib/kernel/types/Activity.ts";
  import type { Question } from "../lib/kernel/types/Question.ts";
  import type { Response } from "../lib/kernel/types/Response.ts";
  import Answer from "./Answer.svelte";
  import Finish from "./Finish.svelte";
  import Fold from "./Fold.svelte";
  import Picture from "./Picture.svelte";
  import Wall from "./Wall.svelte";
  import { sound } from "./sound.svelte.ts";
  import { stepsFor } from "./steps.ts";

  let { outcome, onclose }: { outcome: string; onclose: () => void } = $props();

  const text = (l: Record<string, string> | undefined) => (l ? (l[app.locale] ?? l.en ?? Object.values(l)[0] ?? "") : "");
  // The steps are fixed when the session starts; help the kernel assigns on the way is offered at the end.
  // svelte-ignore state_referenced_locally
  const steps: Activity[] = stepsFor(app.pack!, outcome, app.learnerView()?.assigned ?? []);
  const units = (a: Activity) => (a.kind.type === "quiz" ? questionsOf(a).questions.length : 1);
  const total = steps.reduce((n, a) => n + units(a), 0);

  let at = $state(0);
  let q = $state(0);
  let done = $state(0);
  let combo = $state(0);
  let gained = $state(0);
  let finished = $state(false);
  let busy = $state(false);

  // The current quiz: the version the kernel says is due, and the first try at each question.
  let quiz: { variant: number; questions: Question[]; responses: Response[] } | undefined = $state();
  let response: Response | undefined = $state();
  let verdict: boolean | undefined = $state();
  let explored = $state(false);
  let explanation = $state("");
  let sent = $state(false);
  let videoMissing = $state(false);

  const step = $derived(steps[at]);

  function questionsOf(a: Activity) {
    if (a.kind.type !== "quiz") return { variant: 0, questions: [] };
    const variant = app.kernel!.nextVariant(app.me!.id, a.id);
    return { variant, questions: variant === 0 ? a.kind.questions : (a.kind.variants[variant - 1] ?? a.kind.questions) };
  }

  function begin(i: number) {
    at = i;
    q = 0;
    response = verdict = undefined;
    explored = sent = videoMissing = false;
    explanation = "";
    const next = steps[i];
    quiz = next?.kind.type === "quiz" ? { ...questionsOf(next), responses: [] } : undefined;
    if (!next) finish();
  }
  begin(0);

  function finish() {
    finished = true;
  }

  function advance() {
    done++;
    sound.play("tap");
    begin(at + 1);
  }

  function check() {
    if (!quiz || !response || verdict !== undefined) return;
    verdict = app.kernel!.check(step!.id, quiz.variant, q, $state.snapshot(response));
    quiz.responses[q] = response;
    if (verdict) {
      combo++;
      gained += 10;
      sound.play(combo >= 3 ? "combo" : "correct");
    } else {
      combo = 0;
      sound.play("nudge");
    }
  }

  async function nextQuestion() {
    done++;
    if (quiz && q + 1 < quiz.questions.length) {
      q++;
      response = verdict = undefined;
      return;
    }
    busy = true;
    await app.run({ type: "submit_quiz", activity: step!.id, variant: quiz!.variant, responses: $state.snapshot(quiz!.responses) });
    busy = false;
    begin(at + 1);
  }

  async function send() {
    busy = true;
    const result = await app.run({ type: "submit_answer", activity: step!.id, answer: explanation.trim() });
    busy = false;
    if (result.status !== "refused") {
      sent = true;
      gained += 5;
      sound.play("correct");
    }
  }

  /** The right answer, in words a child can read. */
  function answerOf(question: Question): string {
    const e = question.expected;
    if (e.type === "choice") return text(e.options[e.correct]);
    if (e.type === "exactly") return `${e.num}/${e.den}`;
    return t(app.locale, "feedback.equal", { fraction: `${e.num}/${e.den}` });
  }

  const PRAISE = ["feedback.right.1", "feedback.right.2", "feedback.right.3"] as const;
  const question = $derived(quiz?.questions[q]);
</script>

<div class="session" role="dialog" aria-modal="true" aria-labelledby="step-title">
  <header class="bar">
    <button class="close" onclick={onclose} aria-label={t(app.locale, "session.close")}><svg viewBox="0 0 24 24" width="22" height="22" aria-hidden="true"><path d="M6 6l12 12M18 6L6 18" /></svg></button>
    <div class="progress meter-track" role="progressbar" aria-valuemin="0" aria-valuemax={total} aria-valuenow={Math.min(done, total)}
      aria-label={t(app.locale, "session.progress")}>
      <span style:width={`${(100 * Math.min(done, total)) / Math.max(total, 1)}%`}></span>
    </div>
    {#if combo >= 2}<span class="combo" aria-label={t(app.locale, "session.combo", { n: combo })}><span aria-hidden="true">⚡</span> {combo}</span>{/if}
    <button class="mute" onclick={() => sound.toggle()} aria-pressed={sound.on} aria-label={t(app.locale, "session.sound")}>{sound.on ? "🔊" : "🔇"}</button>
  </header>

  {#if finished}
    <Finish {outcome} {gained} onclose={onclose} />
  {:else if step}
    {#key `${at}-${q}`}
      <section class="step" data-variant={quiz?.variant}>
        {#if step.kind.type === "quiz" && question}
          <p class="kicker">{text(step.title)} · <bdi>{q + 1}/{quiz!.questions.length}</bdi></p>
          <h2 id="step-title">{text(question.prompt)}</h2>
          <Picture visual={lookOf(question).visual} />
          <Answer {question} bind:response locked={verdict !== undefined} />
        {:else}
          <p class="kicker">{t(app.locale, `activity.${step.kind.type}`)}</p>
          <h2 id="step-title">{text(step.title)}</h2>
          {#if step.kind.type === "video"}
            {#if videoMissing}
              <p>{t(app.locale, "video.missing")}</p>
            {:else}
              <!-- The film is silent, its words on screen in the learner's language; the transcript is its text alternative. -->
              <!-- svelte-ignore a11y_media_has_caption -->
              <video controls autoplay muted playsinline preload="metadata" src={`/${step.kind.src.replace("{locale}", app.locale)}`} onerror={() => (videoMissing = true)}></video>
            {/if}
            {#if step.kind.transcript}
              <details><summary>{t(app.locale, "video.transcript")}</summary><p>{text(step.kind.transcript)}</p></details>
            {/if}
          {:else if step.kind.type === "reading"}
            <p class="reading">{text(step.kind.body)}</p>
            {#if step.kind.audio}<audio controls preload="none" src={`/${step.kind.audio}`}></audio>{/if}
          {:else if step.kind.type === "explore"}
            <p class="reading">{text(step.kind.body)}</p>
            {#if step.kind.toy.type === "fold"}
              <Fold folds={step.kind.toy.folds} ondone={() => (explored = true)} />
            {:else}
              <Wall rows={step.kind.toy.rows} ondone={() => (explored = true)} />
            {/if}
          {:else if step.kind.type === "open_response"}
            <label for="explain" class="reading">{text(step.kind.prompt)}</label>
            <textarea id="explain" rows="4" placeholder={t(app.locale, "open.placeholder")} bind:value={explanation} disabled={sent}></textarea>
            {#if sent}
              <p class="agent" role="status"><span aria-hidden="true">🤖</span> {t(app.locale, "session.sent")}</p>
            {/if}
          {/if}
        {/if}
      </section>
    {/key}

    <footer class="sheet" class:right={verdict === true} class:again={verdict === false}>
      {#if verdict !== undefined && question}
        <div class="feedback" role="status">
          <span class="badge" aria-hidden="true">
            {#if verdict}<svg viewBox="0 0 24 24" width="26" height="26"><path d="M5 12.5l4.5 4.5L19 7.5" /></svg>{:else}<svg viewBox="0 0 24 24" width="24" height="24"><path d="M12 7v6M12 17v.5" /></svg>{/if}
          </span>
          <p class="headline">
            {#if verdict}{t(app.locale, PRAISE[(q + at) % PRAISE.length]!)}{:else}{t(app.locale, "feedback.again")}{/if}
          </p>
          {#if verdict}<p class="plus">+10 XP</p>{/if}
          {#if !verdict}<p>{t(app.locale, "feedback.answer")} <bdi dir="ltr" class="solution">{answerOf(question)}</bdi></p>{/if}
          {#if lookOf(question).explain}<p>{text(lookOf(question).explain ?? undefined)}</p>{/if}
        </div>
        <button class="primary big" class:go-right={verdict} class:go-again={!verdict} onclick={nextQuestion} disabled={busy}>{t(app.locale, "session.continue")}</button>
      {:else if step.kind.type === "quiz"}
        <button class="primary big" onclick={check} disabled={!response}>{t(app.locale, "session.check")}</button>
      {:else if step.kind.type === "open_response" && !sent}
        <button class="primary big" onclick={send} disabled={explanation.trim() === "" || busy}>{t(app.locale, "open.send")}</button>
      {:else}
        <button class="primary big" onclick={advance} disabled={step.kind.type === "explore" && !explored}>{t(app.locale, "session.continue")}</button>
      {/if}
    </footer>
  {/if}
</div>

<style>
  /* A lesson screen in the manner of Brilliant (ADR 0016): a thin progress bar under a close
     button, one idea in the middle of the screen, and the action pinned at the bottom. After a
     check, the bottom panel turns green or orange and says why. */
  .session { position: fixed; inset: 0; background: var(--paper); display: flex; flex-direction: column; z-index: 10; overflow-y: auto; overflow-x: clip; }
  .bar { display: flex; align-items: center; gap: 14px; padding: 14px 16px; max-width: 720px; width: 100%; margin: 0 auto; }
  .close, .mute { border: none; background: none; box-shadow: none; font-size: 20px; padding: 0; color: var(--muted); border-radius: 50%; }
  .close:hover, .mute:hover { background: var(--paper-2); }
  .close svg { fill: none; stroke: currentColor; stroke-width: 2.4; stroke-linecap: round; }
  .close:active, .mute:active { transform: none; box-shadow: none; }
  .progress { flex: 1; height: 14px; }
  .progress span { transition: width 450ms cubic-bezier(.2,.8,.2,1); }
  .combo { font-weight: 700; font-size: 16px; color: var(--try-ink); animation: pop 280ms cubic-bezier(.2,1.5,.4,1) both; }
  .step { flex: 1; width: 100%; max-width: 640px; margin: 0 auto; padding: 24px 16px 32px; animation: enter 280ms cubic-bezier(.2,.8,.2,1) both; }
  .kicker { margin: 0 0 6px; color: var(--muted); font-size: 15px; font-weight: 500; }
  h2 { font-size: clamp(24px, 5.4vw, 30px); margin: 0 0 20px; line-height: 1.3; font-weight: 700; }
  .reading { font-size: 20px; line-height: 1.6; display: block; margin: 0 0 16px; }
  video { width: 100%; border-radius: var(--radius); background: #000; }
  textarea { width: 100%; font-size: 19px; border: 2px solid var(--edge); border-radius: var(--radius); padding: 14px; background: var(--surface); }
  textarea:focus { border-color: var(--ink); outline: none; }
  .agent { color: var(--agent); font-weight: 700; }
  .sheet { position: sticky; bottom: 0; padding: 16px 16px 20px; border-top: 1px solid var(--line); background: var(--surface); display: grid; gap: 14px; }
  .sheet > * { width: 100%; max-width: 640px; margin: 0 auto; }
  .sheet.right { background: var(--right-tint); border-color: transparent; animation: rise 260ms cubic-bezier(.2,.8,.2,1) both; }
  .sheet.again { background: var(--try-tint); border-color: transparent; animation: rise 260ms cubic-bezier(.2,.8,.2,1) both; }
  .feedback { display: grid; grid-template-columns: auto 1fr; column-gap: 14px; align-items: center; }
  .feedback > p { grid-column: 2; }
  .badge { grid-row: span 4; align-self: start; width: 44px; height: 44px; border-radius: 50%; display: grid; place-items: center; color: #fff; }
  .badge svg { fill: none; stroke: currentColor; stroke-width: 3; stroke-linecap: round; stroke-linejoin: round; }
  .right .badge { background: var(--achieved); }
  .again .badge { background: var(--try); }
  .feedback p { margin: 2px 0; font-size: 18px; }
  .headline { font: 800 24px/1.25 var(--display) !important; }
  .right .headline { color: var(--achieved-edge); }
  .again .headline { color: var(--try-ink); }
  .plus { font-weight: 700; color: var(--achieved-edge); font-size: 15px !important; }
  .solution { font-weight: 700; }
  :global(button.big) { min-height: 56px; font-size: 19px; }
  :global(button.primary.go-right) { background: var(--achieved); border-color: var(--achieved); box-shadow: 0 3px 0 var(--achieved-edge); }
  :global(button.primary.go-again) { background: var(--try-ink); border-color: var(--try-ink); box-shadow: 0 3px 0 #7a2f06; }
  @keyframes enter { from { transform: translateX(24px); } }
  :global([dir="rtl"]) .step { animation-name: enter-rtl; }
  @keyframes enter-rtl { from { transform: translateX(-24px); } }
  @keyframes rise { from { transform: translateY(100%); } }
  @keyframes pop { from { transform: scale(0.4); } }
</style>
