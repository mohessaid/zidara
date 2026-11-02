<script lang="ts">
  // The front door: the oasis, Bizra, and two doorways, one per course. A tap on a doorway
  // starts a guest learner in that course (ADR 0012); the demo classes for teachers sit behind
  // a small link (#classes). The look is the oasis of the art bible (ADR 0013).
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
    <span class="logo" lang="en">zidara</span>
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
  .logo { font-size: 34px; font-weight: 800; color: #fff; letter-spacing: -0.01em; text-shadow: 0 2px 0 var(--blue-edge), 0 0 1px var(--blue-edge); }
  .guide { position: absolute; bottom: 14px; inset-inline-start: 50%; translate: -50%; display: flex; align-items: flex-end; gap: 0.25rem; width: max-content; }
  :global([dir="rtl"]) .guide { translate: 50%; }
  .bubble {
    position: relative; margin: 0 0 4.5rem; max-width: 12.5rem; padding: 0.6rem 0.9rem; border-radius: 18px; background: #fff;
    border: 2px solid var(--ink); border-bottom-width: 4px; font-weight: 700; font-size: 17px; line-height: 1.35;
  }
  .bubble::after {
    content: ""; position: absolute; inset-inline-end: -11px; bottom: 14px; width: 16px; height: 16px; background: #fff;
    border-inline-end: 2px solid var(--ink); border-bottom: 2px solid var(--ink); transform: rotate(-45deg);
  }
  :global([dir="rtl"]) .bubble::after { transform: rotate(45deg); }
  .welcome { max-width: 760px; padding-top: 0.5rem; }
  h1 { font-size: 44px; text-align: center; margin: 0.25rem 0 0; line-height: 1.1; }
  .lead { font-size: 19px; color: var(--muted); max-width: 34rem; margin: 0.5rem auto 0; text-align: center; }
  h2 { text-align: center; margin-top: 1.5rem; font-size: 22px; }
  .doors { list-style: none; margin: 0; padding: 0; display: grid; grid-template-columns: 1fr 1fr; gap: 0.75rem; }
  /* A doorway: a horseshoe arch, as in the old towns' gates, with the course in its window. */
  .door {
    width: 100%; height: 100%; display: grid; justify-items: center; align-content: start; gap: 0.2rem; padding: 0.9rem 0.6rem 1rem;
    border-radius: 999px 999px 20px 20px / 120px 120px 20px 20px; color: #fff; text-align: center; border-width: 0 0 7px;
  }
  .door.teal { background: var(--in-progress); border-color: var(--blue-edge); }
  .door.terra { background: var(--terra); border-color: var(--terra-edge); }
  .door:active:not(:disabled) { border-bottom-width: 3px; transform: translateY(4px); }
  .window {
    display: grid; place-items: center; width: 108px; height: 120px; margin-bottom: 0.35rem; background: #fff4dc;
    border-radius: 999px 999px 12px 12px / 90px 90px 12px 12px; box-shadow: inset 0 0 0 5px rgba(255, 255, 255, 0.55), 0 0 0 4px rgba(0, 0, 0, 0.12);
  }
  .name { font-size: 21px; font-weight: 800; line-height: 1.2; }
  .blurb { font-size: 16px; font-weight: 500; line-height: 1.35; }
  .meta { font-size: 15px; font-weight: 700; opacity: 0.95; }
  .enter { margin-top: 0.5rem; padding: 0.15rem 1rem; border-radius: 999px; background: #fff; font-weight: 800; font-size: 17px; }
  .teal .enter { color: var(--teal-text); }
  .terra .enter { color: var(--terra-edge); }
  .busy { text-align: center; font-weight: 700; color: var(--achieved); }
  .alert { border-color: var(--needs-support); color: var(--needs-support); }
  .points { list-style: none; padding: 0; margin: 1.5rem 0 0; display: flex; flex-wrap: wrap; justify-content: center; gap: 0.5rem 1.25rem; color: var(--muted); font-weight: 600; }
  .teachers { text-align: center; margin-top: 1.25rem; }
  .teachers a { color: var(--zellige); font-weight: 700; }
  @media (prefers-reduced-motion: no-preference) {
    .door { animation: rise 420ms cubic-bezier(.2,1.3,.4,1) backwards; }
    li:nth-child(2) .door { animation-delay: 90ms; }
  }
  @keyframes rise { from { transform: translateY(24px); opacity: 0; } }
  @media (max-width: 380px) { h1 { font-size: 36px; } .name { font-size: 19px; } .bubble { max-width: 10rem; font-size: 16px; } }
</style>
