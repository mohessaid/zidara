<script lang="ts">
  // The end of a session: what the class's framework now says about the outcome (achieved,
  // getting there, or "let's try another way" with the help the kernel assigned), the XP this
  // session earned and the streak. Celebration is short and only for real progress.
  import { onMount } from "svelte";
  import { Tween } from "svelte/motion";
  import { cubicOut } from "svelte/easing";
  import { app } from "../lib/app.svelte.ts";
  import { t } from "../lib/i18n/i18n.ts";
  import { stars } from "../lib/stars.ts";
  import Mascot from "./Mascot.svelte";
  import Palm from "./Palm.svelte";
  import { sound } from "./sound.svelte.ts";
  import { streak, xp } from "./xp.ts";

  let { outcome, gained, onclose }: { outcome: string; gained: number; onclose: () => void } = $props();

  const view = $derived(app.learnerView());
  const info = $derived(view?.outcomes.find((o) => o.outcome === outcome));
  const state = $derived(info?.judgement.state ?? "in_progress");
  const earned = $derived(info ? stars(info.judgement) : { n: 0, of: 0 });
  const events = $derived(app.events());
  const total = $derived(xp(events, app.me!.id, (a, v, i, r) => app.kernel!.check(a, v, i, r)));
  const days = $derived(streak(events, app.me!.id, Date.now()));
  const help = $derived((app.pack?.activities ?? []).filter((a) => a.role === "remediation" && a.outcomes.includes(outcome) && view?.assigned.includes(a.id)));
  const text = (l: Record<string, string>) => l[app.locale] ?? l.en ?? Object.values(l)[0] ?? "";
  const CONFETTI = Array.from({ length: 24 }, (_, i) => i);

  // The total counts up from where it stood before this session: the points land, not appear.
  const still = typeof matchMedia === "function" && matchMedia("(prefers-reduced-motion: reduce)").matches;
  // svelte-ignore state_referenced_locally
  const shown = new Tween(Math.max(0, total - gained), { duration: still ? 0 : 900, easing: cubicOut });

  onMount(() => {
    sound.play(state === "achieved" ? "achieved" : "complete");
    void shown.set(total);
  });
</script>

<section class="finish state-{state}" aria-labelledby="finish-title">
  {#if state === "achieved"}
    <div class="confetti" aria-hidden="true">
      {#each CONFETTI as i (i)}<i style:--x={`${(i * 37) % 100}%`} style:--d={`${(i % 6) * 90}ms`} class="k{i % 4}"></i>{/each}
    </div>
  {/if}
  <div class="emblem" aria-hidden="true">
    {#if state === "achieved"}<svg class="palm" width="110" height="150" viewBox="-40 -96 90 100"><Palm grow /></svg>{/if}
    <Mascot size={150} mood={state === "achieved" ? "cheer" : state === "needs_support" ? "encourage" : "happy"} />
  </div>
  <h2 id="finish-title">{t(app.locale, `finish.${state}`)}</h2>
  {#if info}<p class="outcome">{info.title}</p>{/if}
  {#if state === "achieved"}<p class="grew">🌴 {t(app.locale, "finish.oasis")}</p>{/if}
  {#if earned.of > 0}
    <p class="stars" aria-label={t(app.locale, "stars", { n: earned.n, of: earned.of })}>
      {#each Array.from({ length: earned.of }, (_, i) => i) as i (i)}<span class:lit={i < earned.n} style:--d={`${300 + i * 160}ms`}>★</span>{/each}
    </p>
  {/if}
  <dl class="tally">
    <div class="gold"><dt>{t(app.locale, "finish.xp")}</dt><dd>⚡ +{gained}</dd></div>
    <div class="blue"><dt>{t(app.locale, "finish.total")}</dt><dd><span aria-hidden="true">{Math.round(shown.current)}</span><span class="visually-hidden">{total}</span></dd></div>
    <div class="flame"><dt>{t(app.locale, "finish.streak")}</dt><dd>🔥 {days}</dd></div>
  </dl>
  {#if help.length}
    <p class="help">{t(app.locale, "finish.help", { name: text(help[0]!.title) })}</p>
  {/if}
  <button class="primary big" onclick={onclose}>{t(app.locale, "activity.back")}</button>
</section>

<style>
  .finish { position: relative; flex: 1; display: grid; align-content: center; justify-items: center; gap: 0.5rem; padding: 1rem; text-align: center; overflow: hidden; }
  .emblem { position: relative; display: flex; align-items: flex-end; animation: land 420ms cubic-bezier(.2,1.6,.4,1) both; }
  .palm { margin-inline-end: -1.5rem; }
  .grew { margin: 0; font-weight: 800; color: var(--achieved); font-size: 19px; }
  @media (prefers-reduced-motion: no-preference) { .stars .lit { display: inline-block; animation: star 380ms cubic-bezier(.2,1.8,.4,1) var(--d) both; } }
  @keyframes star { from { transform: scale(2.4) rotate(-30deg); opacity: 0; } }
  h2 { font-size: 34px; margin: 0; color: #c25e00; }
  .state-achieved h2 { color: var(--star); }
  .state-in_progress h2 { color: var(--in-progress); }
  .outcome { font-size: 20px; margin: 0; font-weight: 700; }
  .stars { font-size: 44px; margin: 0; letter-spacing: 0.2rem; color: var(--line); }
  .stars .lit { color: var(--gold); -webkit-text-stroke: 2px var(--gold-edge); }
  .tally { display: flex; flex-wrap: wrap; justify-content: center; gap: 0.75rem; margin: 1rem 0; }
  .tally div { border-radius: var(--radius); border: 2px solid; overflow: hidden; min-width: 6.5rem; background: #fff; }
  dt { font-size: 15px; font-weight: 800; color: #fff; padding: 0.15rem 0.5rem; text-transform: uppercase; letter-spacing: 0.03em; }
  dd { margin: 0; font-size: 26px; font-weight: 800; padding: 0.35rem 0.75rem; }
  .gold { border-color: #e0a800 !important; } .gold dt { background: #e0a800; color: #3d2a00; } .gold dd { color: #8a6100; }
  .blue { border-color: var(--in-progress) !important; } .blue dt { background: var(--in-progress); } .blue dd { color: var(--teal-text); }
  .flame { border-color: #ff9600 !important; } .flame dt { background: #ff9600; color: #3d2400; } .flame dd { color: #b35400; }
  .help { color: var(--needs-support); font-weight: 700; }
  .finish :global(button.big) { width: min(100%, 420px); }
  .confetti { position: absolute; inset: 0; pointer-events: none; }
  .confetti i { position: absolute; top: -12px; left: var(--x); width: 10px; height: 16px; border-radius: 3px; animation: fall 1.6s ease-in var(--d) both; }
  .k0 { background: var(--in-progress); } .k1 { background: #3fb950; } .k2 { background: var(--gold); } .k3 { background: var(--terra); }
  @keyframes fall { to { transform: translateY(110vh) rotate(540deg); opacity: 0.2; } }
  @keyframes land { from { transform: scale(0.3); opacity: 0; } }
</style>
