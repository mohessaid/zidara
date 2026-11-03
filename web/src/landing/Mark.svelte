<script lang="ts">
  // Zidara's mark: the eight-pointed khatam of the region's tiles (two squares, one turned),
  // the same star the game's levels are drawn from.
  let { size = 24, tone = "accent" }: { size?: number; tone?: "accent" | "light" } = $props();
</script>

<svg class="mark {tone}" width={size} height={size} viewBox="0 0 32 32" aria-hidden="true">
  <rect x="6.5" y="6.5" width="19" height="19" />
  <rect x="6.5" y="6.5" width="19" height="19" transform="rotate(45 16 16)" />
</svg>

<style>
  .mark { flex: none; display: inline-block; vertical-align: middle; }
  rect { fill: var(--l-accent, var(--accent, #f7b63c)); }
  .light rect { fill: #fff; }
</style>
