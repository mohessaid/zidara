<svelte:options namespace="svg" />

<script lang="ts">
  // A date palm in dots (ADR 0013, ADR 0015): one grows for every outcome the kernel says is
  // achieved. `grow` plays the growing once (≤ 400 ms, art bible), row by row from the ground;
  // `dim` draws the place a palm will stand, so the learner sees what is still to come.
  import { bitmap, PALM, PALM_COLOURS } from "./dots.ts";

  let { x = 0, y = 0, size = 1, grow = false, dim = false, flip = false }: { x?: number; y?: number; size?: number; grow?: boolean; dim?: boolean; flip?: boolean } = $props();
  const DOTS = bitmap(PALM, PALM_COLOURS);
  const CELL = 6.6;
</script>

<g transform={`translate(${x} ${y}) scale(${flip ? -size : size} ${size})`}>
  <g class="palm" class:grow class:dim transform={`translate(${-6.5 * CELL} ${-14 * CELL})`}>
    {#each DOTS as d (`${d.x},${d.y}`)}
      <circle cx={(d.x + 0.5) * CELL} cy={(d.y + 0.5) * CELL} r={CELL * 0.42} fill={d.c} style:--row={13 - d.y} />
    {/each}
  </g>
</g>

<style>
  .dim { opacity: 0.22; }
  @media (prefers-reduced-motion: no-preference) {
    .grow circle { animation: pop 160ms steps(2) both; animation-delay: calc(var(--row) * 18ms); }
  }
  @keyframes pop { from { opacity: 0; } }
</style>
