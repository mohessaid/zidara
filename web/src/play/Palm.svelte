<svelte:options namespace="svg" />

<script lang="ts">
  // A date palm, drawn for the oasis: one grows for every outcome the kernel says is achieved
  // (ADR 0013). `grow` plays the growing once (≤ 400 ms, art bible); `dim` draws the place a palm
  // will stand, so the learner sees what is still to come.
  let { x = 0, y = 0, size = 1, grow = false, dim = false, flip = false }: { x?: number; y?: number; size?: number; grow?: boolean; dim?: boolean; flip?: boolean } = $props();
</script>

<g transform={`translate(${x} ${y}) scale(${flip ? -size : size} ${size})`}>
  <g class="palm" class:grow class:dim>
    <path d="M-3 0 C-4 -20 -2 -40 5 -58 L11 -57 C5 -40 4 -20 4 0 Z" fill="#9a6435" />
    <path d="M-2 -10 L4 -11 M-2 -22 L5 -23 M0 -34 L7 -35 M2 -46 L9 -46" stroke="#6e4321" stroke-width="2" stroke-linecap="round" />
    <g class="crown">
      <path d="M8 -58 C-6 -66 -22 -62 -30 -50 C-16 -56 -6 -55 8 -56 Z" fill="#2f9e5b" />
      <path d="M8 -58 C22 -68 38 -64 44 -52 C30 -58 20 -57 8 -56 Z" fill="#2f9e5b" />
      <path d="M8 -58 C0 -76 -16 -80 -24 -74 C-10 -72 -2 -66 8 -57 Z" fill="#43b96d" />
      <path d="M8 -58 C16 -78 32 -82 40 -74 C26 -72 16 -66 8 -57 Z" fill="#43b96d" />
      <path d="M8 -58 C6 -74 10 -86 18 -90 C14 -78 12 -68 9 -57 Z" fill="#5fcf85" />
      <circle cx="4" cy="-53" r="3.2" fill="#c0541f" /><circle cx="10" cy="-52" r="3.2" fill="#d9682b" /><circle cx="7" cy="-49" r="3" fill="#a8461a" />
    </g>
  </g>
</g>

<style>
  .palm { transform-box: fill-box; transform-origin: 50% 100%; }
  .dim { opacity: 0.22; }
  .crown { transform-box: fill-box; transform-origin: 30% 90%; }
  @media (prefers-reduced-motion: no-preference) {
    .grow { animation: grow 400ms cubic-bezier(.2,1.5,.4,1) both; }
    .grow .crown { animation: unfurl 400ms 120ms cubic-bezier(.2,1.6,.4,1) both; }
  }
  @keyframes grow { from { transform: scale(0.1, 0); } }
  @keyframes unfurl { from { transform: scale(0.2); } }
</style>
