<script lang="ts">
  // The oasis at morning, in dots (ADR 0013, ADR 0015): the same sun, dunes, water and palms,
  // each shape filled with a pattern of dots, the language of every picture in Zidara. A handful
  // of SVG patterns: no image request, no canvas, no frame loop, sharp on any phone. `palms` is
  // how many stand by the water; the front door shows a full oasis, the path the learner's own.
  import Palm from "./Palm.svelte";

  let { palms = 3, of = 3, grown = -1, height = 220 }: { palms?: number; of?: number; grown?: number; height?: number } = $props();
  const uid = $props.id();
  const SPOTS = [
    { x: 300, y: 168, s: 0.95 },
    { x: 92, y: 172, s: 0.85, flip: true },
    { x: 342, y: 176, s: 0.72 },
    { x: 54, y: 180, s: 0.68 },
    { x: 262, y: 180, s: 0.62, flip: true },
  ];
  /** A pattern of one dot per cell: `ring` draws it hollow. */
  const PATTERNS = [
    { id: "sky", gap: 10, r: 1.7, colour: "#c9d5f2", ring: true },
    { id: "sun", gap: 6, r: 2.3, colour: "#f5b301" },
    { id: "far", gap: 7, r: 2.2, colour: "#f1d59a" },
    { id: "near", gap: 7, r: 2.5, colour: "#e8b65c" },
    { id: "water", gap: 6, r: 2.3, colour: "#2450d3" },
    { id: "ground", gap: 7, r: 2.6, colour: "#cf8d36" },
  ];
</script>

<svg class="scene" viewBox="0 0 400 220" preserveAspectRatio="xMidYMax slice" style:height={`${height}px`} aria-hidden="true">
  <defs>
    {#each PATTERNS as p (p.id)}
      <pattern id={`${uid}-${p.id}`} width={p.gap} height={p.gap} patternUnits="userSpaceOnUse">
        {#if p.ring}
          <circle cx={p.gap / 2} cy={p.gap / 2} r={p.r} fill="none" stroke={p.colour} stroke-width="0.9" />
        {:else}
          <circle cx={p.gap / 2} cy={p.gap / 2} r={p.r} fill={p.colour} />
        {/if}
      </pattern>
    {/each}
    <linearGradient id={`${uid}-fade`} x1="0" y1="0" x2="0" y2="1">
      <stop offset="0" stop-color="#fff" stop-opacity="1" />
      <stop offset="0.7" stop-color="#fff" stop-opacity="0.15" />
    </linearGradient>
    <mask id={`${uid}-skymask`}><rect width="400" height="220" fill={`url(#${uid}-fade)`} /></mask>
  </defs>
  <rect width="400" height="220" fill="#f7f8fd" />
  <rect width="400" height="160" fill={`url(#${uid}-sky)`} mask={`url(#${uid}-skymask)`} />
  <circle cx="78" cy="62" r="26" fill={`url(#${uid}-sun)`} />
  <path d="M0 132 C60 104 120 112 180 126 C250 142 320 100 400 116 V220 H0 Z" fill="#f7f8fd" />
  <path d="M0 132 C60 104 120 112 180 126 C250 142 320 100 400 116 V220 H0 Z" fill={`url(#${uid}-far)`} />
  <path d="M0 158 C70 132 150 140 210 152 C280 166 340 136 400 146 V220 H0 Z" fill="#f7f8fd" />
  <path d="M0 158 C70 132 150 140 210 152 C280 166 340 136 400 146 V220 H0 Z" fill={`url(#${uid}-near)`} />
  <ellipse cx="200" cy="184" rx="118" ry="17" fill="#f7f8fd" />
  <ellipse cx="200" cy="184" rx="118" ry="17" fill={`url(#${uid}-water)`} />
  {#each SPOTS.slice(0, of) as spot, i (i)}
    <Palm x={spot.x} y={spot.y} size={spot.s} flip={spot.flip} dim={i >= palms} grow={i === grown} />
  {/each}
  <path d="M0 198 C80 186 140 200 200 204 C270 208 330 190 400 196 V220 H0 Z" fill="#f7f8fd" />
  <path d="M0 198 C80 186 140 200 200 204 C270 208 330 190 400 196 V220 H0 Z" fill={`url(#${uid}-ground)`} />
</svg>

<style>
  .scene { display: block; width: 100%; }
</style>
