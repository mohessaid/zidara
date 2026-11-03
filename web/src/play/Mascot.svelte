<script lang="ts">
  // Bizra (بذرة, "seed"): Zidara's guide, drawn as a grid of dots like every picture in Zidara
  // (ADR 0015). Moods follow the art bible: it cheers real progress and encourages after a miss;
  // it is never sad, never disappointed. `still`: no idle loop (bob, blink, sway), for screens
  // that must stay quiet and cheap, like the path.
  import { BIZRA, BIZRA_COLOURS, bitmap, FACES } from "./dots.ts";

  let { mood = "idle", size = 120, still = false }: { mood?: "idle" | "happy" | "cheer" | "think" | "encourage"; size?: number; still?: boolean } = $props();
  const body = bitmap(BIZRA, BIZRA_COLOURS);
  const leaves = body.filter((d) => d.y < 3);
  const rest = body.filter((d) => d.y >= 3);
  const face = $derived(FACES[mood]);
  const R = 0.42;
</script>

<svg class="mascot {mood}" class:still width={size} height={size} viewBox="-2.5 -0.5 21 19" aria-hidden="true">
  <g class="body">
    <g class="leaves">{#each leaves as d (`${d.x},${d.y}`)}<circle cx={d.x + 0.5} cy={d.y + 0.5} r={R} fill={d.c} />{/each}</g>
    {#each rest as d (`${d.x},${d.y}`)}<circle cx={d.x + 0.5} cy={d.y + 0.5} r={R} fill={d.c} />{/each}
    <g class="eyes">{#each face.eyes as [x, y, c] (`${x},${y}`)}<circle cx={x + 0.5} cy={y + 0.5} r={R + 0.05} fill={c} />{/each}</g>
    {#each face.mouth as [x, y, c] (`${x},${y}`)}<circle cx={x + 0.5} cy={y + 0.5} r={R} fill={c} />{/each}
    {#each face.arms ?? [] as [x, y] (`${x},${y}`)}<circle class="arm" cx={x + 0.5} cy={y + 0.5} r={R} fill={BIZRA_COLOURS.b} />{/each}
  </g>
</svg>

<style>
  .mascot { display: block; overflow: visible; }
  .body { transform-box: fill-box; transform-origin: 50% 100%; }
  .eyes { transform-box: fill-box; transform-origin: 50% 50%; }
  .leaves { transform-box: fill-box; transform-origin: 50% 100%; }
  .idle .body, .think .body { animation: bob 2.4s steps(4) infinite; }
  .happy .body { animation: hop 0.6s steps(3) 2; }
  .cheer .body { animation: jump 0.7s steps(5) 2; }
  .encourage .body { animation: tilt 1.2s steps(6) 1; }
  .eyes { animation: blink 4s infinite; }
  .leaves { animation: sway 3s steps(6) infinite; }
  .still .body, .still .eyes, .still .leaves { animation: none; }
  /* Pixel motion moves in whole steps, like the dots it is made of. */
  @keyframes bob { 50% { transform: translateY(-3%); } }
  @keyframes hop { 40% { transform: translateY(-10%); } }
  @keyframes jump { 0% { transform: none; } 30% { transform: translateY(-16%) scale(0.95, 1.06); } 60% { transform: scale(1.06, 0.94); } }
  @keyframes tilt { 30% { transform: rotate(-5deg); } 70% { transform: rotate(4deg); } }
  @keyframes blink { 0%, 92%, 100% { transform: scaleY(1); } 95% { transform: scaleY(0.1); } }
  @keyframes sway { 50% { transform: rotate(5deg); } }
</style>
