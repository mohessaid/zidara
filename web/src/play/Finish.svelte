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
  <div class="emblem" aria-hidden="true">
    {#if state === "achieved"}<svg viewBox="0 0 24 24" width="56" height="56"><path d="M5 12.5l4.5 4.5L19 7.5" /></svg>{:else if state === "needs_support"}<svg viewBox="0 0 24 24" width="50" height="50"><path d="M19 12a7 7 0 1 1-2.05-4.95M19 5v4h-4" /></svg>{:else}<svg viewBox="0 0 24 24" width="52" height="52"><path d="M5 12h14M13 6l6 6-6 6" /></svg>{/if}
  </div>
  <h2 id="finish-title">{t(app.locale, `finish.${state}`)}</h2>
  {#if info}<p class="outcome">{info.title}</p>{/if}
  {#if earned.of > 0}
    <p class="stars" aria-label={t(app.locale, "stars", { n: earned.n, of: earned.of })}>
      {#each Array.from({ length: earned.of }, (_, i) => i) as i (i)}<span class:lit={i < earned.n} style:--d={`${300 + i * 160}ms`}>{i < earned.n ? "★" : "☆"}</span>{/each}
    </p>
  {/if}
  <dl class="tally">
    <div class="gold"><dt>{t(app.locale, "finish.xp")}</dt><dd>+{gained}</dd></div>
    <div class="blue"><dt>{t(app.locale, "finish.total")}</dt><dd><span aria-hidden="true">{Math.round(shown.current)}</span><span class="visually-hidden">{total}</span></dd></div>
    <div class="flame"><dt>{t(app.locale, "finish.streak")}</dt><dd>{days}</dd></div>
  </dl>
  {#if help.length}
    <p class="help">{t(app.locale, "finish.help", { name: text(help[0]!.title) })}</p>
  {/if}
  <button class="primary big" onclick={onclose}>{t(app.locale, "activity.back")}</button>
</section>

<style>
  /* The end of a lesson (ADR 0016): one emblem, the kernel's verdict in words, three numbers. */
  .finish { flex: 1; display: grid; align-content: center; justify-items: center; gap: 8px; padding: 24px 16px; text-align: center; max-width: 560px; margin: 0 auto; width: 100%; }
  .emblem { width: 104px; height: 104px; border-radius: 50%; display: grid; place-items: center; color: #fff; background: var(--ink); box-shadow: 0 5px 0 #000; margin-bottom: 12px; }
  .state-achieved .emblem { background: var(--achieved); box-shadow: 0 5px 0 var(--achieved-edge); }
  .state-needs_support .emblem { background: var(--try); box-shadow: 0 5px 0 var(--try-ink); }
  .emblem svg { fill: none; stroke: currentColor; stroke-width: 2.8; stroke-linecap: round; stroke-linejoin: round; }
  @media (prefers-reduced-motion: no-preference) {
    .emblem { animation: land 520ms cubic-bezier(.2,1.5,.4,1) both; }
    .emblem path { stroke-dasharray: 40; animation: draw 500ms 220ms ease-out both; }
    .stars .lit { display: inline-block; animation: star 380ms cubic-bezier(.2,1.8,.4,1) var(--d) both; }
  }
  @keyframes land { from { transform: scale(0.4); } }
  @keyframes draw { from { stroke-dashoffset: 40; } }
  @keyframes star { from { transform: scale(2) rotate(-30deg); } }
  h2 { font: 900 clamp(30px, 7vw, 40px)/1.15 var(--display); margin: 0; }
  .outcome { font-size: 19px; margin: 0; color: var(--muted); }
  .stars { font-size: 40px; margin: 4px 0 0; letter-spacing: 4px; color: var(--muted); }
  .stars .lit { color: var(--accent-ink); }
  .tally { display: grid; grid-template-columns: repeat(3, 1fr); gap: 10px; margin: 20px 0; width: 100%; }
  .tally div { border-radius: var(--radius); border: 2px solid var(--line); padding: 12px 8px; background: var(--surface); }
  dt { font-size: 14px; font-weight: 500; color: var(--muted); }
  dd { margin: 2px 0 0; font: 800 28px/1.2 var(--display); }
  .gold dd { color: var(--accent-ink); }
  .help { color: var(--needs-support); font-weight: 700; }
  .finish :global(button.big) { width: 100%; }
</style>
