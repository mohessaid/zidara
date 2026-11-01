<script lang="ts">
  import { app } from "../lib/app.svelte.ts";
  import { t } from "../lib/i18n/i18n.ts";
  import Header from "../components/Header.svelte";
  import Grid from "./Grid.svelte";
  import { startsLight, type DeviceHints } from "../lib/device.ts";
  import { streak, xp } from "../play/xp.ts";

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
  // A session, and a live round, load their code when first needed.
  const session = () => import("../play/Session.svelte");
  const livePhone = () => import("../play/LivePhone.svelte");
  // Fetched at once in the background: a phone that loses the network before opening a lesson
  // must still have the lesson's code (offline first).
  void session().catch(() => undefined);
  void livePhone().catch(() => undefined);
  const events = $derived(app.events());
  const points = $derived(app.me && app.kernel ? xp(events, app.me.id, (a, v, i, r) => app.kernel!.check(a, v, i, r)) : 0);
  const days = $derived(app.me ? streak(events, app.me.id, now) : 0);
  // While a round runs, the phone asks the server every second and a half.
  $effect(() => app.followLive(Boolean(view?.challenge)));
</script>

<Header title={app.pack?.title[app.locale] ?? t(app.locale, "quest.title")}>
  <div class="row" role="group" aria-label={t(app.locale, "quest.map")}>
    <button aria-pressed={experience === "quest"} onclick={showMap}>{t(app.locale, "view.quest")}</button>
    <button aria-pressed={experience === "grid"} onclick={() => (experience = "grid")}>{t(app.locale, "view.grid")}</button>
  </div>
</Header>

<main>
  {#if view}
    <div class="hud" aria-label={t(app.locale, "hud.label")}>
      <span class="chip" title={t(app.locale, "finish.streak")}><span aria-hidden="true">🔥</span> {t(app.locale, "hud.streak", { n: days })}</span>
      <span class="chip" title={t(app.locale, "finish.total")}><span aria-hidden="true">⭐</span> {t(app.locale, "hud.xp", { n: points })}</span>
    </div>
    {#if view.challenge}
      {#await livePhone() then live}<live.default live={view.challenge} />{/await}
    {/if}
    {#if selected}
      {#await session() then play}
        <play.default outcome={selected} onclose={() => (selected = undefined)} />
      {/await}
    {/if}
    <!-- The map stays mounted, and keeps running, while a panel is open. Re-creating the WebGL
         renderer on every return cost a 350-405 ms long task on the economy profile; pausing
         or slowing its ticker while hidden moved a 50-80 ms GPU upload into the first frame back. -->
    {#if map}
      <div hidden={experience !== "quest"}>
        {#await map then quest}
          <quest.default {view} onopen={(id: string) => (selected = id)} />
        {:catch}
          <!-- Handled in loadMap: the table takes over. -->
        {/await}
      </div>
    {/if}
    {#if mapUnavailable}
      <p role="status" class="card">{t(app.locale, "map.unavailable")}</p>
    {/if}
    {#if experience === "grid"}
      <Grid {view} onopen={(id) => (selected = id)} />
    {/if}
  {/if}
</main>

<style>
  [aria-pressed="true"] { background: var(--ink); color: #fff; }
  .hud { display: flex; gap: 0.5rem; justify-content: flex-end; margin-bottom: 0.5rem; }
  .chip { background: var(--surface); border: 2px solid #e6dcc8; border-radius: 999px; padding: 0.2rem 0.9rem; font-weight: 700; }
</style>
