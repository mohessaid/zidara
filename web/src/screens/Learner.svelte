<script lang="ts">
  import { app } from "../lib/app.svelte.ts";
  import { t } from "../lib/i18n/i18n.ts";
  import Header from "../components/Header.svelte";
  import Grid from "./Grid.svelte";
  import OutcomePanel from "./OutcomePanel.svelte";
  import { startsLight, type DeviceHints } from "../lib/device.ts";

  // The map (PixiJS) loads only when first shown: low-end phones and data saver start light.
  // If its code cannot arrive (the connection dropped after sign-in), the table takes over and
  // the map button tries again.
  const light = startsLight(navigator as DeviceHints);
  let experience: "quest" | "grid" = $state(light ? "grid" : "quest");
  let mapUnavailable = $state(false);
  function loadMap() {
    const loading = import("./Quest.svelte");
    loading.catch(() => {
      map = undefined;
      experience = "grid";
      mapUnavailable = true;
    });
    return loading;
  }
  // Once loaded, the map stays mounted (see below).
  let map = $state(light ? undefined : loadMap());
  function showMap() {
    experience = "quest";
    mapUnavailable = false;
    map ??= loadMap();
  }
  let selected: string | undefined = $state();
  let now = $state(Date.now());
  setInterval(() => (now = Date.now()), 60_000);
  const view = $derived(app.learnerView(now));
</script>

<Header title={t(app.locale, "quest.title")}>
  <div class="row" role="group" aria-label={t(app.locale, "quest.map")}>
    <button aria-pressed={experience === "quest"} onclick={showMap}>{t(app.locale, "view.quest")}</button>
    <button aria-pressed={experience === "grid"} onclick={() => (experience = "grid")}>{t(app.locale, "view.grid")}</button>
  </div>
</Header>

<main>
  {#if view}
    {#if selected}
      <OutcomePanel {view} outcome={selected} onclose={() => (selected = undefined)} />
    {/if}
    <!-- The map stays mounted, and keeps running, while a panel is open. Re-creating the WebGL
         renderer on every return cost a 350-405 ms long task on the economy profile; pausing
         or slowing its ticker while hidden moved a 50-80 ms GPU upload into the first frame back. -->
    {#if map}
      <div hidden={selected !== undefined || experience !== "quest"}>
        {#await map then quest}
          <quest.default {view} onopen={(id: string) => (selected = id)} />
        {:catch}
          <!-- Handled in loadMap: the table takes over. -->
        {/await}
      </div>
    {/if}
    {#if mapUnavailable && !selected}
      <p role="status" class="card">{t(app.locale, "map.unavailable")}</p>
    {/if}
    {#if !selected && experience === "grid"}
      <Grid {view} onopen={(id) => (selected = id)} />
    {/if}
  {/if}
</main>

<style>
  [aria-pressed="true"] { background: var(--ink); color: #fff; }
</style>
