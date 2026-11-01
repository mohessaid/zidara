<script lang="ts">
  // A fraction wall: the same whole on every row, cut differently. Tap a part and the length up
  // to it is coloured; every row that can show exactly that length lights up too. Equal
  // fractions are the same length.
  import { app } from "../lib/app.svelte.ts";
  import { t } from "../lib/i18n/i18n.ts";
  import { sound } from "./sound.svelte.ts";

  let { rows, ondone }: { rows: number[]; ondone: () => void } = $props();
  /** The chosen length, as a fraction of the whole: [num, den]. */
  let chosen: [number, number] | undefined = $state();
  let taps = $state(0);

  function pick(i: number, n: number) {
    chosen = [i + 1, n];
    taps++;
    const matches = rows.filter((r) => ((i + 1) * r) % n === 0).length;
    sound.play(matches > 1 ? "combo" : "tap");
    if (taps >= 2) ondone();
  }

  const covered = (row: number) => (chosen && (chosen[0] * row) % chosen[1] === 0 ? (chosen[0] * row) / chosen[1] : 0);
  const equal = $derived(chosen ? rows.filter((r) => covered(r) > 0).map((r) => `${covered(r)}/${r}`) : []);
</script>

<div class="wall" dir="ltr">
  {#each rows as n (n)}
    <div class="row" role="group" aria-label={t(app.locale, "toy.wall.row", { n })}>
      {#each Array.from({ length: n }, (_, i) => i) as i (i)}
        <button class="brick" class:on={i < covered(n)} onclick={() => pick(i, n)}
          aria-label={`${i + 1}/${n}`}>{n <= 6 ? `1/${n}` : ""}</button>
      {/each}
    </div>
  {/each}
</div>
<p class="equal" aria-live="polite">
  {#if equal.length > 1}<bdi dir="ltr">{equal.join(" = ")}</bdi>{:else}{t(app.locale, "toy.wall.hint")}{/if}
</p>

<style>
  .wall { display: grid; gap: 4px; width: min(100%, 480px); margin: 0 auto; }
  .row { display: flex; gap: 3px; }
  .brick { flex: 1; min-width: 0; min-height: 48px; padding: 0; border-radius: 6px; font-size: 15px; border-width: 2px; }
  .brick.on { background: var(--achieved); border-color: var(--achieved); color: #fff; }
  .equal { text-align: center; font-size: 22px; font-weight: 700; min-height: 2.2em; }
</style>
