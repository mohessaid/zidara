<script lang="ts">
  import { app } from "../lib/app.svelte.ts";
  import { t } from "../lib/i18n/i18n.ts";
  import Header from "../components/Header.svelte";
  import Quest from "./Quest.svelte";
  import Grid from "./Grid.svelte";
  import OutcomePanel from "./OutcomePanel.svelte";

  let experience: "quest" | "grid" = $state("quest");
  let selected: string | undefined = $state();
  let now = $state(Date.now());
  setInterval(() => (now = Date.now()), 60_000);
  const view = $derived(app.learnerView(now));
</script>

<Header title={t(app.locale, "quest.title")}>
  <div class="row" role="group" aria-label={t(app.locale, "quest.map")}>
    <button aria-pressed={experience === "quest"} onclick={() => (experience = "quest")}>{t(app.locale, "view.quest")}</button>
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
    <div hidden={selected !== undefined || experience !== "quest"}>
      <Quest {view} onopen={(id) => (selected = id)} />
    </div>
    {#if !selected && experience === "grid"}
      <Grid {view} onopen={(id) => (selected = id)} />
    {/if}
  {/if}
</main>

<style>
  [aria-pressed="true"] { background: var(--ink); color: #fff; }
</style>
