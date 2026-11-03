<script lang="ts">
  // A short burst of pixels after a right answer (ADR 0015): sixteen square dots fly out and fade
  // in 600 ms, and nothing at all with reduced motion (girth-gameui: juice that fits Zidara).
  const BITS = Array.from({ length: 16 }, (_, i) => {
    const a = (i / 16) * Math.PI * 2;
    const r = 64 + (i % 3) * 20;
    return { x: Math.round(Math.cos(a) * r), y: Math.round(Math.sin(a) * r), c: ["#2450d3", "#f5b301", "#1d7a45", "#9fb3e6"][i % 4], d: (i % 4) * 25, s: 6 + (i % 3) * 3 };
  });
</script>

<span class="burst" aria-hidden="true">
  {#each BITS as b, i (i)}
    <i style:--x={`${b.x}px`} style:--y={`${b.y}px`} style:--d={`${b.d}ms`} style:--s={`${b.s}px`} style:background={b.c}></i>
  {/each}
</span>

<style>
  .burst { position: absolute; inset: 50% auto auto 50%; width: 0; height: 0; pointer-events: none; display: none; }
  i { position: absolute; width: var(--s); height: var(--s); left: calc(var(--s) / -2); top: calc(var(--s) / -2); border-radius: 2px; opacity: 0; }
  @media (prefers-reduced-motion: no-preference) {
    .burst { display: block; }
    i { animation: fly 600ms steps(8) var(--d) both; }
  }
  @keyframes fly {
    0% { transform: translate(0, 0) scale(0.4); opacity: 1; }
    70% { opacity: 1; }
    100% { transform: translate(var(--x), var(--y)) scale(1); opacity: 0; }
  }
</style>
