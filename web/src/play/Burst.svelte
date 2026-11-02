<script lang="ts">
  // A short burst of small eight-pointed stars after a right answer: twelve SVG nodes, gone in
  // 600 ms, and nothing at all with reduced motion (girth-gameui: juice that fits Zidara).
  const BITS = Array.from({ length: 12 }, (_, i) => {
    const a = (i / 12) * Math.PI * 2;
    const r = 70 + (i % 3) * 18;
    return { x: Math.round(Math.cos(a) * r), y: Math.round(Math.sin(a) * r), c: ["#f5b301", "#1d7a45", "#0b7285", "#b4471f"][i % 4], d: (i % 4) * 25 };
  });
</script>

<span class="burst" aria-hidden="true">
  {#each BITS as b, i (i)}
    <svg viewBox="0 0 20 20" width="18" height="18" style:--x={`${b.x}px`} style:--y={`${b.y}px`} style:--d={`${b.d}ms`}>
      <path d="M10 0 13 7 20 10 13 13 10 20 7 13 0 10 7 7Z" fill={b.c} />
    </svg>
  {/each}
</span>

<style>
  .burst { position: absolute; inset: 50% auto auto 50%; width: 0; height: 0; pointer-events: none; display: none; }
  svg { position: absolute; left: -9px; top: -9px; opacity: 0; }
  @media (prefers-reduced-motion: no-preference) {
    .burst { display: block; }
    svg { animation: fly 600ms cubic-bezier(.15,.8,.3,1) var(--d) both; }
  }
  @keyframes fly {
    0% { transform: translate(0, 0) scale(0.3) rotate(0); opacity: 1; }
    70% { opacity: 1; }
    100% { transform: translate(var(--x), var(--y)) scale(1) rotate(180deg); opacity: 0; }
  }
</style>
