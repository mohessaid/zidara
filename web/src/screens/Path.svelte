<script lang="ts">
  // The course page (ADR 0016, in the manner of Brilliant): one column of levels joined by a line.
  // Every colour and lock is the kernel's view of the evidence: green with a tick when the
  // framework says mastered, ink and a Start button for the level to play next, outlined while
  // open, grey with a lock until its rule opens it. Each level is one plain button, so it is light
  // on any phone and readable by any screen reader.
  import { app } from "../lib/app.svelte.ts";
  import { direction, formatDate, t, type Key } from "../lib/i18n/i18n.ts";
  import type { FrameworkId } from "../lib/kernel/types/FrameworkId.ts";
  import type { LearnerView } from "../lib/kernel/types/LearnerView.ts";
  import type { OutcomeView } from "../lib/kernel/types/OutcomeView.ts";
  import { FRAMEWORKS } from "../lib/lens.ts";
  import { stars } from "../lib/stars.ts";
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
  <header class="course">
    <p class="unit-no">{t(app.locale, "path.unit", { n: 1 })}</p>
    <h2 id="unit-title">{app.pack?.title[app.locale] ?? ""}</h2>
    <div class="meter meter-track" data-meter aria-hidden="true"><span style:width={`${(mastered / Math.max(1, view.outcomes.length)) * 100}%`}></span></div>
    <p class="progress">{t(app.locale, "path.oasis", { n: mastered, of: view.outcomes.length })}</p>
  </header>

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

  <ol class="levels">
    {#each view.outcomes as o, i (o.outcome)}
      {@const state = stateOf(o)}
      {@const earned = stars(o.judgement)}
      <li class="level {state}" class:next={o.outcome === next}>
        <button
          class="node"
          data-outcome={o.outcome}
          data-state={state}
          aria-disabled={o.lock ? "true" : undefined}
          onclick={() => tap(o)}
        >
          <span class="disc" aria-hidden="true">
            {#if state === "locked"}<svg viewBox="0 0 24 24" width="22" height="22"><rect x="5" y="11" width="14" height="10" rx="2.5" /><path d="M8 11V8a4 4 0 0 1 8 0v3" /></svg>{:else if state === "achieved"}<svg viewBox="0 0 24 24" width="26" height="26"><path d="M5 12.5l4.5 4.5L19 7.5" /></svg>{:else if state === "needs_support"}↻{:else}{i + 1}{/if}
          </span>
          <span class="words" aria-hidden="true">
            <span class="name">{o.title}</span>
            <span class="state">{o.lock ? lockText(o) : t(app.locale, `state.${o.judgement.state}`)}{#if !o.lock && earned.of > 0}<span class="stars">{#each Array.from({ length: earned.of }, (_, k) => k) as k (k)}<span class:lit={k < earned.n}>{k < earned.n ? "★" : "☆"}</span>{/each}</span>{/if}</span>
          </span>
          {#if o.outcome === next}<span class="start" aria-hidden="true">{t(app.locale, "path.start")}</span>{/if}
          <span class="visually-hidden">{t(app.locale, "path.level", { n: i + 1, name: o.title })}. {t(app.locale, `state.${o.judgement.state}`)}. {lockText(o)}</span>
        </button>
      </li>
    {/each}
    <li class="level goal" class:won>
      <span class="node trophy" role="img" aria-label={t(app.locale, won ? "path.won" : "path.trophy")}>
        <span class="disc" aria-hidden="true">🏆</span>
        <span class="words" aria-hidden="true"><span class="name">{t(app.locale, won ? "path.won" : "path.trophy")}</span></span>
      </span>
    </li>
  </ol>

  <p class="tip" role="status">{told ?? (next ? t(app.locale, "path.tip") : "")}</p>
</section>

<style>
  .unit { max-width: 600px; margin: 0 auto; }
  .course { padding: 8px 0 4px; }
  .unit-no { margin: 0; font-size: 15px; font-weight: 500; color: var(--muted); }
  .course h2 { margin: 2px 0 14px; font-size: clamp(28px, 6vw, 38px); }
  .meter { height: 14px; }
  .meter span { transition: width 500ms cubic-bezier(.2,.8,.2,1); }
  .progress { margin: 8px 0 0; font-size: 15px; color: var(--muted); }

  .lens { margin: 20px 0 0; padding: 14px 16px; border-radius: var(--radius); border: 2px solid var(--line); background: var(--surface); }
  .lens.on { border-color: var(--ink); }
  .lens-title { margin: 0 0 10px; font-size: 15px; font-weight: 600; }
  .switch { display: grid; grid-template-columns: 1fr 1fr; gap: 6px; padding: 4px; border-radius: 12px; background: var(--paper-2); }
  .opt { border: 0; box-shadow: none; background: transparent; padding: 6px 10px; font-size: 16px; line-height: 1.2; display: grid; justify-items: center; border-radius: 9px; }
  .opt[aria-pressed="true"] { background: var(--surface); border: 2px solid var(--ink); box-shadow: none; }
  .opt { border: 2px solid transparent; }
  .yours { font-size: 12.5px; font-weight: 500; color: var(--muted); }
  .lens-note { margin: 10px 0 0; font-size: 15px; line-height: 1.45; color: var(--muted); }
  .lens.on .lens-note { color: var(--ink); }

  /* The levels: one column, each disc joined to the next by a line. */
  .levels { list-style: none; margin: 28px 0 0; padding: 0; display: grid; gap: 0; }
  .level { position: relative; padding-bottom: 18px; }
  .level:not(:last-child)::before { content: ""; position: absolute; inset-inline-start: 31px; top: 64px; bottom: 0; width: 3px; border-radius: 3px; background: var(--line); }
  .level.achieved:not(:last-child)::before { background: var(--achieved); }
  .node {
    width: 100%; display: flex; align-items: center; gap: 16px; padding: 0; min-height: 64px; border: 0; box-shadow: none;
    background: none; text-align: start; border-radius: 16px;
  }
  .node:active:not([aria-disabled="true"]) { transform: none; }
  .node:active:not([aria-disabled="true"]) .disc { transform: translateY(2px); box-shadow: 0 1px 0 var(--disc-edge); }
  .disc {
    --disc: var(--surface); --disc-edge: var(--line-strong); --disc-ink: var(--ink);
    flex: none; width: 64px; height: 64px; border-radius: 50%; display: grid; place-items: center; font-size: 22px; font-weight: 700;
    background: var(--disc); color: var(--disc-ink); border: 2px solid var(--disc-edge); box-shadow: 0 4px 0 var(--disc-edge);
    transition: transform 90ms ease, box-shadow 90ms ease;
  }
  .disc svg { fill: none; stroke: currentColor; stroke-width: 2.6; stroke-linecap: round; stroke-linejoin: round; }
  .achieved .disc { --disc: var(--achieved); --disc-edge: var(--achieved-edge); --disc-ink: #fff; }
  .next .disc { --disc: var(--ink); --disc-edge: #000; --disc-ink: #fff; }
  .needs_support .disc { --disc: var(--try-tint); --disc-edge: var(--try); --disc-ink: var(--try-ink); }
  .locked .disc { --disc: var(--paper-2); --disc-edge: var(--edge); --disc-ink: var(--muted); box-shadow: none; }
  .locked .node { cursor: default; }
  .words { display: grid; gap: 2px; flex: 1; min-width: 0; }
  .name { font-family: var(--display); font-weight: 700; font-size: 18px; line-height: 1.25; }
  .state { font-size: 14.5px; color: var(--muted); display: inline-flex; gap: 8px; align-items: center; flex-wrap: wrap; }
  .locked .name { color: var(--muted); }
  .stars { letter-spacing: 1px; color: var(--muted); }
  .stars .lit { color: var(--star); }
  /* The level to play next: a card around the row, and a Start button inside it. */
  .next .node { padding: 12px; margin-inline: -12px; width: calc(100% + 24px); background: var(--surface); border: 2px solid var(--ink); box-shadow: 0 4px 0 var(--ink); }
  .next::before { inset-inline-start: 31px; }
  .start { flex: none; padding: 10px 18px; border-radius: 12px; background: var(--ink); color: #fff; font-weight: 700; font-size: 16px; }
  .next:not(:last-child)::before { top: 80px; }
  .goal .disc { font-size: 28px; filter: grayscale(1); opacity: 0.85; box-shadow: none; }
  .goal.won .disc { filter: none; opacity: 1; --disc: var(--accent-soft); --disc-edge: var(--accent); }
  .goal .name { color: var(--muted); font-weight: 600; }
  .tip { text-align: center; color: var(--muted); min-height: 1.5em; margin: 16px 0 0; }
  @media (prefers-reduced-motion: no-preference) {
    .level { animation: up 420ms cubic-bezier(.2,.8,.2,1) both; animation-delay: calc(var(--d, 0) * 1ms); }
  }
  @keyframes up { from { transform: translateY(10px); } }
</style>
