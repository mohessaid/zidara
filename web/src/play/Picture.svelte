<script lang="ts">
  // A whole cut into equal parts, some coloured: a bar, a round shape, or two bars to compare.
  // Drawn as SVG (sharp at any size, nothing to download); described in words for screen readers.
  import { app } from "../lib/app.svelte.ts";
  import { t } from "../lib/i18n/i18n.ts";
  import type { Visual } from "../lib/kernel/types/Visual.ts";

  let { visual }: { visual: Visual | null } = $props();

  const W = 320;
  const bars = $derived(
    !visual ? [] : visual.type === "pair" ? [visual.top, visual.bottom] : visual.type === "bar" ? [{ parts: visual.parts, shaded: visual.shaded }] : [],
  );
  const label = $derived(
    bars.length
      ? bars.map((b) => t(app.locale, "picture.bar", { shaded: b.shaded, parts: b.parts })).join(" · ")
      : visual?.type === "circle"
        ? t(app.locale, "picture.circle", { shaded: visual.shaded, parts: visual.parts })
        : "",
  );

  /** One slice of a round whole, as an SVG path (from 12 o'clock, clockwise). */
  function slice(i: number, n: number, r: number): string {
    if (n === 1) return `M ${r} 0 A ${r} ${r} 0 1 1 ${r - 0.01} 0 Z`;
    const a0 = (2 * Math.PI * i) / n - Math.PI / 2;
    const a1 = (2 * Math.PI * (i + 1)) / n - Math.PI / 2;
    const [x0, y0, x1, y1] = [r + r * Math.cos(a0), r + r * Math.sin(a0), r + r * Math.cos(a1), r + r * Math.sin(a1)];
    return `M ${r} ${r} L ${x0} ${y0} A ${r} ${r} 0 ${a1 - a0 > Math.PI ? 1 : 0} 1 ${x1} ${y1} Z`;
  }
</script>

{#if !visual}
  <!-- no picture -->
{:else if visual.type === "circle"}
  <svg class="picture circle" viewBox="-4 -4 168 168" role="img" aria-label={label}>
    {#each Array.from({ length: visual.parts }, (_, i) => i) as i (i)}
      <path d={slice(i, visual.parts, 80)} class:on={i < visual.shaded} />
    {/each}
  </svg>
{:else}
  <svg class="picture" viewBox={`-4 -4 ${W + 8} ${bars.length * 64 + 4}`} role="img" aria-label={label}>
    {#each bars as bar, row (row)}
      {#each Array.from({ length: bar.parts }, (_, i) => i) as i (i)}
        <rect x={(i * W) / bar.parts} y={row * 64} width={W / bar.parts} height="52" rx="6" class:on={i < bar.shaded} />
      {/each}
    {/each}
  </svg>
{/if}

<style>
  .picture { display: block; width: min(100%, 360px); margin: 0.75rem auto; overflow: visible; }
  .circle { width: min(60%, 200px); }
  rect, path { fill: var(--surface); stroke: var(--ink); stroke-width: 3; stroke-linejoin: round; }
  .on { fill: #ff9600; }
</style>
