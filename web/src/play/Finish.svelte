<script lang="ts">
  // The end of a session: what the class's framework now says about the outcome (achieved,
  // getting there, or "let's try another way" with the help the kernel assigned), the XP this
  // session earned and the streak. Celebration is short and only for real progress.
  import { onMount } from "svelte";
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
  const events = $derived((void app.version, app.kernel!.events()));
  const total = $derived(xp(events, app.me!.id, (a, v, i, r) => app.kernel!.check(a, v, i, r)));
  const days = $derived(streak(events, app.me!.id, Date.now()));
  const help = $derived((app.pack?.activities ?? []).filter((a) => a.role === "remediation" && a.outcomes.includes(outcome) && view?.assigned.includes(a.id)));
  const text = (l: Record<string, string>) => l[app.locale] ?? l.en ?? Object.values(l)[0] ?? "";
  const CONFETTI = Array.from({ length: 24 }, (_, i) => i);

  onMount(() => sound.play(state === "achieved" ? "achieved" : "complete"));
</script>

<section class="finish state-{state}" aria-labelledby="finish-title">
  {#if state === "achieved"}
    <div class="confetti" aria-hidden="true">
      {#each CONFETTI as i (i)}<i style:--x={`${(i * 37) % 100}%`} style:--d={`${(i % 6) * 90}ms`} class="k{i % 4}"></i>{/each}
    </div>
  {/if}
  <p class="emblem" aria-hidden="true">{state === "achieved" ? "⭐" : state === "needs_support" ? "🏮" : "🌱"}</p>
  <h2 id="finish-title">{t(app.locale, `finish.${state}`)}</h2>
  {#if info}<p class="outcome">{info.title}</p>{/if}
  {#if earned.of > 0}
    <p class="stars" aria-label={t(app.locale, "stars", { n: earned.n, of: earned.of })}>
      {#each Array.from({ length: earned.of }, (_, i) => i) as i (i)}<span class:lit={i < earned.n}>★</span>{/each}
    </p>
  {/if}
  <dl class="tally">
    <div><dt>{t(app.locale, "finish.xp")}</dt><dd>+{gained}</dd></div>
    <div><dt>{t(app.locale, "finish.total")}</dt><dd>{total}</dd></div>
    <div><dt>{t(app.locale, "finish.streak")}</dt><dd>🔥 {days}</dd></div>
  </dl>
  {#if help.length}
    <p class="help">{t(app.locale, "finish.help", { name: text(help[0]!.title) })}</p>
  {/if}
  <button class="primary big" onclick={onclose}>{t(app.locale, "activity.back")}</button>
</section>

<style>
  .finish { position: relative; flex: 1; display: grid; align-content: center; justify-items: center; gap: 0.5rem; padding: 1rem; text-align: center; overflow: hidden; }
  .emblem { font-size: 88px; margin: 0; animation: land 420ms cubic-bezier(.2,1.6,.4,1) both; }
  h2 { font-size: 30px; margin: 0; }
  .outcome { font-size: 20px; margin: 0; }
  .stars { font-size: 36px; margin: 0; letter-spacing: 0.2rem; color: #d9cdb4; }
  .stars .lit { color: var(--star); }
  .tally { display: flex; gap: 1rem; margin: 1rem 0; }
  .tally div { background: var(--surface); border: 2px solid #e6dcc8; border-radius: var(--radius); padding: 0.5rem 1rem; min-width: 6.5rem; }
  dt { font-size: 16px; color: var(--locked); }
  dd { margin: 0; font-size: 26px; font-weight: 700; }
  .help { color: var(--needs-support); font-weight: 700; }
  .finish :global(button.big) { width: min(100%, 420px); }
  .confetti { position: absolute; inset: 0; pointer-events: none; }
  .confetti i { position: absolute; top: -12px; left: var(--x); width: 10px; height: 16px; border-radius: 3px; animation: fall 1.6s ease-in var(--d) both; }
  .k0 { background: var(--in-progress); } .k1 { background: var(--achieved); } .k2 { background: var(--star); } .k3 { background: var(--agent); }
  @keyframes fall { to { transform: translateY(110vh) rotate(540deg); opacity: 0.2; } }
  @keyframes land { from { transform: scale(0.3); opacity: 0; } }
</style>
