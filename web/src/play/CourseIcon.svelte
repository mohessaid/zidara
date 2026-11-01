<script lang="ts">
  // A course's badge, drawn in SVG: a pie in quarters for fractions, a path of levels for the
  // course about Zidara itself. Unknown courses get a star.
  let { lesson, size = 64 }: { lesson: string; size?: number } = $props();
</script>

<svg width={size} height={size} viewBox="0 0 64 64" aria-hidden="true">
  {#if lesson === "equivalent-fractions"}
    <circle cx="32" cy="32" r="28" fill="#ffe8a3" stroke="#e0a800" stroke-width="3" />
    <path d="M32 32 L32 4 A28 28 0 0 1 60 32 Z" fill="#ff9600" />
    <path d="M32 32 L60 32 A28 28 0 0 1 32 60 Z" fill="#ff9600" />
    <path d="M32 32 L32 60 A28 28 0 0 1 4 32 Z" fill="#ff9600" />
    <path d="M32 4 V60 M4 32 H60" stroke="#fff" stroke-width="3" />
  {:else if lesson === "how-zidara-works"}
    <rect x="2" y="2" width="60" height="60" rx="16" fill="#ddf4ff" />
    <path d="M14 48 Q24 30 32 38 T50 16" fill="none" stroke="#1a7fd0" stroke-width="4" stroke-linecap="round" stroke-dasharray="1 8" />
    <circle cx="14" cy="48" r="7" fill="#22983f" />
    <circle cx="32" cy="38" r="7" fill="#1a7fd0" />
    <circle cx="50" cy="16" r="8" fill="#ffc800" stroke="#e0a800" stroke-width="2" />
    <path d="M46 16 l3 3 l5 -6" fill="none" stroke="#fff" stroke-width="2.5" stroke-linecap="round" stroke-linejoin="round" />
  {:else}
    <circle cx="32" cy="32" r="28" fill="#ffc800" />
    <path d="M32 14 l5 11 12 1 -9 8 3 12 -11 -6 -11 6 3 -12 -9 -8 12 -1z" fill="#fff" />
  {/if}
</svg>
