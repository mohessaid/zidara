<script lang="ts">
  // The front door: no list of people, no form. One tap on a course starts a guest learner in
  // it (ADR 0012); the demo classes for teachers sit behind a small link (#classes).
  import { app } from "../lib/app.svelte.ts";
  import { t, type Key } from "../lib/i18n/i18n.ts";
  import LanguagePicker from "../components/LanguagePicker.svelte";
  import CourseIcon from "../play/CourseIcon.svelte";
  import Mascot from "../play/Mascot.svelte";
  import { sound } from "../play/sound.svelte.ts";

  const COURSES = [
    { lesson: "how-zidara-works", key: "concept", title: { ar: "كيف تعمل زيدارا", fr: "Comment marche Zidara", en: "How Zidara works" } },
    { lesson: "equivalent-fractions", key: "fractions", title: { ar: "الكسور المتكافئة", fr: "Fractions équivalentes", en: "Equivalent fractions" } },
  ] as const;

  let busy: string | undefined = $state();

  async function start(lesson: string) {
    if (busy) return;
    busy = lesson;
    sound.play("tap");
    try {
      await app.guest(lesson);
    } catch {
      app.error = t(app.locale, "error.generic");
    } finally {
      busy = undefined;
    }
  }
</script>

<div class="top">
  <span class="logo" lang="en">zidara</span>
  <LanguagePicker />
</div>

<main class="welcome">
  <section class="hero">
    <div class="speaker">
      <Mascot size={140} mood={busy ? "cheer" : "idle"} />
      <p class="bubble">{t(app.locale, "welcome.hello")}</p>
    </div>
    <h1>{t(app.locale, "welcome.title")}</h1>
    <p class="lead">{t(app.locale, "welcome.lead")}</p>
  </section>

  {#if app.error}<p role="alert" class="card alert">{app.error}</p>{/if}

  <h2>{t(app.locale, "welcome.pick")}</h2>
  <ul class="courses">
    {#each COURSES as course (course.lesson)}
      <li>
        <button class="course" data-lesson={course.lesson} disabled={busy !== undefined} onclick={() => start(course.lesson)}>
          <CourseIcon lesson={course.lesson} size={72} />
          <span class="text">
            <span class="name">{course.title[app.locale]}</span>
            <span class="blurb">{t(app.locale, `course.${course.key}.blurb` as Key)}</span>
            <span class="meta">{t(app.locale, `course.${course.key}.meta` as Key)}</span>
          </span>
          <span class="go" aria-hidden="true">{busy === course.lesson ? "…" : "›"}</span>
        </button>
      </li>
    {/each}
  </ul>
  {#if busy}<p role="status" class="busy">{t(app.locale, "welcome.busy")}</p>{/if}

  <ul class="points">
    <li><span aria-hidden="true">📶</span> {t(app.locale, "welcome.offline")}</li>
    <li><span aria-hidden="true">🌍</span> {t(app.locale, "welcome.languages")}</li>
    <li><span aria-hidden="true">🔒</span> {t(app.locale, "welcome.free")}</li>
  </ul>

  <p class="teachers"><a href="#classes">{t(app.locale, "welcome.teachers")}</a></p>
</main>

<style>
  .top { display: flex; justify-content: space-between; align-items: center; padding: 0.5rem 1rem; max-width: 720px; margin: 0 auto; }
  .logo { font-size: 32px; font-weight: 800; color: var(--go); letter-spacing: -0.02em; }
  .welcome { max-width: 720px; }
  .hero { text-align: center; display: grid; justify-items: center; gap: 0.25rem; }
  .speaker { display: flex; align-items: center; gap: 0.75rem; }
  .bubble {
    position: relative; margin: 0; max-width: 15rem; padding: 0.75rem 1rem; border: 2px solid var(--line);
    border-radius: var(--radius); background: var(--surface); font-weight: 700; text-align: start;
  }
  .bubble::before {
    content: ""; position: absolute; inset-inline-start: -10px; top: 50%; width: 16px; height: 16px; margin-top: -8px;
    background: var(--surface); border-inline-start: 2px solid var(--line); border-bottom: 2px solid var(--line); transform: rotate(45deg);
  }
  :global([dir="rtl"]) .bubble::before { transform: rotate(-45deg); }
  h1 { font-size: 40px; margin: 0.5rem 0 0; }
  .lead { font-size: 20px; color: var(--muted); max-width: 34rem; margin: 0.25rem auto 0; }
  h2 { text-align: center; margin-top: 1.5rem; }
  .courses { list-style: none; margin: 0; padding: 0; display: grid; gap: 1rem; }
  .course {
    width: 100%; display: flex; align-items: center; gap: 1rem; padding: 1rem; text-align: start;
    border-width: 2px 2px 5px; border-radius: 20px;
  }
  .course:hover:not(:disabled) { background: #f7fbff; border-color: #84d8ff; border-bottom-color: #4fb8ef; }
  .text { flex: 1; display: grid; gap: 0.1rem; }
  .name { font-size: 24px; font-weight: 800; line-height: 1.2; }
  .blurb { font-size: 18px; font-weight: 500; }
  .meta { font-size: 16px; color: var(--muted); font-weight: 600; }
  .go { font-size: 40px; color: var(--in-progress); line-height: 1; }
  :global([dir="rtl"]) .go { transform: scaleX(-1); }
  .busy { text-align: center; font-weight: 700; color: var(--achieved); }
  .alert { border-color: var(--needs-support); color: var(--needs-support); }
  .points { list-style: none; padding: 0; margin: 1.5rem 0 0; display: flex; flex-wrap: wrap; justify-content: center; gap: 0.5rem 1.25rem; color: var(--muted); font-weight: 600; }
  .teachers { text-align: center; margin-top: 1.5rem; }
  .teachers a { color: var(--blue-edge); font-weight: 700; }
  @media (max-width: 420px) { h1 { font-size: 32px; } .bubble { max-width: 11rem; font-size: 16px; } }
</style>
