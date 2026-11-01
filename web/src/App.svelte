<script lang="ts">
  import { onMount } from "svelte";
  import { app } from "./lib/app.svelte.ts";
  import { t } from "./lib/i18n/i18n.ts";
  import Welcome from "./screens/Welcome.svelte";
  // Everything else loads on demand: the front door needs no map, no session and no studio.
  const SignIn = () => import("./screens/SignIn.svelte");
  const Learner = () => import("./screens/Learner.svelte");
  const Studio = () => import("./screens/Studio.svelte");

  // The demo classes (named people, teachers) live at #classes; everyone else starts as a guest.
  let hash = $state(location.hash);
  onMount(() => {
    app.setLocale(app.locale);
    void app.start();
    const follow = () => (hash = location.hash);
    addEventListener("hashchange", follow);
    return () => removeEventListener("hashchange", follow);
  });
</script>

<svelte:boundary onerror={(error) => console.error(error)}>
{#if !app.ready}
  <main aria-busy="true"><p>{t(app.locale, "app.name")}…</p></main>
{:else if !app.me || !app.cohort}
  {#if hash === "#classes"}
    {#await SignIn() then screen}<screen.default />{/await}
  {:else}
    <Welcome />
  {/if}
{:else}
  {#await app.cohort.role === "teacher" ? Studio() : Learner() then screen}
    <screen.default />
  {/await}
{/if}
  {#snippet failed(_error, reset)}
    <main><p role="alert">{t(app.locale, "error.generic")}</p><button onclick={reset}>↻</button></main>
  {/snippet}
</svelte:boundary>
