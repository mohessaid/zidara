<script lang="ts">
  // One dot-matrix drawing (ADR 0015). Decorative: whatever it argues is also said in words.
  import { pixels, type Scene } from "./pixels.ts";
  let { scene, cols, rows, cell, still = 2 }: { scene: Scene; cols?: number; rows?: number; cell?: number; still?: number } = $props();
</script>

<div class="px"><canvas use:pixels={{ scene, cols, rows, cell, still }} aria-hidden="true"></canvas></div>

<style>
  /* Out of the layout flow: a canvas sized for one layout must never widen the next one. */
  .px { position: absolute; inset: var(--px-inset, 0); display: grid; place-items: center; overflow: hidden; }
  canvas { max-width: 100%; max-height: 100%; }
  canvas { display: block; }
</style>
