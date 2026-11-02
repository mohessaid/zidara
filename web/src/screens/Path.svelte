<script lang="ts">
  // The learner's path across the oasis (ADR 0013). Every colour and lock is the kernel's view
  // of the evidence: gold when the framework says mastered (and a palm grows beside it), water
  // blue for the level to play next, sand and locked until its rule opens it. The levels are
  // eight-pointed stars, the khatam of the region's tiles. Plain buttons, so it is light on any
  // phone and readable by any screen reader.
  import { app } from "../lib/app.svelte.ts";
  import { direction, formatDate, t, type Key } from "../lib/i18n/i18n.ts";
  import type { FrameworkId } from "../lib/kernel/types/FrameworkId.ts";
  import type { LearnerView } from "../lib/kernel/types/LearnerView.ts";
  import type { OutcomeView } from "../lib/kernel/types/OutcomeView.ts";
  import { FRAMEWORKS } from "../lib/lens.ts";
  import { stars } from "../lib/stars.ts";
  import Mascot from "../play/Mascot.svelte";
  import Palm from "../play/Palm.svelte";
  import Scene from "../play/Scene.svelte";
  import { sound } from "../play/sound.svelte.ts";

  let {
    view,
    real,
    lens = $bindable(),
    onopen,
  }: { view: LearnerView; real: FrameworkId; lens?: FrameworkId; onopen: (outcome: string) => void } = $props();

  const stateOf = (o: OutcomeView) => (o.lock ? "locked" : o.judgement.state);
  /** The level to play next: the first open one not yet mastered. */
  const next = $derived(view.outcomes.find((o) => !o.lock && o.judgement.state !== "achieved")?.outcome);
  const mastered = $derived(view.outcomes.filter((o) => !o.lock && o.judgement.state === "achieved").length);
  const won = $derived(mastered === view.outcomes.length && mastered > 0);
  const flip = $derived(direction(app.locale) === "rtl" ? -1 : 1);
  const ROW = 172;
  /** The winding: each level sits a little further along a gentle wave. */
  const shift = (i: number) => Math.round(Math.sin((i * Math.PI) / 2.5) * 70) * flip;
  /** The trail between the levels, drawn from the centre line. */
  const trail = $derived.by(() => {
    const n = view.outcomes.length + 1;
    let d = `M ${shift(0)} 46`;
    for (let i = 1; i < n; i++) {
      const y0 = (i - 1) * ROW + 46;
      const y1 = i * ROW + 46;
      d += ` C ${shift(i - 1)} ${y0 + ROW / 2}, ${shift(i)} ${y1 - ROW / 2}, ${shift(i)} ${y1}`;
    }
    return d;
  });
  let told: string | undefined = $state();
  const lensed = $derived(lens !== undefined && lens !== real);

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

  function choose(f: FrameworkId) {
    sound.play("tap");
    lens = f === real ? undefined : f;
  }
</script>

