<script lang="ts">
  // The oasis at morning: sky, sun, dunes, water and palms, drawn in SVG so it costs no image
  // request and stays sharp on any phone (art bible: "a path across a sunlit landscape").
  // `palms` is how many stand by the water; the front door shows a full oasis, the path shows
  // the learner's own, one palm per outcome achieved.
  import Palm from "./Palm.svelte";

  let { palms = 3, of = 3, grown = -1, height = 220 }: { palms?: number; of?: number; grown?: number; height?: number } = $props();
  const uid = $props.id();
  const SPOTS = [
    { x: 300, y: 168, s: 1.05 },
    { x: 92, y: 172, s: 0.95, flip: true },
    { x: 342, y: 176, s: 0.8 },
    { x: 54, y: 180, s: 0.75 },
    { x: 262, y: 180, s: 0.7, flip: true },
  ];
</script>

<svg class="scene" viewBox="0 0 400 220" preserveAspectRatio="xMidYMax slice" style:height={`${height}px`} aria-hidden="true">
  <defs>
    <linearGradient id={`${uid}-sky`} x1="0" y1="0" x2="0" y2="1">
      <stop offset="0" stop-color="#7fd3e6" />
      <stop offset="0.75" stop-color="#ffe7b8" />
    </linearGradient>
    <radialGradient id={`${uid}-sun`}>
      <stop offset="0.55" stop-color="#ffd23f" />
      <stop offset="1" stop-color="#ffd23f" stop-opacity="0" />
    </radialGradient>
  </defs>
  <rect width="400" height="220" fill={`url(#${uid}-sky)`} />
  <circle class="sun" cx="78" cy="62" r="46" fill={`url(#${uid}-sun)`} />
  <circle cx="78" cy="62" r="25" fill="#ffc21a" />
  <g fill="#fff" opacity="0.9">
    <ellipse cx="250" cy="44" rx="34" ry="11" /><ellipse cx="270" cy="36" rx="18" ry="12" /><ellipse cx="236" cy="38" rx="14" ry="9" />
  </g>
  <path d="M0 132 C60 104 120 112 180 126 C250 142 320 100 400 116 V220 H0 Z" fill="#f8dc9f" />
  <path d="M0 158 C70 132 150 140 210 152 C280 166 340 136 400 146 V220 H0 Z" fill="#f2c879" />
  <ellipse cx="200" cy="184" rx="118" ry="17" fill="#0b7285" />
  <ellipse cx="200" cy="181" rx="104" ry="11" fill="#2bb3c0" />
  <path d="M150 180 h30 M214 184 h22 M246 178 h14" stroke="#c9f3f7" stroke-width="3" stroke-linecap="round" />
  {#each SPOTS.slice(0, of) as spot, i (i)}
    <Palm x={spot.x} y={spot.y} size={spot.s} flip={spot.flip} dim={i >= palms} grow={i === grown} />
  {/each}
  <path d="M0 198 C80 186 140 200 200 204 C270 208 330 190 400 196 V220 H0 Z" fill="#e3a24f" />
</svg>

<style>
  .scene { display: block; width: 100%; }
</style>
