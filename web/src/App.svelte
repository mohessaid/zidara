<script lang="ts">
  import { onMount } from "svelte";
  import { app } from "./lib/app.svelte.ts";
  import { t } from "./lib/i18n/i18n.ts";
  import SignIn from "./screens/SignIn.svelte";
  // Learners' and teachers' screens load on demand: sign-in needs neither PixiJS nor the studio.
  const Learner = () => import("./screens/Learner.svelte");
  const Studio = () => import("./screens/Studio.svelte");

  onMount(() => {
    app.setLocale(app.locale);
    void app.start();
  });
</script>

<svelte:boundary onerror={(error) => console.error(error)}>
{#if !app.ready}
  <main aria-busy="true"><p>{t(app.locale, "app.name")}…</p></main>
{:else if !app.me || !app.cohort}
  <SignIn />
{:else}
  {#await app.cohort.role === "teacher" ? Studio() : Learner() then screen}
    <screen.default />
  {/await}
{/if}
  {#snippet failed(_error, reset)}
    <main><p role="alert">{t(app.locale, "error.generic")}</p><button onclick={reset}>↻</button></main>
  {/snippet}
</svelte:boundary>