<section class="unit" aria-labelledby="unit-title">
  <div class="oasis">
    <Scene height={170} palms={mastered} of={view.outcomes.length} />
    <div class="plate">
      <p class="kicker">{t(app.locale, "path.unit", { n: 1 })}</p>
      <h2 id="unit-title">{app.pack?.title[app.locale] ?? ""}</h2>
      <p class="progress">🌴 {t(app.locale, "path.oasis", { n: mastered, of: view.outcomes.length })}</p>
    </div>
  </div>

  <div class="lens" class:on={lensed}>
    <p class="lens-title" id="lens-title">{t(app.locale, "lens.title")}</p>
    <div class="switch" role="group" aria-labelledby="lens-title">
      {#each FRAMEWORKS as f (f)}
        <button class="opt" data-framework={f} aria-pressed={(lens ?? real) === f} onclick={() => choose(f)}>
          {t(app.locale, `lens.${f}` as Key)}
          {#if f === real}<span class="yours">{t(app.locale, "lens.yours")}</span>{/if}
        </button>
      {/each}
    </div>
    <p class="lens-note" role="status">
      {#if lensed}{t(app.locale, "lens.note", { name: t(app.locale, `lens.${lens!}` as Key), class: t(app.locale, `lens.${real}` as Key) })}{:else}{t(app.locale, "lens.hint")}{/if}
    </p>
  </div>

  <div class="map">
  <svg class="trail" aria-hidden="true" width="1" height={(view.outcomes.length + 1) * ROW}>
    <path d={trail} />
  </svg>
  <ol class="levels" style:--row={`${ROW}px`}>
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
          <span class="edge" aria-hidden="true"></span>
          <span class="face" aria-hidden="true">
            {#if state === "locked"}🔒{:else if state === "achieved"}✓{:else if state === "needs_support"}↻{:else}{i + 1}{/if}
          </span>
          <span class="visually-hidden">{t(app.locale, "path.level", { n: i + 1, name: o.title })}. {t(app.locale, `state.${o.judgement.state}`)}. {lockText(o)}</span>
        </button>
        <span class="label" aria-hidden="true">{o.title}</span>
        {#if !o.lock && earned.of > 0}
          <span class="stars" aria-hidden="true">{#each Array.from({ length: earned.of }, (_, k) => k) as k (k)}<span class:lit={k < earned.n}>★</span>{/each}</span>
        {/if}
        {#if state === "achieved"}
          <svg class="palm" width="70" height="96" viewBox="-40 -96 90 100" aria-hidden="true"><Palm /></svg>
        {/if}
        {#if o.outcome === next || (won && i === view.outcomes.length - 1)}
          <div class="buddy" aria-hidden="true"><Mascot size={84} mood={won ? "cheer" : "idle"} still={!won} /></div>
        {/if}
      </li>
    {/each}
    <li class="level" style:--shift={`${shift(view.outcomes.length)}px`}>
      <span class="trophy" class:won role="img" aria-label={t(app.locale, won ? "path.won" : "path.trophy")}>
        <span class="edge" aria-hidden="true"></span><span class="face" aria-hidden="true">🏆</span>
      </span>
    </li>
  </ol>
  </div>

  <p class="tip" role="status">{told ?? (next ? t(app.locale, "path.tip") : "")}</p>
</section>

<style>
  .unit { max-width: 560px; margin: 0 auto; }
  .oasis { position: relative; border-radius: 24px; overflow: hidden; border: 3px solid var(--ink); border-bottom-width: 6px; }
  .plate {
    position: absolute; inset-inline-start: 0.75rem; top: 0.75rem; padding: 0.4rem 0.9rem 0.5rem; border-radius: 16px;
    background: rgba(255, 255, 255, 0.94); border: 2px solid var(--ink);
  }
  .plate h2 { margin: 0; font-size: 22px; line-height: 1.15; }
  .kicker { margin: 0; font-weight: 800; font-size: 15px; color: var(--teal-text); }
  .progress { margin: 0.1rem 0 0; font-weight: 700; font-size: 15px; }

  .lens { margin: 0.9rem 0 0; padding: 0.6rem 0.75rem 0.7rem; border-radius: 20px; background: var(--surface); border: 2px solid var(--line); border-bottom-width: 5px; }
  .lens.on { border-color: var(--zellige); background: #eef4ff; }
  .lens-title { margin: 0 0 0.35rem; font-weight: 800; font-size: 16px; }
  .switch { display: grid; grid-template-columns: 1fr 1fr; gap: 0.4rem; }
  .opt { padding: 0.3rem 0.5rem; font-size: 17px; line-height: 1.2; display: grid; justify-items: center; }
  .opt[aria-pressed="true"] { background: var(--zellige); border-color: var(--zellige); border-bottom-color: var(--zellige-edge); color: #fff; }
  .yours { font-size: 13px; font-weight: 700; opacity: 0.9; }
  .lens-note { margin: 0.45rem 0 0; font-size: 15px; line-height: 1.4; color: var(--muted); font-weight: 600; }
  .lens.on .lens-note { color: var(--zellige-edge); }

  .map { position: relative; overflow-x: clip; }
  .levels { position: relative; list-style: none; padding: 2.75rem 0 0.5rem; margin: 0; display: grid; grid-auto-rows: var(--row); justify-items: center; }
  .trail { position: absolute; top: 2.75rem; left: 50%; overflow: visible; }
  .trail path { fill: none; stroke: var(--dune-deep); stroke-width: 7; stroke-linecap: round; stroke-dasharray: 0.5 16; opacity: 0.8; }
  .level { position: relative; display: grid; justify-items: center; align-content: start; gap: 0.1rem; transform: translateX(var(--shift)); }

  /* The level: an eight-pointed star with a darker copy below it that it sinks into. */
  .node, .trophy { position: relative; width: 92px; height: 98px; padding: 0; border: none; background: none; min-height: 0; display: block; }
  .node:active:not(:disabled) { transform: none; }
  .edge, .face {
    position: absolute; inset-inline: 0; width: 92px; height: 92px;
    clip-path: polygon(50% 0%, 65.3% 13%, 85.4% 14.6%, 87% 34.7%, 100% 50%, 87% 65.3%, 85.4% 85.4%, 65.3% 87%, 50% 100%, 34.7% 87%, 14.6% 85.4%, 13% 65.3%, 0% 50%, 13% 34.7%, 14.6% 14.6%, 34.7% 13%);
  }
  .edge { top: 6px; }
  .face { top: 0; display: grid; place-items: center; font-size: 32px; font-weight: 800; color: #fff; transition: transform 80ms ease, background-color 300ms ease; }
  .node:active:not([aria-disabled="true"]) .face { transform: translateY(4px); }
  .not_started .face, .in_progress .face { background: var(--in-progress); }
  .not_started .edge, .in_progress .edge { background: var(--blue-edge); }
  .next .face { background: var(--go); }
  .next .edge { background: var(--go-edge); }
  /* The level to play next wears a fixed halo: a glow that never stops would cost every frame. */
  .next::before {
    content: ""; position: absolute; inset: -12px -12px -6px; border-radius: 50%; background: radial-gradient(closest-side, #c9f0b0 62%, transparent 100%);
  }
  .achieved .face { background: var(--gold); color: var(--ink); }
  .achieved .edge { background: var(--gold-edge); }
  .needs_support .face { background: var(--terra); }
  .needs_support .edge { background: var(--terra-edge); }
  .locked .face { background: var(--locked-fill); font-size: 28px; }
  .locked .edge { background: var(--locked-edge); }
  .locked { cursor: default; }
  .trophy .face { background: var(--locked-fill); font-size: 40px; filter: grayscale(1); }
  .trophy .edge { background: var(--locked-edge); }
  .trophy.won .face { background: var(--gold); filter: none; }
  .trophy.won .edge { background: var(--gold-edge); }

  .start {
    position: absolute; top: -2.5rem; padding: 0.05rem 0.9rem; border: 2px solid var(--ink); border-bottom-width: 4px; border-radius: 12px; background: #fff;
    color: var(--go-edge); font-weight: 800; z-index: 1;
  }
  .start::after {
    content: ""; position: absolute; left: 50%; bottom: -9px; width: 12px; height: 12px; margin-left: -6px; background: #fff;
    border-right: 2px solid var(--ink); border-bottom: 2px solid var(--ink); transform: rotate(45deg);
  }
  .label { font-weight: 800; font-size: 16px; max-width: 11rem; text-align: center; line-height: 1.2; padding: 0.05rem 0.5rem; border-radius: 10px; background: var(--paper); }
  .stars { font-size: 18px; color: var(--locked-edge); letter-spacing: 0.1rem; line-height: 1; }
  .stars .lit { color: var(--gold-edge); }
  .palm { position: absolute; inset-inline-end: calc(100% - 0.5rem); top: -0.5rem; pointer-events: none; }
  .buddy { position: absolute; inset-inline-start: calc(50% + 3.4rem); top: 0.75rem; }
  .tip { text-align: center; font-weight: 700; color: var(--muted); min-height: 1.5em; }

  @media (prefers-reduced-motion: no-preference) {
    .start { animation: float 1.6s ease-in-out infinite; }
    /* The palm grows as a whole box (composited), not shape by shape inside the SVG. */
    .palm { transform-origin: 50% 100%; animation: sprout 400ms cubic-bezier(.2,1.5,.4,1) both; }
  }
  @keyframes sprout { from { transform: scale(0.1, 0); } }
  @keyframes float { 50% { transform: translateY(-5px); } }
  @media (max-width: 400px) { .buddy { inset-inline-start: calc(50% + 2.9rem); } }
</style>
