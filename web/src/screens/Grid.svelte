<script lang="ts">
  import { app } from "../lib/app.svelte.ts";
  import { t } from "../lib/i18n/i18n.ts";
  import type { LearnerView } from "../lib/kernel/types/LearnerView.ts";

  let { view, onopen }: { view: LearnerView; onopen: (outcome: string) => void } = $props();
</script>

<table class="card">
  <thead>
    <tr><th scope="col">{t(app.locale, "grid.outcome")}</th><th scope="col">{t(app.locale, "grid.state")}</th><th scope="col">{t(app.locale, "grid.score")}</th><th scope="col">{t(app.locale, "grid.attempts")}</th></tr>
  </thead>
  <tbody>
    {#each view.outcomes as o (o.outcome)}
      <tr>
        <th scope="row"><button disabled={o.lock !== null} onclick={() => onopen(o.outcome)}>{o.outcome} · {o.title}</button></th>
        <td class={o.lock ? "state-locked" : `state-${o.judgement.state}`}>{o.lock ? "🔒" : ""} {t(app.locale, `state.${o.judgement.state}`)}</td>
        <td>{o.judgement.score === null ? "—" : `${Math.round(o.judgement.score / 100)}%`}</td>
        <td>{o.judgement.attempts}</td>
      </tr>
    {/each}
  </tbody>
</table>

<style>
  table { width: 100%; border-collapse: collapse; }
  th, td { padding: 0.5rem; text-align: start; border-bottom: 1px solid #e6dcc8; }
  th button { width: 100%; text-align: start; }
</style>
