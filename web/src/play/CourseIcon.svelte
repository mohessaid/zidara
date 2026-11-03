<script lang="ts">
  // A course's badge (ADR 0016): one flat, solid shape, in the manner of Brilliant's course art.
  // A circle in quarters, two shaded, for fractions; three stacked layers (experience, framework,
  // kernel) for the course about Zidara itself. Unknown courses get a plain star.
  let { lesson, size = 64 }: { lesson: string; size?: number } = $props();
</script>

<svg class="icon" width={size} height={size} viewBox="0 0 64 64" aria-hidden="true">
  {#if lesson === "equivalent-fractions"}
    <rect x="2" y="2" width="60" height="60" rx="16" fill="var(--accent-soft)" />
    <circle cx="32" cy="32" r="20" fill="#fff" stroke="var(--ink)" stroke-width="3" />
    <path d="M32 32 L32 12 A20 20 0 0 1 52 32 Z" fill="var(--accent)" stroke="var(--ink)" stroke-width="3" stroke-linejoin="round" />
    <path d="M32 32 L52 32 A20 20 0 0 1 32 52 Z" fill="var(--accent)" stroke="var(--ink)" stroke-width="3" stroke-linejoin="round" />
    <path d="M12 32 H52 M32 12 V52" stroke="var(--ink)" stroke-width="3" />
  {:else if lesson === "how-zidara-works"}
    <rect x="2" y="2" width="60" height="60" rx="16" fill="var(--right-tint)" />
    <rect x="13" y="14" width="38" height="10" rx="4" fill="#fff" stroke="var(--ink)" stroke-width="3" />
    <rect x="13" y="27" width="38" height="10" rx="4" fill="#fff" stroke="var(--ink)" stroke-width="3" />
    <rect x="13" y="40" width="38" height="10" rx="4" fill="var(--achieved)" stroke="var(--ink)" stroke-width="3" />
  {:else}
    <rect x="2" y="2" width="60" height="60" rx="16" fill="var(--paper-2)" />
    <path d="M32 13 l5.6 11.4 12.6 1.8 -9.1 8.9 2.1 12.5 -11.2 -5.9 -11.2 5.9 2.1 -12.5 -9.1 -8.9 12.6 -1.8 Z" fill="var(--accent)" stroke="var(--ink)" stroke-width="3" stroke-linejoin="round" />
  {/if}
</svg>

<style>
  .icon { display: block; flex: none; }
</style>
