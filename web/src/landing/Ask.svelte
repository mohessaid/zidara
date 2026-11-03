<script lang="ts">
  // The hero's console (ADR 0016): someone asks Zidara for a course and the engine answers with
  // what it set up, layer by layer. The request types itself, the reply arrives a line at a time,
  // and the next example follows after a pause. Under reduced motion everything is shown at once
  // and nothing advances on its own; the pager always works.
  import { onMount } from "svelte";
  import type { AskCopy } from "../lib/i18n/landing.ts";

  let { copy }: { copy: AskCopy } = $props();

  let at = $state(0);
  let typed = $state(Number.MAX_SAFE_INTEGER);
  let lines = $state(Number.MAX_SAFE_INTEGER);
  let auto = $state(false);
  let host: HTMLElement;
  let timer: ReturnType<typeof setTimeout> | undefined;
  let visible = false;

  const item = $derived(copy.items[at]!);
  const shown = $derived(item.ask.slice(0, typed));
  const typing = $derived(typed < item.ask.length);

  function stop() {
    clearTimeout(timer);
  }
  function play() {
    stop();
    if (!auto || !visible) {
      typed = lines = Number.MAX_SAFE_INTEGER;
      return;
    }
    typed = 0;
    lines = 0;
    const type = () => {
      if (typed < item.ask.length) {
        typed += 1;
        timer = setTimeout(type, 28);
      } else timer = setTimeout(reply, 420);
    };
    const reply = () => {
      if (lines < item.reply.length) {
        lines += 1;
        timer = setTimeout(reply, 380);
      } else timer = setTimeout(() => go(at + 1, false), 4200);
    };
    timer = setTimeout(type, 300);
  }
  function go(i: number, byHand = true) {
    if (byHand) auto = false;
    at = (i + copy.items.length) % copy.items.length;
    play();
  }

  onMount(() => {
    auto = !matchMedia("(prefers-reduced-motion: reduce)").matches;
    const io = new IntersectionObserver(([e]) => {
      const was = visible;
      visible = !!e?.isIntersecting;
      if (visible && !was) play();
      if (!visible) stop();
    });
    io.observe(host);
    return () => {
      stop();
      io.disconnect();
    };
  });
</script>

<figure class="ask" bind:this={host} aria-label={copy.label}>
  <div class="head">
    <span class="dots" aria-hidden="true"><i></i><i></i><i></i></span>
    <span class="name">{copy.label}</span>
    <span class="live">{copy.tag}</span>
  </div>
  <div class="body" aria-live="off">
    <p class="q"><span class="caret" aria-hidden="true">›</span><span>{shown}</span>{#if typing}<span class="cursor" aria-hidden="true"></span>{/if}</p>
    <ul class="a">
      {#each item.reply as line, i (line.k + i)}
        {#if i < lines}<li><span class="k">{line.k}</span><span class="v">{line.v}</span></li>{/if}
      {/each}
    </ul>
  </div>
  <div class="pager">
    <button type="button" aria-label={copy.prev} onclick={() => go(at - 1)}><span class="chev" aria-hidden="true">‹</span></button>
    <span class="count" dir="ltr">{at + 1} / {copy.items.length}</span>
    <button type="button" aria-label={copy.next} onclick={() => go(at + 1)}><span class="chev" aria-hidden="true">›</span></button>
  </div>
</figure>

<style>
  .ask {
    margin: 0; width: min(560px, 100%); border: 1px solid var(--l-raise); border-radius: 16px; overflow: hidden;
    background: var(--l-bg); text-align: start; box-shadow: 0 24px 60px -36px rgb(15 15 16 / 0.28);
  }
  .head { display: flex; align-items: center; gap: 12px; padding: 12px 16px; border-bottom: 1px solid var(--l-line); background: var(--l-surface); }
  .dots { display: inline-flex; gap: 6px; }
  .dots i { width: 9px; height: 9px; border-radius: 50%; background: var(--l-raise); }
  .name { flex: 1; text-align: center; font: 500 12px/1 var(--l-mono); letter-spacing: 0.12em; text-transform: uppercase; color: var(--l-muted); }
  .live { font: 600 12px/1 var(--l-mono); color: var(--l-accent-ink); }
  .body { padding: 18px 20px 8px; min-height: 196px; }
  .q { margin: 0 0 14px; font-size: 16px; color: var(--l-ink); display: flex; gap: 10px; align-items: baseline; }
  .caret { color: var(--l-accent); font-weight: 700; }
  :global([dir="rtl"]) .caret { transform: scaleX(-1); display: inline-block; }
  .cursor { display: inline-block; width: 8px; height: 1.05em; margin-inline-start: 2px; background: var(--l-accent); vertical-align: text-bottom; animation: blink 900ms steps(1) infinite; }
  .a { list-style: none; margin: 0; padding: 0; display: grid; gap: 8px; }
  .a li { display: grid; grid-template-columns: 7.5rem 1fr; gap: 12px; font-size: 14.5px; line-height: 1.45; animation: line-in 320ms cubic-bezier(.16,1,.3,1) both; }
  .k { font: 600 12px/1.9 var(--l-mono); text-transform: uppercase; letter-spacing: 0.06em; color: var(--l-accent-ink); }
  .v { color: var(--l-ink); }
  .pager { display: flex; align-items: center; justify-content: space-between; padding: 6px 8px; border-top: 1px solid var(--l-line); }
  .pager button { min-height: 40px; min-width: 40px; border: 0; background: none; color: var(--l-muted); border-radius: 10px; cursor: pointer; font-size: 20px; }
  .pager button:hover { color: var(--l-ink); background: var(--l-surface); }
  :global([dir="rtl"]) .chev { display: inline-block; transform: scaleX(-1); }
  .count { font: 500 12px/1 var(--l-mono); letter-spacing: 0.1em; color: var(--l-muted); }
  @keyframes blink { 50% { opacity: 0; } }
  @keyframes line-in { from { transform: translateY(4px); } }
  @media (max-width: 520px) {
    .a li { grid-template-columns: 1fr; gap: 0; }
    .body { min-height: 260px; }
  }
</style>
