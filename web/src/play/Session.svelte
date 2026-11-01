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
  import Mascot from "./Mascot.svelte";
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
    <button class="close" onclick={onclose} aria-label={t(app.locale, "session.close")}>✕</button>
    <div class="progress" role="progressbar" aria-valuemin="0" aria-valuemax={total} aria-valuenow={Math.min(done, total)}
      aria-label={t(app.locale, "session.progress")}>
      <span style:width={`${(100 * Math.min(done, total)) / Math.max(total, 1)}%`}></span>
    </div>
    {#if combo >= 2}<span class="combo" aria-label={t(app.locale, "session.combo", { n: combo })}>⚡ {combo}</span>{/if}
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
            <div class="speaker">
              <Mascot size={88} mood="think" />
              <p class="reading bubble">{text(step.kind.body)}</p>
            </div>
            {#if step.kind.audio}<audio controls preload="none" src={`/${step.kind.audio}`}></audio>{/if}
          {:else if step.kind.type === "explore"}
            <div class="speaker">
              <Mascot size={72} mood={explored ? "happy" : "idle"} />
              <p class="reading bubble">{text(step.kind.body)}</p>
            </div>
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
          <span class="badge" aria-hidden="true">{verdict ? "✓" : "↻"}</span>
          <p class="headline">
            {#if verdict}{t(app.locale, PRAISE[(q + at) % PRAISE.length]!)}{:else}{t(app.locale, "feedback.again")}{/if}
          </p>
          {#if !verdict}<p>{t(app.locale, "feedback.answer")} <bdi dir="ltr" class="solution">{answerOf(question)}</bdi></p>{/if}
          {#if lookOf(question).explain}<p>{text(lookOf(question).explain ?? undefined)}</p>{/if}
        </div>
        <button class="primary big" class:blue={!verdict} onclick={nextQuestion} disabled={busy}>{t(app.locale, "session.continue")}</button>
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
  .session { position: fixed; inset: 0; background: var(--paper); display: flex; flex-direction: column; z-index: 10; overflow-y: auto; }
  .bar { display: flex; align-items: center; gap: 0.75rem; padding: 1rem; max-width: 760px; width: 100%; margin: 0 auto; }
  .close, .mute { border: none; background: none; font-size: 26px; padding: 0; color: var(--muted); }
  .close:active, .mute:active { transform: none; }
  .progress { flex: 1; height: 18px; background: var(--line); border-radius: 999px; overflow: hidden; }
  .progress span {
    display: block; height: 100%; background: #3fb950; border-radius: 999px; transition: width 400ms cubic-bezier(.3,1.4,.5,1);
    background-image: linear-gradient(transparent 22%, rgba(255,255,255,0.35) 22%, rgba(255,255,255,0.35) 42%, transparent 42%);
  }
  .combo { font-weight: 800; font-size: 20px; color: #c25e00; animation: pop 300ms ease both; }
  .step { flex: 1; width: 100%; max-width: 760px; margin: 0 auto; padding: 0.5rem 1rem 1.5rem; animation: enter 260ms ease both; }
  .kicker { margin: 0; color: var(--agent); font-size: 16px; font-weight: 800; text-transform: uppercase; letter-spacing: 0.05em; }
  h2 { font-size: 28px; margin: 0.25rem 0 1rem; line-height: 1.25; }
  .reading { font-size: 21px; display: block; }
  .speaker { display: flex; align-items: flex-start; gap: 0.75rem; }
  .speaker :global(.mascot) { flex: none; }
  .bubble {
    position: relative; margin: 0.5rem 0 0; padding: 0.9rem 1.1rem; border: 2px solid var(--line); border-radius: var(--radius);
    background: var(--surface); line-height: 1.55;
  }
  .bubble::before {
    content: ""; position: absolute; inset-inline-start: -10px; top: 1.5rem; width: 16px; height: 16px;
    background: var(--surface); border-inline-start: 2px solid var(--line); border-bottom: 2px solid var(--line); transform: rotate(45deg);
  }
  :global([dir="rtl"]) .bubble::before { transform: rotate(-45deg); }
  video { width: 100%; border-radius: var(--radius); background: #000; }
  textarea { width: 100%; font-size: 20px; border: 2px solid var(--line); border-bottom-width: 4px; border-radius: var(--radius); padding: 0.75rem; background: #f7f7f7; }
  .agent { color: var(--agent); font-weight: 700; }
  .sheet { position: sticky; bottom: 0; padding: 1rem 1rem 1.25rem; border-top: 2px solid var(--line); background: var(--surface); display: grid; gap: 0.75rem; }
  .sheet > * { width: 100%; max-width: 760px; margin: 0 auto; }
  .sheet.right { background: var(--mint); border-color: transparent; animation: rise 220ms ease both; }
  .sheet.again { background: var(--sky); border-color: transparent; animation: rise 220ms ease both; }
  .feedback { display: grid; grid-template-columns: auto 1fr; column-gap: 0.75rem; align-items: center; }
  .feedback > p { grid-column: 2; }
  .badge { grid-row: span 3; width: 56px; height: 56px; border-radius: 50%; background: #fff; display: grid; place-items: center; font-size: 30px; font-weight: 800; }
  .right .badge { color: var(--achieved); }
  .again .badge { color: var(--blue-edge); }
  .feedback p { margin: 0.1rem 0; font-size: 19px; }
  .headline { font-weight: 800; font-size: 26px !important; }
  .right .headline { color: #1a7431; }
  .again .headline { color: var(--blue-edge); }
  .solution { font-weight: 800; }
  :global(button.big) { min-height: 56px; font-size: 20px; }
  @keyframes enter { from { opacity: 0; transform: translateY(10px); } }
  @keyframes rise { from { transform: translateY(100%); } }
  @keyframes pop { from { transform: scale(0.4); } }
</style>
