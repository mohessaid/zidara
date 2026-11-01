<script lang="ts">
  // The learner's path: one unit per course, a level per outcome, winding down the screen.
  // Every colour and lock is the kernel's view of the evidence (ADR 0011): gold when the class's
  // framework says mastered, green for the level to play next, grey and locked until its rule
  // opens it. Plain buttons, so it is light on any phone and readable by any screen reader.
  import { app } from "../lib/app.svelte.ts";
  import { direction, formatDate, t } from "../lib/i18n/i18n.ts";
  import type { LearnerView } from "../lib/kernel/types/LearnerView.ts";
  import type { OutcomeView } from "../lib/kernel/types/OutcomeView.ts";
  import { stars } from "../lib/stars.ts";
  import CourseIcon from "../play/CourseIcon.svelte";
  import Mascot from "../play/Mascot.svelte";
  import { sound } from "../play/sound.svelte.ts";

  let { view, onopen }: { view: LearnerView; onopen: (outcome: string) => void } = $props();

  const stateOf = (o: OutcomeView) => (o.lock ? "locked" : o.judgement.state);
  /** The level to play next: the first open one not yet mastered. */
  const next = $derived(view.outcomes.find((o) => !o.lock && o.judgement.state !== "achieved")?.outcome);
  const mastered = $derived(view.outcomes.filter((o) => !o.lock && o.judgement.state === "achieved").length);
  const won = $derived(mastered === view.outcomes.length && mastered > 0);
  const flip = $derived(direction(app.locale) === "rtl" ? -1 : 1);
  /** The winding: each level sits a little further along a gentle wave. */
  const shift = (i: number) => Math.round(Math.sin((i * Math.PI) / 2.5) * 72) * flip;
  let told: string | undefined = $state();

  function lockText(o: OutcomeView): string {
    const lock = o.lock;
    if (!lock) return "";
    if (lock.type === "prerequisite") {
      const needed = view.outcomes.find((x) => x.outcome === lock.outcome);
      return t(app.locale, "lock.prerequisite", { name: needed?.title ?? lock.outcome });
    }
    return t(app.locale, "lock.opens_at", { date: formatDate(app.locale, lock.at) });
  }

  function tap(o: OutcomeView) {
    if (o.lock) {
      told = lockText(o);
      sound.play("nudge");
      return;
    }
    told = undefined;
    sound.play("tap");
    onopen(o.outcome);
  }
</script>

