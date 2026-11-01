<script lang="ts">
  // The learner's home: a game bar (course, streak, points), then the path. A session, and a
  // live round when the teacher starts one, open over it. Points and the streak are read from
  // the evidence log (ADR 0011); nothing here is stored on its own.
  import { app } from "../lib/app.svelte.ts";
  import { t } from "../lib/i18n/i18n.ts";
  import LanguagePicker from "../components/LanguagePicker.svelte";
  import SyncBadge from "../components/SyncBadge.svelte";
  import CourseIcon from "../play/CourseIcon.svelte";
  import { streak, xp } from "../play/xp.ts";
  import Path from "./Path.svelte";

  let selected: string | undefined = $state();
  let menu = $state(false);
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
  // `events` is read first: `app.kernel` is not reactive, so a derived that short-circuited on
  // it before the kernel arrived would never recompute (points stuck at 0 until a reload).
  const points = $derived.by(() => {
    const log = events;
    return app.me && app.kernel ? xp(log, app.me.id, (a, v, i, r) => app.kernel!.check(a, v, i, r)) : 0;
  });
  const days = $derived(app.me ? streak(events, app.me.id, now) : 0);
  const courses = $derived(app.me?.cohorts.filter((c) => c.role === "learner") ?? []);
  const COURSE_NAMES: Record<string, Record<string, string>> = {
    "equivalent-fractions": { ar: "الكسور المتكافئة", fr: "Fractions équivalentes", en: "Equivalent fractions" },
    "how-zidara-works": { ar: "كيف تعمل زيدارا", fr: "Comment marche Zidara", en: "How Zidara works" },
  };
  const courseName = (lesson: string, fallback: string) => COURSE_NAMES[lesson]?.[app.locale] ?? fallback;
  // While a round runs, the phone asks the server every second and a half.
  $effect(() => app.followLive(Boolean(view?.challenge)));

  async function switchTo(id: string) {
    menu = false;
    await app.switchCohort(id);
  }
</script>

<header class="bar">
  <button class="pill course" aria-expanded={menu} aria-controls="menu" onclick={() => (menu = !menu)}>
    <CourseIcon lesson={app.cohort?.lesson_id ?? ""} size={30} />
    <span class="visually-hidden">{t(app.locale, "path.menu")}</span>
  </button>
  <h1 class="title">{app.pack?.title[app.locale] ?? t(app.locale, "quest.title")}</h1>
  <div class="hud" role="group" aria-label={t(app.locale, "hud.label")}>
    <span class="chip flame" title={t(app.locale, "finish.streak")}><span aria-hidden="true">🔥</span> <span class="visually-hidden">{t(app.locale, "hud.streak", { n: days })}</span><span aria-hidden="true">{days}</span></span>
    <span class="chip gem" title={t(app.locale, "finish.total")}><span aria-hidden="true">⚡</span> <span class="visually-hidden">{t(app.locale, "hud.xp", { n: points })}</span><span aria-hidden="true">{points}</span></span>
  </div>
</header>

{#if menu}
  <div class="menu card" id="menu">
    {#if courses.length > 1}
      <h2>{t(app.locale, "path.courses")}</h2>
      <ul class="courses">
        {#each courses as c (c.id)}
          <li>
            <button class="course-row" aria-current={c.id === app.cohort?.id ? "true" : undefined} onclick={() => switchTo(c.id)}>
              <CourseIcon lesson={c.lesson_id} size={40} /> {courseName(c.lesson_id, c.name)}
            </button>
          </li>
        {/each}
      </ul>
    {/if}
    <div class="row"><LanguagePicker /><button onclick={() => app.signOut()}>{t(app.locale, "signin.out")}</button></div>
  </div>
{/if}

<main>
  <div class="status"><SyncBadge /></div>
  {#if view}
    {#if view.challenge}
      {#await livePhone() then live}<live.default live={view.challenge} />{/await}
    {/if}
    {#if selected}
      {#await session() then play}
        <play.default outcome={selected} onclose={() => (selected = undefined)} />
      {/await}
    {/if}
    <Path {view} onopen={(id: string) => (selected = id)} />
  {/if}
</main>

<style>
  .bar {
    position: sticky; top: 0; z-index: 5; display: flex; align-items: center; gap: 0.5rem; padding: 0.5rem 1rem;
    background: var(--surface); border-bottom: 2px solid var(--line);
  }
  .title { flex: 1; font-size: 20px; margin: 0; white-space: nowrap; overflow: hidden; text-overflow: ellipsis; }
  .pill { padding: 0.25rem 0.5rem; display: flex; align-items: center; gap: 0.25rem; }
  .hud { display: flex; gap: 0.5rem; }
  .chip { display: flex; align-items: center; gap: 0.2rem; font-weight: 800; font-size: 20px; padding: 0 0.25rem; }
  .flame { color: #c25e00; }
  .gem { color: var(--in-progress); }
  .menu { position: sticky; top: 66px; z-index: 4; max-width: 560px; margin: 0.5rem auto; display: grid; gap: 0.75rem; box-shadow: 0 8px 24px rgba(0, 0, 0, 0.08); }
  .menu h2 { margin: 0; font-size: 20px; }
  .courses { list-style: none; margin: 0; padding: 0; display: grid; gap: 0.5rem; }
  .course-row { width: 100%; display: flex; align-items: center; gap: 0.75rem; text-align: start; }
  .course-row[aria-current="true"] { border-color: #84d8ff; background: var(--sky); border-bottom-color: #4fb8ef; }
  .status { display: flex; justify-content: center; margin-bottom: 0.75rem; }
</style>
