<script lang="ts">
  import { onMount } from "svelte";
  import { app } from "../lib/app.svelte.ts";
  import type { Person } from "../lib/api.ts";
  import { t } from "../lib/i18n/i18n.ts";
  import type { Command } from "../lib/kernel/types/Command.ts";
  import type { StudioView } from "../lib/kernel/types/StudioView.ts";
  import Header from "../components/Header.svelte";
  import LiveBoard from "./LiveBoard.svelte";

  let studio: StudioView | undefined = $state();
  let people: Person[] = $state([]);
  let framework = $state(app.cohort!.framework);
  let busy = $state(false);

  const name = (id: string) => people.find((p) => p.id === id)?.display_name ?? id.slice(-4);
  const outcomeTitle = (id: string) => studio?.outcomes.find((o) => o.outcome === id)?.title ?? id;
  const pct = (bp: number) => Math.round(bp / 100);

  async function refresh() {
    const cohort = app.cohort!.id;
    try {
      [studio, people] = await Promise.all([app.api!.studio(cohort, app.locale), app.api!.roster(cohort)]);
      framework = studio.framework;
    } catch {
      app.sync.online = false;
    }
  }

  let refusal: string | undefined = $state();

  // The list on screen comes from the server; the command runs through this device's kernel
  // first. Sync before acting, so the kernel knows what the teacher is looking at (a proposal
  // the agent made after she opened the studio, for example).
  async function command(c: Command) {
    busy = true;
    refusal = undefined;
    await app.syncNow();
    const result = await app.run(c);
    if (result.status === "refused") refusal = t(app.locale, "error.generic");
    await refresh();
    busy = false;
  }

  // Every 5 s, and every 1.5 s while a live round runs so the board keeps pace with the class.
  onMount(() => {
    void refresh();
    let ticks = 0;
    const timer = setInterval(() => {
      ticks++;
      if (studio?.challenge || ticks % 3 === 0) void refresh();
    }, 1_500);
    return () => clearInterval(timer);
  });
  $effect(() => {
    void app.locale;
    void refresh();
  });
</script>

<Header title={t(app.locale, "studio.title")} />

<main class="studio" data-framework={studio?.framework}>
  {#if refusal}<p role="alert" class="state-needs_support">{refusal}</p>{/if}
  {#if studio}
    <LiveBoard tally={studio.challenge} learners={people.length} {busy} run={command} />

    <section class="card" aria-labelledby="progress">
      <h2 id="progress">{t(app.locale, "studio.progress")}</h2>
      <ul class="bars">
        {#each studio.outcomes as o (o.outcome)}
          <li>
            <span class="title">{o.outcome} · {o.title}</span>
            <span class="bar" role="img" aria-label={t(app.locale, "studio.achieved", { n: o.achieved, of: o.of })}>
              <span class="fill" style="inline-size: {o.of === 0 ? 0 : (100 * o.achieved) / o.of}%"></span>
            </span>
            <span>{t(app.locale, "studio.achieved", { n: o.achieved, of: o.of })}</span>
            {#if o.needs_support > 0}<span class="state-needs_support">· {t(app.locale, "studio.support", { n: o.needs_support })}</span>{/if}
          </li>
        {/each}
      </ul>
    </section>

    <section class="card" aria-labelledby="queue">
      <h2 id="queue">{t(app.locale, "studio.queue")}</h2>
      {#if studio.queue.length === 0}<p>{t(app.locale, "studio.queue.empty")}</p>{/if}
      {#each studio.queue as q (q.id)}
        <article class="proposal" data-testid="proposal">
          <p><strong>{name(q.proposal.learner)}</strong> · {outcomeTitle(q.proposal.outcome)}</p>
          <blockquote><bdi>{q.answer}</bdi></blockquote>
          <p class="agent">🤖 {t(app.locale, "studio.agent.proposes", { score: pct(q.proposal.assessment.score), confidence: pct(q.proposal.assessment.confidence) })}<br /><small>{q.proposal.assessment.met
              ? t(app.locale, "studio.agent.found", { met: q.proposal.assessment.met.criteria, of: q.proposal.assessment.met.of, words: q.proposal.assessment.met.words })
              : q.proposal.assessment.rationale}</small></p>
          <div class="row">
            <button class="primary" disabled={busy} onclick={() => command({ type: "decide_proposal", proposal: q.id, accept: true })}>{t(app.locale, "studio.accept")}</button>
            <button disabled={busy} onclick={() => command({ type: "decide_proposal", proposal: q.id, accept: false })}>{t(app.locale, "studio.reject")}</button>
          </div>
        </article>
      {/each}
    </section>

    {#if studio.flags.length > 0}
      <section class="card" aria-labelledby="flags">
        <h2 id="flags">{t(app.locale, "studio.flags")}</h2>
        <ul>
          {#each studio.flags as f (`${f.learner}-${f.outcome}-${f.reason}`)}
            <li class="state-needs_support">{t(app.locale, `flag.${f.reason}`, { name: name(f.learner), outcome: outcomeTitle(f.outcome) })}</li>
          {/each}
        </ul>
      </section>
    {/if}

    {#if studio.agent_actions.length > 0}
      <section class="card" aria-labelledby="agent">
        <h2 id="agent">🤖 {t(app.locale, "studio.agent.actions")}</h2>
        <ul>
          {#each studio.agent_actions as a (a.id)}
            <li class="row agent">
              <span>{t(app.locale, "studio.agent.marked", { name: name(a.evidence.learner), score: pct(a.evidence.score) })} · {outcomeTitle(a.evidence.outcome)}</span>
              <button disabled={busy} onclick={() => command({ type: "undo", target: a.id })}>{t(app.locale, "studio.undo")}</button>
            </li>
          {/each}
        </ul>
      </section>
    {/if}

    <section class="card" aria-labelledby="framework">
      <h2 id="framework">{t(app.locale, "studio.framework")}</h2>
      <form class="row" onsubmit={(e) => (e.preventDefault(), command({ type: "change_framework", framework }))}>
        <label><span class="visually-hidden">{t(app.locale, "studio.framework")}</span>
          <select bind:value={framework}>
            <option value="mastery">{t(app.locale, "framework.mastery")}</option>
            <option value="cambridge-style">{t(app.locale, "framework.cambridge-style")}</option>
          </select>
        </label>
        <button class="primary" type="submit" disabled={busy || framework === studio.framework}>{t(app.locale, "studio.switch")}</button>
      </form>
    </section>
  {/if}
</main>

<style>
  .studio { display: grid; gap: 1rem; font-size: 16px; }
  .studio :global(.card) { border-radius: var(--radius-studio); }
  .bars { list-style: none; padding: 0; display: grid; gap: 0.5rem; }
  .bars li { display: grid; grid-template-columns: minmax(10rem, 1fr) 2fr auto auto; gap: 0.5rem; align-items: center; }
  .bar { block-size: 12px; background: #e6dcc8; border-radius: 6px; overflow: hidden; }
  .fill { display: block; block-size: 100%; background: var(--achieved); }
  .proposal { border-top: 1px solid #e6dcc8; padding-top: 0.5rem; }
  blockquote { margin: 0.5rem 0; padding: 0.5rem 1rem; border-inline-start: 4px solid var(--ink); background: var(--paper); }
  .agent { color: var(--agent); }
  select { min-height: 48px; border-radius: var(--radius-studio); border: 2px solid var(--ink); padding: 0 0.5rem; }
</style>
