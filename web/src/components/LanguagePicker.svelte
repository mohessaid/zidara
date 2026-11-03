<script lang="ts">
  import { app } from "../lib/app.svelte.ts";
  import { LOCALES, t, type Locale } from "../lib/i18n/i18n.ts";

  const NAMES: Record<Locale, string> = { ar: "العربية", fr: "Français", en: "English" };
</script>

<label class="row">
  <span class="visually-hidden">{t(app.locale, "app.language")}</span>
  <select value={app.locale} onchange={(e) => app.setLocale(e.currentTarget.value as Locale)}>
    {#each LOCALES as locale (locale)}
      <option value={locale} lang={locale}>{NAMES[locale]}</option>
    {/each}
  </select>
</label>

<style>
  select { min-height: 44px; border-radius: 12px; border: 2px solid var(--edge, #8a8a93); padding: 0 0.75rem; background: var(--surface); font-weight: 600; color: var(--ink); }
</style>
