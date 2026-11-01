<script lang="ts">
  import { app } from "../lib/app.svelte.ts";
  import { t } from "../lib/i18n/i18n.ts";
  import type { Activity as ActivityData } from "../lib/kernel/types/Activity.ts";
  import type { LearnerView } from "../lib/kernel/types/LearnerView.ts";
  import Activity from "./Activity.svelte";

  let { view, outcome, onclose }: { view: LearnerView; outcome: string; onclose: () => void } = $props();
  let open: ActivityData | undefined = $state();

  const info = $derived(view.outcomes.find((o) => o.outcome === outcome)!);
  const activities = $derived(
    (app.pack?.activities ?? []).filter(
      (a) => a.outcomes.includes(outcome) && (a.role !== "remediation" || view.assigned.includes(a.id)),
    ),
  );
  const ICONS = { video: "▶", reading: "📖", quiz: "✏️", open_response: "💬", explore: "🧩" } as const;
</script>

<section class="card" aria-labelledby="outcome-title">
  <button onclick={() => (open ? (open = undefined) : onclose())}>← {t(app.locale, "activity.back")}</button>
  <h2 id="outcome-title">{info.title}</h2>
  <p class="state-{info.judgement.state}">{t(app.locale, `state.${info.judgement.state}`)}</p>
  {#if open}
    <Activity activity={open} ondone={() => (open = undefined)} />
  {:else}
    <ul class="activities">
      {#each activities as a (a.id)}
        <li>
          <button onclick={() => (open = a)} class:help={a.role === "remediation"}>
            <span aria-hidden="true">{ICONS[a.kind.type]}</span>
            <span>{a.title[app.locale] ?? a.title.en}</span>
            <small>{a.role === "remediation" ? t(app.locale, "activity.help") : t(app.locale, `activity.${a.kind.type}`)}</small>
          </button>
        </li>
      {/each}
    </ul>
  {/if}
</section>

<style>
  .activities { list-style: none; padding: 0; display: grid; gap: 0.75rem; }
  .activities button { width: 100%; display: flex; gap: 0.75rem; align-items: center; text-align: start; }
  .activities small { margin-inline-start: auto; color: var(--locked); font-size: 16px; }
  .help { border-color: var(--needs-support); }
</style>
