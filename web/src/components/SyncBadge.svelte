<script lang="ts">
  import { app } from "../lib/app.svelte.ts";
  import { t } from "../lib/i18n/i18n.ts";

  const message = $derived(
    !app.sync.online
      ? t(app.locale, "sync.offline")
      : app.sync.pending > 0
        ? t(app.locale, "sync.pending", { n: app.sync.pending })
        : t(app.locale, "sync.synced"),
  );
</script>

<p class="sync" class:offline={!app.sync.online} role="status" data-testid="sync">
  {message}
  {#if app.sync.refused > 0}<span> · {t(app.locale, "sync.refused", { n: app.sync.refused })}</span>{/if}
</p>

<style>
  /* A plain line of words, not a badge: green while saved, grey while offline. */
  .sync { margin: 0; font-size: 15px; font-weight: 500; color: var(--achieved); }
  .offline { color: var(--locked); }
</style>
