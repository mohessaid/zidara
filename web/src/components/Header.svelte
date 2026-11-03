<script lang="ts">
  import type { Snippet } from "svelte";
  import { app } from "../lib/app.svelte.ts";
  import { t } from "../lib/i18n/i18n.ts";
  import LanguagePicker from "./LanguagePicker.svelte";
  import SyncBadge from "./SyncBadge.svelte";

  let { title, children }: { title: string; children?: Snippet } = $props();
</script>

<header class="row">
  <h1>{title}</h1>
  <span class="spacer"></span>
  {@render children?.()}
  <SyncBadge />
  <LanguagePicker />
  <span>{app.me?.display_name}</span>
  <button onclick={() => app.signOut()}>{t(app.locale, "signin.out")}</button>
</header>

<style>
  header { padding: 0.5rem 1rem; border-bottom: 1px solid var(--line); background: var(--surface); }
  .spacer { flex: 1; }
</style>
