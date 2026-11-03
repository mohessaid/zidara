<script lang="ts">
  // The learner's path across the oasis (ADR 0013), in dots (ADR 0015). Every colour and lock is
  // the kernel's view of the evidence: blue when the framework says mastered (and a palm of dots
  // grows beside it), ink for the level to play next, teal while open, grey and locked until its
  // rule opens it. The levels are dotted tiles, like the front page's pictures. Plain buttons, so it is light on any
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
  const ROW = 196;
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
          <svg class="palm" width="70" height="96" viewBox="-46 -96 92 100" aria-hidden="true"><Palm /></svg>
        {/if}
        {#if o.outcome === next || (won && i === view.outcomes.length - 1)}
          <div class="buddy" class:right={shift(i) < 0} aria-hidden="true"><Mascot size={84} mood={won ? "cheer" : "idle"} still={!won} /></div>
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
  .oasis { position: relative; border-radius: 16px; overflow: hidden; border: 1px solid var(--line); border-bottom-width: 4px; background: var(--surface); }
  .plate {
    position: absolute; inset-inline-start: 0.75rem; top: 0.75rem; padding: 0.4rem 0.9rem 0.5rem; border-radius: 16px;
    background: var(--surface); border: 1px solid var(--line);
  }
  .plate h2 { margin: 0.1rem 0 0; font-size: 21px; line-height: 1.15; }
  .kicker { margin: 0; font: 500 12.5px/1.2 var(--mono); text-transform: uppercase; letter-spacing: 0.06em; color: var(--accent); }
  .progress { margin: 0.1rem 0 0; font-weight: 500; font-size: 15px; color: var(--muted); }

  .lens { margin: 0.9rem 0 0; padding: 0.6rem 0.75rem 0.7rem; border-radius: 16px; background: var(--surface); border: 1px solid var(--line); border-bottom-width: 4px; }
  .lens.on { border-color: var(--accent); background: var(--accent-tint); }
  .lens-title { margin: 0 0 0.4rem; font: 500 12.5px/1.2 var(--mono); text-transform: uppercase; letter-spacing: 0.06em; color: var(--muted); }
  .switch { display: grid; grid-template-columns: 1fr 1fr; gap: 0.4rem; }
  .opt { padding: 0.3rem 0.5rem; font-size: 17px; line-height: 1.2; display: grid; justify-items: center; }
  .opt[aria-pressed="true"] { background: var(--zellige); border-color: var(--zellige); border-bottom-color: var(--zellige-edge); color: #fff; }
  .yours { font: 500 12px/1.2 var(--mono); opacity: 0.9; }
  .lens-note { margin: 0.45rem 0 0; font-size: 15px; line-height: 1.4; color: var(--muted); font-weight: 500; }
  .lens.on .lens-note { color: var(--zellige-edge); }

  .map { position: relative; overflow-x: clip; }
  .levels { position: relative; list-style: none; padding: 2.75rem 0 0.5rem; margin: 0; display: grid; grid-auto-rows: var(--row); justify-items: center; }
  .trail { position: absolute; top: 2.75rem; left: 50%; overflow: visible; }
  .trail path { fill: none; stroke: var(--dune-deep); stroke-width: 7; stroke-linecap: round; stroke-dasharray: 0.5 16; opacity: 0.8; }
  .level { position: relative; display: grid; justify-items: center; align-content: start; gap: 0.1rem; transform: translateX(var(--shift)); }

  /* The level: a dotted tile with a darker copy below it that it sinks into. */
  .node, .trophy { position: relative; width: 92px; height: 98px; padding: 0; border: none; background: none; min-height: 0; display: block; }
  .node:active:not(:disabled) { transform: none; }
  .edge, .face {
    position: absolute; inset-inline: 4px; width: 84px; height: 84px; border-radius: 22px;
  }
  .edge { top: 6px; }
  .face {
    top: 0; display: grid; place-items: center; font: 600 30px/1 var(--mono); color: #fff; transition: transform 80ms ease, background-color 300ms ease;
    background-image: radial-gradient(circle, rgb(255 255 255 / 0.2) 1.3px, transparent 1.7px); background-size: 9px 9px;
  }
  .node:active:not([aria-disabled="true"]) .face { transform: translateY(4px); }
  .not_started .face, .in_progress .face { background-color: var(--in-progress); }
  .not_started .edge, .in_progress .edge { background: var(--blue-edge); }
  .next .face { background-color: var(--go); }
  .next .edge { background: var(--go-edge); }
  /* The level to play next wears a fixed halo: a glow that never stops would cost every frame. */
  .next::before {
    content: ""; position: absolute; inset: -14px -10px -8px; border-radius: 30px; border: 2px dotted var(--accent); background: var(--accent-tint);
  }
  .achieved .face { background-color: var(--accent); color: #fff; }
  .achieved .edge { background: var(--accent-edge); }
  .needs_support .face { background-color: var(--terra); }
  .needs_support .edge { background: var(--terra-edge); }
  .locked .face { background-color: var(--locked-fill); font-size: 26px; background-image: radial-gradient(circle, var(--locked-edge) 1.3px, transparent 1.7px); }
  .locked .edge { background: var(--locked-edge); }
  .locked { cursor: default; }
  .trophy .face { background-color: var(--locked-fill); font-size: 38px; filter: grayscale(1); }
  .trophy .edge { background: var(--locked-edge); }
  .trophy.won .face { background-color: var(--gold); filter: none; }
  .trophy.won .edge { background: var(--gold-edge); }

  .start {
    position: absolute; top: -2.6rem; padding: 0.1rem 0.8rem; border: 1px solid var(--ink); border-bottom-width: 3px; border-radius: 10px; background: #fff;
    color: var(--ink); font: 600 14px/1.5 var(--mono); text-transform: uppercase; letter-spacing: 0.05em; z-index: 1;
  }
  .start::after {
    content: ""; position: absolute; left: 50%; bottom: -9px; width: 12px; height: 12px; margin-left: -6px; background: #fff;
    border-right: 1px solid var(--ink); border-bottom: 1px solid var(--ink); transform: rotate(45deg);
  }
  .label { font-weight: 600; font-size: 16px; max-width: 11rem; text-align: center; line-height: 1.2; padding: 0.05rem 0.5rem; border-radius: 10px; background: var(--paper); }
  .stars { font-size: 18px; color: var(--locked-edge); letter-spacing: 0.1rem; line-height: 1; }
  .stars .lit { color: var(--gold-edge); }
  .palm { position: absolute; inset-inline-end: calc(100% - 0.5rem); top: -0.5rem; pointer-events: none; }
  /* The guide stands on whichever side of the level has room. */
  .buddy { position: absolute; right: calc(50% + 3.2rem); top: 0.75rem; }
  .buddy.right { right: auto; left: calc(50% + 3.2rem); }
  .tip { text-align: center; font-weight: 500; color: var(--muted); min-height: 1.5em; }

  @media (prefers-reduced-motion: no-preference) {
    .start { animation: float 1.6s steps(4) infinite; }
    /* The palm grows as a whole box (composited), not shape by shape inside the SVG. */
    .palm { transform-origin: 50% 100%; animation: sprout 400ms steps(5) both; }
  }
  @keyframes sprout { from { transform: scale(0.1, 0); } }
  @keyframes float { 50% { transform: translateY(-5px); } }
</style>
