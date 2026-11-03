<script lang="ts">
  // The front page's proof, live (ADR 0015): the real kernel, compiled to WebAssembly and run in
  // a worker after first paint, judges one learner's same answers under the framework chosen.
  // The experience switch then draws that one view four ways. Every state shown comes from the
  // engine's view; nothing here decides one by itself (the view-intent contract, rule 7).
  import { onMount } from "svelte";
  import { app } from "../lib/app.svelte.ts";
  import { formatDate, t, type Key } from "../lib/i18n/i18n.ts";
  import { LANDING } from "../lib/i18n/landing.ts";
  import type { Showcase } from "../lib/showcase.ts";
  import type { OutcomeView } from "../lib/kernel/types/OutcomeView.ts";
  import { FRAMEWORKS } from "../lib/lens.ts";

  const EXPERIENCES = ["path", "grid", "studio", "text"] as const;
  type Experience = (typeof EXPERIENCES)[number];

  const copy = $derived(LANDING[app.locale]);
  let framework: (typeof FRAMEWORKS)[number] = $state("mastery");
  let experience: Experience = $state("path");
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

  // The engine words the outcomes in the visitor's language: ask again when it changes.
  $effect(() => {
    const locale = app.locale;
    if (!worker || locale === asked) return;
    asked = locale;
    worker.postMessage(locale);
  });

  const level = (outcome: string) => Number(outcome.replace(/\D/g, "")) || outcome;
  const tone = (o: OutcomeView) => (o.lock ? "locked" : o.judgement.state);
  const percent = (o: OutcomeView) => (o.judgement.score === null ? "" : `${Math.round(Number(o.judgement.score) / 100)}%`);
  const bandOf = (o: OutcomeView) => (o.judgement.band ? copy.live.band.replace("{n}", String(o.judgement.band.rank + 1)).replace("{of}", String(o.judgement.band.of)) : "");

  function status(o: OutcomeView): string {
    if (o.lock?.type === "prerequisite") return copy.live.after.replace("{n}", String(level(o.lock.outcome)));
    if (o.lock?.type === "opens_at") return copy.live.on.replace("{date}", formatDate(app.locale, o.lock.at));
    if (o.judgement.state === "not_started") return copy.live.open;
    return copy.live.state[o.judgement.state];
  }
  const name = (o: OutcomeView) => copy.live.level.replace("{n}", String(level(o.outcome)));
</script>

