<script lang="ts">
  // The landing page's proof, live (ADR 0014): the real kernel, loaded after first paint, judges
  // one learner's same answers under each school's rules. Switching the rules redraws the three
  // levels from the engine's own view; nothing here decides a state by itself.
  import { onMount } from "svelte";
  import { app } from "../lib/app.svelte.ts";
  import { formatDate } from "../lib/i18n/i18n.ts";
  import { LANDING } from "../lib/i18n/landing.ts";
  import type { Showcase } from "../lib/showcase.ts";
  import type { OutcomeView } from "../lib/kernel/types/OutcomeView.ts";
  import { FRAMEWORKS } from "../lib/lens.ts";
  import { t, type Key } from "../lib/i18n/i18n.ts";

  const copy = $derived(LANDING[app.locale]);
  let framework: (typeof FRAMEWORKS)[number] = $state("mastery");
  let views: Showcase | undefined = $state();
  let failed = $state(false);
  const view = $derived(views?.[framework]);
  let worker: Worker | undefined;
  let asked = "";

  onMount(() => {
    // After first paint, in a worker: the page's words come first, and the engine never blocks them.
    const go = () => {
      try {
        worker = new Worker(new URL("./showcase.worker.ts", import.meta.url), { type: "module" });
        worker.onmessage = (event: MessageEvent<{ ok: boolean; views?: Showcase }>) => {
          if (event.data.ok) views = event.data.views;
          else failed = true;
        };
        worker.onerror = () => (failed = true);
        asked = app.locale;
        worker.postMessage(asked);
      } catch {
        failed = true;
      }
    };
    const idle = (globalThis as { requestIdleCallback?: (f: () => void, o?: { timeout: number }) => number }).requestIdleCallback;
    if (idle) idle(go, { timeout: 1500 });
    else setTimeout(go, 300);
    return () => worker?.terminate();
  });

  // The engine words the levels in the visitor's language: ask again when it changes.
  $effect(() => {
    const locale = app.locale;
    if (!worker || locale === asked) return;
    asked = locale;
    worker.postMessage(locale);
  });

  const level = (outcome: string) => Number(outcome.replace(/\D/g, "")) || outcome;

  function status(o: OutcomeView): string {
    if (o.lock?.type === "prerequisite") return copy.live.after.replace("{n}", String(level(o.lock.outcome)));
    if (o.lock?.type === "opens_at") return copy.live.on.replace("{date}", formatDate(app.locale, o.lock.at));
    if (o.judgement.state === "not_started") return copy.live.open;
    const band = o.judgement.band && o.judgement.state !== "achieved" ? ` · ${copy.live.band.replace("{n}", String(o.judgement.band.rank + 1)).replace("{of}", String(o.judgement.band.of))}` : "";
    return copy.live.state[o.judgement.state] + band;
  }

  const tone = (o: OutcomeView) => (o.lock ? "locked" : o.judgement.state);
</script>

