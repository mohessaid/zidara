<script lang="ts">
  import { onMount } from "svelte";
  import { app } from "./lib/app.svelte.ts";
  import { t } from "./lib/i18n/i18n.ts";
  import Landing from "./landing/Landing.svelte";
  // Everything else loads on demand: the front page needs no game, no map and no studio.
  const Welcome = () => import("./screens/Welcome.svelte");
  const SignIn = () => import("./screens/SignIn.svelte");
  const Learner = () => import("./screens/Learner.svelte");
  const Studio = () => import("./screens/Studio.svelte");

  // Routes (ADR 0014): the front page tells the story at `/` (its own anchors included); the
  // game lives at #play, where a visitor starts as a guest; the demo classes are at #classes.
  let hash = $state(location.hash);
  const inApp = $derived(hash === "#play" || hash === "#classes");
  onMount(() => {
    app.setLocale(app.locale);
    void app.start();
    const follow = () => {
      const was = inApp;
      hash = location.hash;
      // Entering or leaving the game starts at the top of the screen, not halfway down the page.
      if (was !== (hash === "#play" || hash === "#classes")) scrollTo(0, 0);
    };
    addEventListener("hashchange", follow);
    return () => removeEventListener("hashchange", follow);
  });
</script>

<svelte:boundary onerror={(error) => console.error(error)}>
{#if !inApp}
  <Landing />
{:else if !app.ready}
  <main aria-busy="true"><p>{t(app.locale, "app.name")}…</p></main>
{:else if !app.me || !app.cohort}
  {#await hash === "#classes" ? SignIn() : Welcome() then screen}<screen.default />{/await}
{:else}
  {#await app.cohort.role === "teacher" ? Studio() : Learner() then screen}
    <screen.default />
  {/await}
{/if}
  {#snippet failed(_error, reset)}
    <main><p role="alert">{t(app.locale, "error.generic")}</p><button onclick={reset}>↻</button></main>
  {/snippet}
</svelte:boundary>
