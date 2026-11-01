<script lang="ts">
  // A strip of paper, half of it coloured. Each fold doubles the parts and the coloured parts:
  // the numbers change, the amount of paper does not. 1/2 = 2/4 = 4/8.
  import { app } from "../lib/app.svelte.ts";
  import { t } from "../lib/i18n/i18n.ts";
  import { sound } from "./sound.svelte.ts";

  let { folds, ondone }: { folds: number; ondone: () => void } = $props();
  let done = $state(0);
  const parts = $derived(2 ** (done + 1));

  function fold() {
    done++;
    sound.play(done === folds ? "combo" : "tap");
    if (done >= Math.min(2, folds)) ondone();
  }
</script>

<div class="fold">
  <div class="strip" dir="ltr" role="img" aria-label={t(app.locale, "picture.bar", { shaded: parts / 2, parts })}>
    {#each Array.from({ length: parts }, (_, i) => i) as i (`${parts}-${i}`)}
      <span class="part" class:on={i < parts / 2} style:animation-delay={`${i * 25}ms`}></span>
    {/each}
  </div>
  <p class="reads" aria-live="polite"><bdi dir="ltr">{parts / 2}/{parts}</bdi> — {t(app.locale, "toy.fold.same")}</p>
  <button class="primary big" onclick={fold} disabled={done >= folds}>{t(app.locale, "toy.fold.button")}</button>
  <ol class="trail" dir="ltr" aria-label={t(app.locale, "toy.fold.trail")}>
    {#each Array.from({ length: done + 1 }, (_, k) => 2 ** (k + 1)) as n (n)}
      <li>{n / 2}/{n}</li>
    {/each}
  </ol>
</div>

<style>
  .fold { display: grid; gap: 1rem; justify-items: center; }
  .strip { display: flex; width: min(100%, 420px); height: 72px; gap: 3px; }
  .part { flex: 1; border: 3px solid var(--ink); border-radius: 6px; background: var(--surface); animation: pop 260ms ease both; }
  .part.on { background: var(--in-progress); border-color: var(--in-progress); }
  .reads { font-size: 22px; font-weight: 700; margin: 0; text-align: center; }
  .trail { display: flex; gap: 0.5rem; list-style: none; padding: 0; margin: 0; font-size: 20px; }
  .trail li { background: var(--surface); border: 2px solid var(--in-progress); border-radius: 999px; padding: 0.1rem 0.8rem; }
  .trail li + li::before { content: "="; margin-inline-end: 0.5rem; color: var(--locked); }
  @keyframes pop { from { transform: scaleY(0.6); opacity: 0.4; } }
</style>