<figure class="live" aria-labelledby="live-label">
  <div class="screen">
    <div class="head">
      <p class="label" id="live-label">{copy.live.label}</p>
      <div class="tabs" role="group" aria-labelledby="live-label">
        {#each FRAMEWORKS as f (f)}
          <button type="button" class:on={framework === f} aria-pressed={framework === f} data-framework={f} onclick={() => (framework = f)}>
            {t(app.locale, `lens.${f}` as Key)}
          </button>
        {/each}
      </div>
    </div>
    <p class="story">{copy.live.story}</p>

    {#if view}
      <ol class="levels">
        {#each view.outcomes as o (o.outcome)}
          <li class="lv {tone(o)}" data-outcome={o.outcome} data-state={tone(o)}>
            <svg class="star" viewBox="0 0 40 40" aria-hidden="true">
              <path d="M20 1 25.6 6.4 33.4 6.6 33.6 14.4 39 20 33.6 25.6 33.4 33.4 25.6 33.6 20 39 14.4 33.6 6.6 33.4 6.4 25.6 1 20 6.4 14.4 6.6 6.6 14.4 6.4Z" />
              {#if o.lock}
                <path class="glyph" d="M15 19v-3a5 5 0 0 1 10 0v3M14 19h12v9H14z" />
              {:else if o.judgement.state === "achieved"}
                <path class="glyph" d="m13 20 5 5 9-10" />
              {:else}
                <text x="20" y="25.5" text-anchor="middle">{level(o.outcome)}</text>
              {/if}
            </svg>
            <span class="text">
              <span class="name">{copy.live.level.replace("{n}", String(level(o.outcome)))} <span class="title">{o.title}</span></span>
              <span class="status">{status(o)}</span>
            </span>
          </li>
        {/each}
      </ol>
      <p class="verdict" aria-live="polite">{copy.live.verdict[framework]}</p>
    {:else if failed}
      <p class="verdict">{copy.live.failed}</p>
    {:else}
      <ol class="levels skeleton" aria-busy="true" aria-label={copy.live.loading}>
        <li></li><li></li><li></li>
      </ol>
      <p class="verdict muted">{copy.live.loading}</p>
    {/if}
  </div>
  <figcaption><span class="pulse" aria-hidden="true"></span>{copy.live.engine}</figcaption>
</figure>

<style>
  .live { margin: 0; padding: 14px; border-radius: 20px; background: var(--l-frame); position: relative; }
  .live::before {
    content: ""; position: absolute; inset: 0; border-radius: inherit; pointer-events: none;
    background-image: radial-gradient(var(--l-frame-dot) 1.3px, transparent 1.6px); background-size: 10px 10px; opacity: 0.55;
  }
  .screen { position: relative; background: var(--l-surface); border-radius: 12px; padding: 20px; box-shadow: 0 1px 0 var(--l-line), 0 18px 40px -24px var(--l-shadow); }
  .head { display: flex; flex-wrap: wrap; gap: 10px 16px; align-items: center; justify-content: space-between; }
  .label { margin: 0; font-weight: 600; font-size: 15px; color: var(--l-muted); }
  .tabs { display: inline-flex; padding: 4px; gap: 4px; border-radius: 999px; background: var(--l-tint); }
  .tabs button {
    min-height: 40px; border: 0; border-radius: 999px; padding: 0 16px; background: transparent; color: var(--l-ink);
    font-weight: 600; font-size: 15px; transition: background-color 200ms ease, color 200ms ease, transform 120ms ease;
  }
  .tabs button:active { transform: scale(0.97); }
  .tabs button.on { background: var(--l-accent); color: var(--l-on-accent); }
  .story { margin: 16px 0 12px; color: var(--l-muted); font-size: 15px; line-height: 1.55; }
  .levels { list-style: none; margin: 0; padding: 0; display: grid; gap: 8px; }
  .lv { display: flex; gap: 14px; align-items: center; padding: 10px 12px; border-radius: 12px; border: 1px solid var(--l-line); background: var(--l-bg); transition: border-color 300ms ease, background-color 300ms ease; }
  .star { width: 44px; height: 44px; flex: none; transition: transform 400ms cubic-bezier(.2,1.4,.4,1); }
  .star path:first-child { fill: var(--c); transition: fill 300ms ease; }
  .glyph { fill: none; stroke: #fff; stroke-width: 3; stroke-linecap: round; stroke-linejoin: round; }
  .star text { fill: #fff; font-weight: 700; font-size: 15px; font-family: inherit; }
  .achieved { --c: #e0a100; border-color: color-mix(in srgb, #e0a100 45%, var(--l-line)); background: color-mix(in srgb, #f5b301 10%, var(--l-bg)); }
  .achieved .star { transform: scale(1.08); }
  .achieved .glyph { stroke: #3b2a00; }
  .in_progress { --c: #0b7285; }
  .not_started { --c: #1d7a45; }
  .needs_support { --c: #b4471f; }
  .locked { --c: var(--l-locked); }
  .locked .glyph { stroke: var(--l-muted); }
  .text { display: grid; gap: 2px; min-width: 0; }
  .name { font-weight: 600; font-size: 15px; }
  .title { font-weight: 400; color: var(--l-muted); }
  .status { font-size: 14px; color: var(--l-ink); }
  .verdict { margin: 14px 0 0; padding: 12px 14px; border-radius: 12px; background: var(--l-tint); font-size: 15px; line-height: 1.5; min-height: 4.6em; }
  .verdict.muted { color: var(--l-muted); }
  .skeleton li { height: 66px; border-radius: 12px; background: var(--l-tint); }
  figcaption { position: relative; display: flex; gap: 10px; align-items: center; margin: 12px 4px 0; font-size: 13.5px; color: var(--l-on-frame); }
  .pulse { width: 8px; height: 8px; border-radius: 50%; background: #3ddc84; flex: none; box-shadow: 0 0 0 3px color-mix(in srgb, #3ddc84 30%, transparent); }
  @media (prefers-reduced-motion: reduce) {
    .star, .lv, .tabs button, .star path:first-child { transition: none; }
  }
</style>