<figure class="console" aria-labelledby="engine-label">
  <div class="screen">
    <div class="bar">
      <span class="lights" aria-hidden="true"><i></i><i></i><i></i></span>
      <p class="label" id="engine-label">{copy.live.label}</p>
      <span class="live-tag"><span class="pulse" aria-hidden="true"></span>{copy.live.tag}</span>
    </div>

    <div class="controls">
      <div class="axis">
        <p class="axis-name" id="fw-name">{copy.live.frameworkAxis}</p>
        <div class="tabs" role="group" aria-labelledby="fw-name">
          {#each FRAMEWORKS as f (f)}
            <button type="button" class:on={framework === f} aria-pressed={framework === f} data-framework={f} onclick={() => (framework = f)}>
              {t(app.locale, `lens.${f}` as Key)}
            </button>
          {/each}
        </div>
      </div>
      <div class="axis">
        <p class="axis-name" id="ex-name">{copy.live.experienceAxis}</p>
        <div class="tabs" role="group" aria-labelledby="ex-name">
          {#each EXPERIENCES as e (e)}
            <button type="button" class:on={experience === e} aria-pressed={experience === e} data-experience={e} onclick={() => (experience = e)}>
              {copy.live.experiences[e]}
            </button>
          {/each}
        </div>
      </div>
    </div>

    <p class="story">{copy.live.story}</p>

    <div class="stage" data-showing={experience}>
      {#if view}
        {#if experience === "path"}
          <ol class="path">
            {#each view.outcomes as o (o.outcome)}
              <li class="stone {tone(o)}" data-outcome={o.outcome} data-state={tone(o)}>
                <span class="dot" aria-hidden="true">{o.judgement.state === "achieved" && !o.lock ? "✓" : level(o.outcome)}</span>
                <span class="text"><strong>{name(o)}</strong> <span class="muted">{o.title}</span><span class="status">{status(o)}</span></span>
              </li>
            {/each}
          </ol>
        {:else if experience === "grid"}
          <table class="grid">
            <thead><tr><th scope="col">{copy.live.cols.outcome}</th><th scope="col">{copy.live.cols.status}</th><th scope="col">{copy.live.cols.score}</th><th scope="col">{copy.live.cols.band}</th></tr></thead>
            <tbody>
              {#each view.outcomes as o (o.outcome)}
                <tr class={tone(o)} data-outcome={o.outcome} data-state={tone(o)}>
                  <th scope="row"><span class="code" lang="en" dir="ltr">{o.outcome}</span> {o.title}</th>
                  <td><span class="chip">{status(o)}</span></td>
                  <td class="num">{percent(o) || "·"}</td>
                  <td>{bandOf(o) || "·"}</td>
                </tr>
              {/each}
            </tbody>
          </table>
        {:else if experience === "studio"}
          <div class="studio">
            <p class="studio-head">{copy.live.studioHead}</p>
            <ul>
              {#each view.outcomes as o (o.outcome)}
                <li class={tone(o)} data-outcome={o.outcome} data-state={tone(o)}>
                  <span class="bar-mark" aria-hidden="true"></span>
                  <span><strong>{o.title}</strong>: {status(o)}{o.judgement.attempts ? ` · ${copy.live.attempts.replace("{n}", String(o.judgement.attempts))}` : ""}</span>
                </li>
              {/each}
            </ul>
          </div>
        {:else}
          <div class="sms">
            <p class="sms-from" lang="en" dir="ltr">ZIDARA</p>
            <p class="bubble">
              {#each view.outcomes as o, i (o.outcome)}<span data-outcome={o.outcome} data-state={tone(o)}>{name(o)}: {status(o)}{i < view.outcomes.length - 1 ? ". " : "."}</span>{/each}
            </p>
            <p class="sms-note">{copy.live.smsNote}</p>
          </div>
        {/if}
        <p class="verdict" aria-live="polite">{copy.live.verdict[framework]}</p>
      {:else if failed}
        <p class="verdict">{copy.live.failed}</p>
      {:else}
        <ol class="path skeleton" aria-busy="true" aria-label={copy.live.loading}><li></li><li></li><li></li></ol>
        <p class="verdict muted">{copy.live.loading}</p>
      {/if}
    </div>
  </div>
  <figcaption>{copy.live.engine}</figcaption>
</figure>

<style>
  /* A plain white window with one grey edge and a soft shadow (ADR 0017): no gradient frame. */
  .console { margin: 0; border: 1px solid var(--l-raise); border-radius: 20px; background: var(--l-bg); position: relative; box-shadow: 0 30px 70px -40px rgb(15 15 16 / 0.3); overflow: hidden; }
  .screen { background: var(--l-bg); padding: 16px 18px 18px; text-align: start; }
  .bar { display: flex; align-items: center; gap: 12px; padding-bottom: 12px; border-bottom: 1px solid var(--l-line); }
  .lights { display: inline-flex; gap: 6px; }
  .lights i { width: 10px; height: 10px; border-radius: 50%; background: var(--l-raise); }
  .label { margin: 0; font: 500 13px/1 var(--l-mono); color: var(--l-muted); }
  .live-tag { margin-inline-start: auto; display: inline-flex; gap: 8px; align-items: center; font: 500 12.5px/1 var(--l-mono); color: var(--l-muted); }
  .pulse { width: 8px; height: 8px; border-radius: 50%; background: #17773f; }

  .controls { display: grid; gap: 10px; margin: 14px 0 6px; }
  .axis { display: flex; flex-wrap: wrap; align-items: center; gap: 8px 12px; }
  .axis-name { margin: 0; min-width: 7.5em; font: 500 12.5px/1.2 var(--l-mono); color: var(--l-muted); text-transform: uppercase; letter-spacing: 0.04em; }
  .tabs { display: inline-flex; flex-wrap: wrap; gap: 4px; padding: 4px; border-radius: 12px; background: var(--l-surface); }
  .tabs button {
    min-height: 40px; border: 0; border-radius: 9px; padding: 0 12px; background: transparent; color: var(--l-ink);
    font: inherit; font-weight: 500; font-size: 14.5px; cursor: pointer; transition: background-color 160ms ease, color 160ms ease, transform 120ms ease;
  }
  .tabs button:hover { background: var(--l-raise); }
  .tabs button:active { transform: scale(0.97); }
  .tabs button.on { background: var(--l-ink); color: #fff; }
  .story { margin: 10px 0 12px; color: var(--l-muted); font-size: 15px; line-height: 1.55; }
  .stage { min-height: 268px; }
  .muted { color: var(--l-muted); font-weight: 400; }

  /* The children's path. */
  .path { list-style: none; margin: 0; padding: 0; display: grid; gap: 8px; }
  .stone { display: flex; gap: 14px; align-items: center; padding: 10px 12px; border-radius: 12px; border: 1px solid var(--l-raise); background: var(--l-bg); }
  .dot { width: 38px; height: 38px; flex: none; display: grid; place-items: center; border-radius: 50%; font-weight: 700; color: #fff; background: var(--c); }
  .text { display: grid; gap: 1px; font-size: 15px; }
  .status { font-size: 14px; }
  .achieved { --c: var(--l-accent); }
  .stone.achieved { border-color: var(--l-ink); }
  .in_progress { --c: #0b7285; }
  .not_started { --c: #1d7a45; }
  .needs_support { --c: #b4471f; }
  .locked { --c: var(--l-locked); }
  .stone.locked .dot { color: var(--l-ink); }
  .stone.achieved .dot { color: var(--l-on-accent); }

  /* The secondary grid. */
  .grid { width: 100%; border-collapse: collapse; font-size: 14.5px; }
  .grid th, .grid td { text-align: start; padding: 10px 8px; border-bottom: 1px solid var(--l-line); vertical-align: middle; }
  .grid thead th { font: 500 12px/1 var(--l-mono); color: var(--l-muted); text-transform: uppercase; letter-spacing: 0.04em; }
  .grid tbody th { font-weight: 500; }
  .code { font: 600 12.5px/1 var(--l-mono); color: var(--l-accent-ink); margin-inline-end: 4px; }
  /* The state in words, after a small square of its colour: no badge. */
  .chip { display: inline-flex; align-items: center; gap: 8px; font-size: 14px; }
  .chip::before { content: ""; width: 10px; height: 10px; border-radius: 2px; background: var(--c); flex: none; }
  .num { font-variant-numeric: tabular-nums; font-family: var(--l-mono); }

  /* The teacher's studio. */
  .studio-head { margin: 0 0 8px; font: 500 12.5px/1.3 var(--l-mono); color: var(--l-muted); text-transform: uppercase; letter-spacing: 0.04em; }
  .studio ul { list-style: none; margin: 0; padding: 0; display: grid; gap: 6px; }
  .studio li { display: flex; gap: 10px; align-items: stretch; padding: 10px 12px; border-radius: 10px; background: var(--l-bg); border: 1px solid var(--l-raise); font-size: 15px; }
  .bar-mark { width: 4px; flex: none; border-radius: 2px; background: var(--c); }

  /* A text message for a basic phone. */
  .sms { max-width: 360px; }
  .sms-from { margin: 0 0 6px; font: 600 12px/1 var(--l-mono); color: var(--l-muted); letter-spacing: 0.08em; }
  .bubble { margin: 0; padding: 12px 14px; border-radius: 16px 16px 16px 4px; background: var(--l-surface); font: 400 14.5px/1.55 var(--l-mono); }
  .sms-note { margin: 8px 0 0; font-size: 13.5px; color: var(--l-muted); }

  .verdict { margin: 14px 0 0; padding: 12px 14px; border-radius: 12px; background: var(--l-surface); font-size: 15px; line-height: 1.5; }
  .verdict.muted { color: var(--l-muted); }
  .skeleton li { height: 60px; border-radius: 12px; background: var(--l-tint); }
  figcaption { margin: 0; padding: 12px 16px 14px; font: 500 13px/1.5 var(--l-mono); color: var(--l-muted); text-align: center; background: var(--l-surface); border-top: 1px solid var(--l-line); }
  @media (max-width: 720px) {
    .screen { padding: 14px; }
    .axis-name { min-width: 0; width: 100%; }
    .tabs button { padding: 0 10px; font-size: 14px; }
    .grid th, .grid td { padding: 8px 4px; }
    .grid thead th:nth-child(4), .grid td:nth-child(4) { display: none; }
  }
  @media (prefers-reduced-motion: reduce) {
    .tabs button { transition: none; }
  }
</style>