<section class="unit" aria-labelledby="unit-title">
  <div class="banner">
    <div>
      <p class="kicker">{t(app.locale, "path.unit", { n: 1 })}</p>
      <h2 id="unit-title">{app.pack?.title[app.locale] ?? ""}</h2>
      <p class="progress">{t(app.locale, "path.mastered", { n: mastered, of: view.outcomes.length })}</p>
    </div>
    <CourseIcon lesson={app.pack?.id ?? ""} size={64} />
  </div>

  <ol class="levels">
    {#each view.outcomes as o, i (o.outcome)}
      {@const state = stateOf(o)}
      {@const earned = stars(o.judgement)}
      <li class="level" style:--shift={`${shift(i)}px`}>
        {#if o.outcome === next}
          <span class="start" aria-hidden="true">{t(app.locale, "path.start")}</span>
        {/if}
        <button
          class="node {state}"
          class:next={o.outcome === next}
          data-outcome={o.outcome}
          data-state={state}
          aria-disabled={o.lock ? "true" : undefined}
          onclick={() => tap(o)}
        >
          <span class="icon" aria-hidden="true">
            {#if state === "locked"}🔒{:else if state === "achieved"}✓{:else if state === "needs_support"}↻{:else}★{/if}
          </span>
          <span class="visually-hidden">{t(app.locale, "path.level", { n: i + 1, name: o.title })}. {t(app.locale, `state.${o.judgement.state}`)}. {lockText(o)}</span>
        </button>
        <span class="label" aria-hidden="true">{o.title}</span>
        {#if !o.lock && earned.of > 0}
          <span class="stars" aria-hidden="true">{#each Array.from({ length: earned.of }, (_, k) => k) as k (k)}<span class:lit={k < earned.n}>★</span>{/each}</span>
        {/if}
        {#if i === 0}
          <div class="buddy" aria-hidden="true"><Mascot size={96} mood={won ? "cheer" : "idle"} /></div>
        {/if}
      </li>
    {/each}
    <li class="level" style:--shift={`${shift(view.outcomes.length)}px`}>
      <span class="trophy" class:won role="img" aria-label={t(app.locale, won ? "path.won" : "path.trophy")}>🏆</span>
    </li>
  </ol>

  <p class="tip" role="status">{told ?? (next ? t(app.locale, "path.tip") : "")}</p>
</section>

<style>
  .unit { max-width: 560px; margin: 0 auto; }
  .banner {
    display: flex; align-items: center; justify-content: space-between; gap: 1rem; padding: 1rem 1.25rem;
    border-radius: 20px; color: #fff; background: var(--go-edge); border-bottom: 5px solid #135726;
  }
  .banner h2 { margin: 0; font-size: 24px; color: #fff; }
  .kicker { margin: 0; font-weight: 800; text-transform: uppercase; letter-spacing: 0.06em; font-size: 16px; opacity: 0.95; }
  .progress { margin: 0; font-weight: 700; font-size: 16px; }
  .levels { list-style: none; padding: 3.25rem 0 0.5rem; margin: 0; display: grid; gap: 1.75rem; justify-items: center; }
  .level { position: relative; display: grid; justify-items: center; gap: 0.2rem; transform: translateX(var(--shift)); }
  .node {
    width: 84px; height: 78px; min-height: 0; border-radius: 50%; padding: 0; display: grid; place-items: center;
    border: none; border-bottom: 8px solid; font-size: 34px; color: #fff; position: relative;
  }
  .node:active:not([aria-disabled="true"]) { transform: translateY(4px); border-bottom-width: 4px; }
  .node.not_started, .node.in_progress { background: var(--in-progress); border-bottom-color: var(--blue-edge); }
  .node.next { background: var(--go); border-bottom-color: var(--go-edge); box-shadow: 0 0 0 7px #fff, 0 0 0 12px #c9f0b0; animation: glow 1.8s ease-in-out infinite; }
  .node.achieved { background: var(--gold); border-bottom-color: var(--gold-edge); color: #6b4a00; }
  .node.needs_support { background: #ff9600; border-bottom-color: #c76a00; }
  .node.locked { background: var(--locked-fill); border-bottom-color: var(--locked-edge); font-size: 28px; cursor: default; }
  .icon { line-height: 1; font-weight: 800; }
  .start {
    position: absolute; top: -2.6rem; padding: 0.15rem 0.9rem; border: 2px solid var(--line); border-radius: 12px; background: #fff;
    color: var(--go-edge); font-weight: 800; text-transform: uppercase; letter-spacing: 0.05em; animation: float 1.6s ease-in-out infinite; z-index: 1;
  }
  .start::after {
    content: ""; position: absolute; left: 50%; bottom: -8px; width: 12px; height: 12px; margin-left: -6px; background: #fff;
    border-right: 2px solid var(--line); border-bottom: 2px solid var(--line); transform: rotate(45deg);
  }
  .label { font-weight: 700; font-size: 16px; max-width: 12rem; text-align: center; line-height: 1.25; margin-top: 0.6rem; }
  .stars { font-size: 18px; color: var(--line); letter-spacing: 0.1rem; }
  .stars .lit { color: var(--gold-edge); }
  .buddy { position: absolute; inset-inline-start: calc(100% + 1.5rem); top: 3.5rem; }
  .trophy { font-size: 56px; filter: grayscale(1) opacity(0.45); }
  .trophy.won { filter: none; animation: float 1.6s ease-in-out infinite; }
  .tip { text-align: center; font-weight: 700; color: var(--muted); min-height: 1.5em; }
  @keyframes glow { 50% { box-shadow: 0 0 0 7px #fff, 0 0 0 16px #e3f8d4; } }
  @keyframes float { 50% { transform: translateY(-6px); } }
  @media (max-width: 420px) { .buddy { inset-inline-start: calc(100% + 0.5rem); } }
</style>
