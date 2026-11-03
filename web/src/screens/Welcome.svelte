<script lang="ts">
  // The demo's front door (#play), in the manner of Brilliant (ADR 0016): a heading, two course
  // cards and nothing else to read past. A tap on a card starts a guest learner in that course
  // (ADR 0012); the demo classes for teachers sit behind a small link (#classes).
  import { app } from "../lib/app.svelte.ts";
  import { t, type Key } from "../lib/i18n/i18n.ts";
  import LanguagePicker from "../components/LanguagePicker.svelte";
  import CourseIcon from "../play/CourseIcon.svelte";
  import { sound } from "../play/sound.svelte.ts";

  const COURSES = [
    { lesson: "how-zidara-works", key: "concept", tone: "teal", title: { ar: "كيف تعمل زيدارا", fr: "Comment marche Zidara", en: "How Zidara works" } },
    { lesson: "equivalent-fractions", key: "fractions", tone: "terra", title: { ar: "الكسور المتكافئة", fr: "Fractions équivalentes", en: "Equivalent fractions" } },
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
  <a class="logo" href="#top" lang="en">zidara</a>
  <LanguagePicker />
</div>

<main class="welcome">
  <h1>{t(app.locale, "welcome.title")}</h1>
  <p class="lead">{t(app.locale, "welcome.lead")}</p>

  {#if app.error}<p role="alert" class="card alert">{app.error}</p>{/if}

  <h2>{t(app.locale, "welcome.pick")}</h2>
  <ul class="doors">
    {#each COURSES as course (course.lesson)}
      <li>
        <button class="door {course.tone}" data-lesson={course.lesson} disabled={busy !== undefined} onclick={() => start(course.lesson)}>
          <CourseIcon lesson={course.lesson} size={72} />
          <span class="text">
            <span class="name">{course.title[app.locale]}</span>
            <span class="blurb">{t(app.locale, `course.${course.key}.blurb` as Key)}</span>
            <span class="meta">{t(app.locale, `course.${course.key}.meta` as Key)}</span>
          </span>
          <span class="enter" aria-hidden="true">{busy === course.lesson ? "…" : t(app.locale, "welcome.enter")}</span>
        </button>
      </li>
    {/each}
  </ul>
  {#if busy}<p role="status" class="busy">{t(app.locale, "welcome.busy")}</p>{/if}

  <ul class="points">
    <li>{t(app.locale, "welcome.offline")}</li>
    <li>{t(app.locale, "welcome.languages")}</li>
    <li>{t(app.locale, "welcome.free")}</li>
  </ul>

  <p class="teachers"><a href="#classes">{t(app.locale, "welcome.teachers")}</a></p>
</main>

<style>
  .top { display: flex; justify-content: space-between; align-items: center; padding: 12px 16px; max-width: 760px; margin: 0 auto; }
  .logo { display: inline-flex; align-items: center; gap: 8px; font: 800 22px/1 var(--display); color: var(--ink); letter-spacing: -0.02em; text-decoration: none; }
  .welcome { max-width: 640px; padding-top: 40px; }
  h1 { font-size: clamp(38px, 9vw, 56px); margin: 0; line-height: 1.08; font-weight: 900; }
  .lead { font-size: 19px; color: var(--muted); max-width: 32rem; margin: 12px 0 0; }
  h2 { margin: 36px 0 12px; font-size: 18px; font-weight: 700; }
  .doors { list-style: none; margin: 0; padding: 0; display: grid; gap: 12px; }
  /* A course card: the badge, the words, and a Start button at the end of the row. */
  .door {
    width: 100%; display: flex; align-items: center; gap: 16px; padding: 16px; text-align: start; color: var(--ink);
    border-radius: 16px; border: 2px solid var(--edge); box-shadow: 0 4px 0 var(--edge); background: var(--surface);
  }
  .door:hover:not(:disabled) { border-color: var(--ink); box-shadow: 0 4px 0 var(--ink); }
  .door:active:not(:disabled) { transform: translateY(3px); box-shadow: 0 1px 0 var(--ink); }
  .text { display: grid; gap: 3px; flex: 1; min-width: 0; }
  .name { font: 700 20px/1.2 var(--display); }
  .blurb { font-size: 16px; font-weight: 400; line-height: 1.4; color: var(--muted); }
  .meta { font-size: 14px; font-weight: 500; color: var(--muted); }
  .enter { flex: none; padding: 10px 18px; border-radius: 12px; background: var(--accent); color: var(--on-accent); font-weight: 700; font-size: 16px; }
  .busy { font-weight: 600; color: var(--achieved); }
  .alert { border-color: var(--needs-support); color: var(--needs-support); }
  /* Three plain facts on one line, separated by dots: words, not badges. */
  .points { list-style: none; padding: 0; margin: 28px 0 0; display: flex; flex-wrap: wrap; gap: 4px 0; color: var(--muted); font-size: 15px; font-weight: 500; }
  .points li:not(:last-child)::after { content: "·"; margin: 0 10px; }
  .teachers { margin-top: 24px; }
  .teachers a { color: var(--ink); font-weight: 600; text-underline-offset: 3px; }
  @media (prefers-reduced-motion: no-preference) {
    .door { animation: rise 420ms cubic-bezier(.2,.8,.2,1) backwards; }
    li:nth-child(2) .door { animation-delay: 70ms; }
  }
  /* Slide without fading: text is never caught half transparent, or below its real contrast. */
  @keyframes rise { from { transform: translateY(14px); } }
  @media (max-width: 480px) {
    .door { flex-wrap: wrap; }
    .enter { width: 100%; text-align: center; }
  }
</style>
