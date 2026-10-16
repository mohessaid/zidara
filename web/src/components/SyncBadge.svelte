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

<p class="badge" class:offline={!app.sync.online} role="status" data-testid="sync">
  {message}
  {#if app.sync.refused > 0}<span> · {t(app.locale, "sync.refused", { n: app.sync.refused })}</span>{/if}
</p>

<style>
  .badge { margin: 0; font-size: 16px; padding: 0.25rem 0.75rem; border-radius: 999px; border: 2px solid var(--achieved); color: var(--achieved); }
  .offline { border-color: var(--locked); color: var(--locked); }
</style>
