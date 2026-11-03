<script lang="ts">
  // The game's front door (#play): the oasis in dots, Bizra, and two doorways, one per course.
  // A tap on a doorway starts a guest learner in that course (ADR 0012); the demo classes for
  // teachers sit behind a small link (#classes). The look is the front page's (ADR 0015).
  import { app } from "../lib/app.svelte.ts";
  import { t, type Key } from "../lib/i18n/i18n.ts";
  import LanguagePicker from "../components/LanguagePicker.svelte";
  import CourseIcon from "../play/CourseIcon.svelte";
  import Mascot from "../play/Mascot.svelte";
  import Scene from "../play/Scene.svelte";
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

<div class="hero">
  <Scene height={300} />
  <div class="top">
    <a class="logo" href="#top" lang="en">zidara</a>
    <LanguagePicker />
  </div>
  <div class="guide">
    <p class="bubble">{t(app.locale, "welcome.hello")}</p>
    <Mascot size={118} mood={busy ? "cheer" : "idle"} />
  </div>
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
          <span class="window"><CourseIcon lesson={course.lesson} size={76} /></span>
          <span class="name">{course.title[app.locale]}</span>
          <span class="blurb">{t(app.locale, `course.${course.key}.blurb` as Key)}</span>
          <span class="meta">{t(app.locale, `course.${course.key}.meta` as Key)}</span>
          <span class="enter" aria-hidden="true">{busy === course.lesson ? "…" : t(app.locale, "welcome.enter")}</span>
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
  .hero { position: relative; overflow: hidden; }
  .top { position: absolute; inset: 0 0 auto; display: flex; justify-content: space-between; align-items: center; padding: 0.5rem 1rem; max-width: 760px; margin: 0 auto; }
  .logo { font-size: 26px; font-weight: 700; color: var(--ink); letter-spacing: -0.02em; text-decoration: none; padding: 0.1rem 0.7rem; border-radius: 10px; background: var(--surface); border: 1px solid var(--line); }
  .guide { position: absolute; bottom: 14px; inset-inline-start: 50%; translate: -50%; display: flex; align-items: flex-end; gap: 0.25rem; width: max-content; }
  :global([dir="rtl"]) .guide { translate: 50%; }
  .bubble {
    position: relative; margin: 0 0 4.5rem; max-width: 12.5rem; padding: 0.6rem 0.9rem; border-radius: 14px; background: #fff;
    border: 1px solid var(--ink); border-bottom-width: 3px; font-weight: 500; font-size: 17px; line-height: 1.35;
  }
  .bubble::after {
    content: ""; position: absolute; inset-inline-end: -11px; bottom: 14px; width: 16px; height: 16px; background: #fff;
    border-inline-end: 1px solid var(--ink); border-bottom: 1px solid var(--ink); transform: rotate(-45deg);
  }
  :global([dir="rtl"]) .bubble::after { transform: rotate(45deg); }
  .welcome { max-width: 760px; padding-top: 0.5rem; }
  h1 { font-size: 44px; text-align: center; margin: 0.25rem 0 0; line-height: 1.1; font-weight: 600; letter-spacing: -0.03em; }
  .lead { font-size: 19px; color: var(--muted); max-width: 34rem; margin: 0.5rem auto 0; text-align: center; }
  h2 { text-align: center; margin-top: 1.5rem; font: 500 13px/1.2 var(--mono); text-transform: uppercase; letter-spacing: 0.06em; color: var(--muted); }
  .doors { list-style: none; margin: 0; padding: 0; display: grid; grid-template-columns: 1fr 1fr; gap: 0.75rem; }
  /* A doorway: a horseshoe arch, as in the old towns' gates, with the course in a dotted window. */
  .door {
    width: 100%; height: 100%; display: grid; justify-items: center; align-content: start; gap: 0.2rem; padding: 0.9rem 0.6rem 1rem;
    border-radius: 999px 999px 18px 18px / 120px 120px 18px 18px; color: var(--ink); text-align: center; background: var(--surface);
    border: 1px solid var(--line); border-bottom-width: 5px;
  }
  .door:hover:not(:disabled) { border-color: var(--accent); }
  .door.teal { --tone: var(--in-progress); }
  .door.terra { --tone: var(--terra); }
  .door:active:not(:disabled) { border-bottom-width: 2px; transform: translateY(3px); }
  .window {
    display: grid; place-items: center; width: 108px; height: 120px; margin-bottom: 0.35rem; background-color: var(--accent-tint);
    background-image: radial-gradient(circle, color-mix(in srgb, var(--tone) 22%, transparent) 1.2px, transparent 1.6px); background-size: 8px 8px;
    border-radius: 999px 999px 12px 12px / 90px 90px 12px 12px; border: 1px dashed color-mix(in srgb, var(--tone) 45%, var(--line));
  }
  .name { font-size: 21px; font-weight: 600; line-height: 1.2; }
  .blurb { font-size: 16px; font-weight: 400; line-height: 1.35; color: var(--muted); }
  .meta { font: 500 13px/1.4 var(--mono); color: var(--tone); }
  .enter { margin-top: 0.6rem; padding: 0.25rem 1.1rem; border-radius: 999px; background: var(--ink); color: #fff; font-weight: 600; font-size: 16px; }
  .door:hover:not(:disabled) .enter { background: var(--accent); }
  .busy { text-align: center; font-weight: 600; color: var(--achieved); }
  .alert { border-color: var(--needs-support); color: var(--needs-support); }
  .points { list-style: none; padding: 0; margin: 1.5rem 0 0; display: flex; flex-wrap: wrap; justify-content: center; gap: 0.5rem 1.25rem; color: var(--muted); font-weight: 500; font-size: 16px; }
  .teachers { text-align: center; margin-top: 1.25rem; }
  .teachers a { color: var(--accent); font-weight: 600; }
  @media (prefers-reduced-motion: no-preference) {
    .door { animation: rise 420ms steps(6) backwards; }
    li:nth-child(2) .door { animation-delay: 90ms; }
  }
  /* Slide without fading: text is never caught half transparent, or below its real contrast. */
  @keyframes rise { from { transform: translateY(24px); } }
  @media (max-width: 380px) { h1 { font-size: 36px; } .name { font-size: 19px; } .bubble { max-width: 10rem; font-size: 16px; } }
</style>
