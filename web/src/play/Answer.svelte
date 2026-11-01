<script lang="ts">
  import { lookOf } from "./look.ts";
  // How a learner gives one answer, as the question's look asks: choice tiles, a fraction typed
  // as two numbers, a bar to shade, or a bar to cut and then shade. It only collects the answer;
  // the kernel marks it.
  import { app } from "../lib/app.svelte.ts";
  import { t } from "../lib/i18n/i18n.ts";
  import type { Question } from "../lib/kernel/types/Question.ts";
  import type { Response } from "../lib/kernel/types/Response.ts";
  import { sound } from "./sound.svelte.ts";

  let {
    question,
    response = $bindable(),
    locked = false,
    live = false,
    onchoose,
  }: { question: Question; response?: Response; locked?: boolean; live?: boolean; onchoose?: () => void } = $props();

  const text = (l: Record<string, string>) => l[app.locale] ?? l.en ?? Object.values(l)[0] ?? "";
  const SHAPES = ["●", "▲", "■", "◆"];

  // svelte-ignore state_referenced_locally
  const input = lookOf(question).input;
  let num: number | undefined = $state();
  let den: number | undefined = $state();
  // svelte-ignore state_referenced_locally
  let parts = $state(input.type === "shade" ? input.parts : 1);
  let shaded: boolean[] = $state(Array.from({ length: input.type === "shade" ? input.parts : 1 }, () => false));
  let touched = $state(false);

  function choose(index: number) {
    if (locked) return;
    response = { type: "choice", index };
    sound.play("select");
    onchoose?.();
  }

  function typed() {
    response = Number.isInteger(num) && Number.isInteger(den) ? { type: "fraction", num: num!, den: den! } : undefined;
  }

  function toggle(i: number) {
    if (locked) return;
    shaded[i] = !shaded[i];
    touched = true;
    sound.play("tap");
    response = { type: "fraction", num: shaded.filter(Boolean).length, den: parts };
  }

  function cut(by: number) {
    if (locked || input.type !== "cut") return;
    parts = Math.min(input.max, Math.max(1, parts + by));
    shaded = Array.from({ length: parts }, () => false);
    sound.play("tap");
    response = touched ? { type: "fraction", num: 0, den: parts } : undefined;
  }
</script>

{#if question.expected.type === "choice"}
  <div class="tiles" class:live class:long={question.expected.options.some((o) => text(o).length > 14)} role="radiogroup" aria-label={text(question.prompt)}>
    {#each question.expected.options as option, k (k)}
      <button
        role="radio"
        aria-checked={response?.type === "choice" && response.index === k}
        class="tile c{k % 4}"
        disabled={locked}
        onclick={() => choose(k)}
      >
        {#if live}<span class="shape" aria-hidden="true">{SHAPES[k % 4]}</span>{/if}
        <bdi>{text(option)}</bdi>
      </button>
    {/each}
  </div>
{:else if input.type === "keypad"}
  <div class="fraction" dir="ltr">
    <label><span class="visually-hidden">{t(app.locale, "quiz.numerator")}</span>
      <input type="number" inputmode="numeric" min="0" disabled={locked} bind:value={num} oninput={typed} /></label>
    <span class="line" aria-hidden="true"></span>
    <label><span class="visually-hidden">{t(app.locale, "quiz.denominator")}</span>
      <input type="number" inputmode="numeric" min="0" disabled={locked} bind:value={den} oninput={typed} /></label>
  </div>
{:else}
  <div class="shade">
    {#if input.type === "cut"}
      <div class="stepper" role="group" aria-label={t(app.locale, "answer.cut")}>
        <button disabled={locked || parts <= 1} onclick={() => cut(-1)} aria-label={t(app.locale, "answer.fewer")}>−</button>
        <output aria-live="polite">{t(app.locale, "answer.parts", { n: parts })}</output>
        <button disabled={locked || parts >= input.max} onclick={() => cut(1)} aria-label={t(app.locale, "answer.more")}>+</button>
      </div>
    {/if}
    <div class="cells" dir="ltr" role="group" aria-label={t(app.locale, "answer.shade")}>
      {#each shaded as on, i (i)}
        <button class="cell" class:on aria-pressed={on} disabled={locked} onclick={() => toggle(i)}
          aria-label={t(app.locale, "answer.part", { i: i + 1, n: parts })}></button>
      {/each}
    </div>
    <p class="count" aria-live="polite">
      <span dir="ltr">{shaded.filter(Boolean).length}/{parts}</span>
    </p>
  </div>
{/if}

<style>
  .tiles { display: grid; gap: 0.75rem; grid-template-columns: repeat(auto-fit, minmax(140px, 1fr)); }
  .tile { min-height: 76px; font-size: 26px; font-weight: 800; border-width: 2px 2px 5px; border-radius: 18px; }
  .tile[aria-checked="true"] { border-color: #84d8ff; border-bottom-color: #4fb8ef; background: var(--sky); color: var(--blue-edge); }
  /* Sentences rather than numbers: one per row, at reading size. */
  .long { grid-template-columns: 1fr; }
  .long .tile { font-size: 20px; min-height: 60px; text-align: start; justify-content: flex-start; padding: 0.6rem 1rem; }
  .live .tile { color: #fff; border-color: transparent; display: flex; gap: 0.75rem; align-items: center; justify-content: center; }
  .live .c0 { background: var(--in-progress); } .live .c1 { background: var(--achieved); }
  .live .c2 { background: var(--needs-support); } .live .c3 { background: var(--agent); }
  .live .tile[aria-checked="true"] { outline: 4px solid var(--ink); outline-offset: 3px; }
  .shape { font-size: 30px; }
  .fraction { display: flex; width: fit-content; flex-direction: column; align-items: center; gap: 0.4rem; margin: 0 auto; }
  .fraction input { width: 6rem; min-height: 64px; font-size: 34px; font-weight: 800; text-align: center; border: 2px solid var(--line); border-bottom-width: 4px; border-radius: var(--radius); background: #f7f7f7; }
  .fraction input:focus { border-color: #84d8ff; background: #fff; }
  .line { width: 7rem; height: 4px; background: var(--ink); border-radius: 2px; }
  .shade { display: grid; gap: 0.75rem; justify-items: center; }
  .stepper { display: flex; align-items: center; gap: 1rem; }
  .stepper button { font-size: 28px; font-weight: 700; width: 56px; }
  .stepper output { font-size: 22px; font-weight: 700; min-width: 7rem; text-align: center; }
  .cells { display: flex; width: min(100%, 420px); gap: 4px; }
  .cell { flex: 1; min-width: 0; min-height: 72px; padding: 0; border-radius: 10px; transition: background 120ms ease; }
  .cell.on { background: #ff9600; border-color: #c76a00; }
  .cell:disabled, .tile:disabled { opacity: 1; }
  .count { font-size: 28px; font-weight: 700; margin: 0; }
</style>
