<script lang="ts">
  // Bizra (بذرة, "seed"): Zidara's guide, a seed that grows a sprout. Drawn here, in SVG, so it
  // costs no image request and scales to any screen. Moods follow the art bible: it cheers real
  // progress and encourages after a miss; it is never sad, never disappointed.
  let { mood = "idle", size = 120 }: { mood?: "idle" | "happy" | "cheer" | "think" | "encourage"; size?: number } = $props();
  const uid = $props.id();
</script>

<svg class="mascot {mood}" width={size} height={size} viewBox="0 0 120 120" aria-hidden="true">
  <g class="body">
    <!-- Sprout -->
    <path d="M60 30 C60 22 60 16 60 12" stroke="#2f8f3a" stroke-width="4" stroke-linecap="round" fill="none" />
    <path class="leaf l" d="M60 16 C48 4 34 8 32 14 C40 22 52 22 60 16Z" fill="#7ed957" stroke="#2f8f3a" stroke-width="2.5" stroke-linejoin="round" />
    <path class="leaf r" d="M60 13 C70 0 86 2 88 9 C80 18 68 19 60 13Z" fill="#9be36f" stroke="#2f8f3a" stroke-width="2.5" stroke-linejoin="round" />
    <!-- Seed body -->
    <ellipse cx="60" cy="72" rx="40" ry="42" fill="#3fb950" />
    <ellipse cx="60" cy="72" rx="40" ry="42" fill={`url(#${uid}-shade)`} />
    <ellipse cx="60" cy="86" rx="25" ry="22" fill="#b9f0a6" />
    <!-- Feet -->
    <ellipse cx="45" cy="112" rx="10" ry="5" fill="#ff9600" />
    <ellipse cx="75" cy="112" rx="10" ry="5" fill="#ff9600" />
    <!-- Cheeks -->
    <ellipse cx="33" cy="76" rx="6" ry="4" fill="#ff8fa3" opacity="0.7" />
    <ellipse cx="87" cy="76" rx="6" ry="4" fill="#ff8fa3" opacity="0.7" />
    <!-- Eyes -->
    {#if mood === "happy" || mood === "cheer"}
      <path d="M38 60 Q46 50 54 60" stroke="#1c2433" stroke-width="5" stroke-linecap="round" fill="none" />
      <path d="M66 60 Q74 50 82 60" stroke="#1c2433" stroke-width="5" stroke-linecap="round" fill="none" />
    {:else}
      <g class="eyes">
        <ellipse cx="46" cy="60" rx="10" ry="12" fill="#fff" />
        <ellipse cx="74" cy="60" rx="10" ry="12" fill="#fff" />
        <circle class="pupil" cx={mood === "think" ? 48 : 47} cy={mood === "think" ? 55 : 62} r="6" fill="#1c2433" />
        <circle class="pupil" cx={mood === "think" ? 76 : 75} cy={mood === "think" ? 55 : 62} r="6" fill="#1c2433" />
        <circle cx="49" cy="58" r="2" fill="#fff" />
        <circle cx="77" cy="58" r="2" fill="#fff" />
      </g>
    {/if}
    <!-- Mouth -->
    {#if mood === "cheer"}
      <path d="M48 76 Q60 94 72 76 Z" fill="#1c2433" />
      <path d="M53 84 Q60 90 67 84" fill="#ff6b81" />
    {:else if mood === "think"}
      <ellipse cx="62" cy="80" rx="4" ry="5" fill="#1c2433" />
    {:else if mood === "encourage"}
      <path d="M50 78 Q60 86 70 78" stroke="#1c2433" stroke-width="4" stroke-linecap="round" fill="none" />
    {:else}
      <path d="M48 76 Q60 90 72 76" stroke="#1c2433" stroke-width="4" stroke-linecap="round" fill="none" />
    {/if}
    {#if mood === "cheer"}
      <path class="arm" d="M22 70 L8 50" stroke="#3fb950" stroke-width="8" stroke-linecap="round" />
      <path class="arm" d="M98 70 L112 50" stroke="#3fb950" stroke-width="8" stroke-linecap="round" />
    {/if}
  </g>
  <defs>
    <radialGradient id={`${uid}-shade`} cx="0.35" cy="0.3" r="0.8">
      <stop offset="0" stop-color="#ffffff" stop-opacity="0.35" />
      <stop offset="0.6" stop-color="#ffffff" stop-opacity="0" />
      <stop offset="1" stop-color="#1a6b2a" stop-opacity="0.35" />
    </radialGradient>
  </defs>
</svg>

<style>
  .mascot { display: block; overflow: visible; }
  .body { transform-origin: 60px 112px; }
  .idle .body, .think .body { animation: bob 2.4s ease-in-out infinite; }
  .happy .body { animation: hop 0.6s ease-out 2; }
  .cheer .body { animation: jump 0.7s cubic-bezier(.3,1.6,.5,1) 2; }
  .encourage .body { animation: tilt 1.2s ease-in-out 1; }
  .eyes { animation: blink 4s infinite; transform-origin: 60px 60px; }
  .leaf { transform-origin: 60px 16px; animation: sway 3s ease-in-out infinite; }
  .leaf.r { animation-delay: -1.5s; }
  @keyframes bob { 50% { transform: translateY(-4px) scaleY(1.02); } }
  @keyframes hop { 40% { transform: translateY(-12px); } }
  @keyframes jump { 0% { transform: scale(1, 1); } 30% { transform: translateY(-22px) scale(0.95, 1.08); } 60% { transform: translateY(0) scale(1.08, 0.92); } }
  @keyframes tilt { 30% { transform: rotate(-6deg); } 70% { transform: rotate(5deg); } }
  @keyframes blink { 0%, 92%, 100% { transform: scaleY(1); } 95% { transform: scaleY(0.1); } }
  @keyframes sway { 50% { transform: rotate(6deg); } }
</style>
