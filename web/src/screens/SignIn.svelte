<script lang="ts">
  import { app } from "../lib/app.svelte.ts";
  import { DEMO_PEOPLE } from "../lib/demo.ts";
  import { t } from "../lib/i18n/i18n.ts";
  import LanguagePicker from "../components/LanguagePicker.svelte";

  let busy = $state(false);
  const groups = [
    { key: "signin.learners", people: DEMO_PEOPLE.filter((p) => p.role === "learner") },
    { key: "signin.teachers", people: DEMO_PEOPLE.filter((p) => p.role === "teacher") },
  ] as const;

  async function choose(id: string) {
    busy = true;
    try {
      await app.signIn(id);
    } catch {
      app.error = t(app.locale, "error.generic");
    } finally {
      busy = false;
    }
  }
</script>

<main>
  <div class="row"><h1>{t(app.locale, "signin.title")}</h1><span class="spacer"></span><LanguagePicker /></div>
  {#if app.error}<p role="alert" class="state-needs_support">{app.error}</p>{/if}
  {#each groups as group (group.key)}
    <h2>{t(app.locale, group.key)}</h2>
    <ul class="people">
      {#each group.people as person (person.id)}
        <li>
          <!-- The visible name stays inside the accessible name (WCAG 2.5.3): the prefix is hidden text, not aria-label. -->
          <button disabled={busy} onclick={() => choose(person.id)}>
            <span class="avatar" aria-hidden="true">{person.name[app.locale].slice(0, 1)}</span>
            <span><span class="visually-hidden">{t(app.locale, "signin.as", { name: "" }).trim()} </span>{person.name[app.locale]}</span>
            <small>{person.cohort}</small>
          </button>
        </li>
      {/each}
    </ul>
  {/each}
</main>

<style>
  .spacer { flex: 1; }
  .people { list-style: none; padding: 0; display: grid; grid-template-columns: repeat(auto-fill, minmax(160px, 1fr)); gap: 1rem; }
  .people button { width: 100%; display: flex; flex-direction: column; align-items: center; gap: 0.25rem; padding: 1rem; }
  .avatar { width: 56px; height: 56px; border-radius: 50%; background: var(--in-progress); color: #fff; display: grid; place-items: center; font-size: 24px; font-weight: 700; }
  small { font-size: 16px; color: var(--locked); }
</style>
