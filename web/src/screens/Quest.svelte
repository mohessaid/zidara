<script lang="ts">
  import { onMount } from "svelte";
  import { Application, Graphics, Container } from "pixi.js";
  import gsap from "gsap";
  import { app } from "../lib/app.svelte.ts";
  import { direction, formatDate, t } from "../lib/i18n/i18n.ts";
  import type { LearnerView } from "../lib/kernel/types/LearnerView.ts";
  import type { OutcomeView } from "../lib/kernel/types/OutcomeView.ts";
  import { stonePositions } from "../lib/path.ts";
  import { stars } from "../lib/stars.ts";

  let { view, onopen }: { view: LearnerView; onopen: (outcome: string) => void } = $props();

  const COLOURS = { achieved: 0x1e7b4f, in_progress: 0x1f5fad, needs_support: 0xb23b2e, not_started: 0x1f5fad, locked: 0x5b6475 } as const;
  const reduced = matchMedia("(prefers-reduced-motion: reduce)").matches;

  let host: HTMLDivElement;
  let pixi: Application | undefined;
  let layer: Container | undefined;
  let previous = new Map<string, string>();

  const positions = $derived(stonePositions(view.outcomes.length, direction(app.locale) === "rtl"));
  const stateOf = (o: OutcomeView) => (o.lock ? "locked" : o.judgement.state);

  function lockText(o: OutcomeView): string {
    const lock = o.lock;
    if (!lock) return "";
    if (lock.type === "prerequisite") {
      const needed = view.outcomes.find((x) => x.outcome === lock.outcome);
      return t(app.locale, "lock.prerequisite", { name: needed?.title ?? lock.outcome });
    }
    return t(app.locale, "lock.opens_at", { date: formatDate(app.locale, lock.at) });
  }

  function draw() {
    if (!pixi || !layer) return;
    const { width, height } = pixi.screen;
    layer.removeChildren().forEach((c) => c.destroy());
    const points = positions.map((p) => ({ x: p.x * width, y: p.y * height }));
    const radius = Math.min(width, height) * 0.11;
    // The path: solid where the next stone is open, dashed where it is still ahead.
    for (let i = 1; i < points.length; i++) {
      const [a, b] = [points[i - 1]!, points[i]!];
      const open = !view.outcomes[i]!.lock;
      const path = new Graphics();
      const steps = open ? 1 : 12;
      for (let s = 0; s < steps; s += open ? 1 : 2) {
        const [f0, f1] = [s / steps, Math.min((s + 1) / steps, 1)];
        path.moveTo(a.x + (b.x - a.x) * f0, a.y + (b.y - a.y) * f0).lineTo(a.x + (b.x - a.x) * f1, a.y + (b.y - a.y) * f1);
      }
      path.stroke({ width: 8, color: open ? 0xc9a96e : 0xd8ccb5, cap: "round" });
      layer.addChild(path);
    }
    view.outcomes.forEach((o, i) => {
      const p = points[i]!;
      const state = stateOf(o);
      const colour = COLOURS[state];
      const stone = new Container({ x: p.x, y: p.y });
      const disc = new Graphics().circle(0, 0, radius);
      if (state === "achieved") disc.fill(colour);
      else disc.fill(0xffffff).stroke({ width: 6, color: colour });
      if (view.next === o.outcome) disc.circle(0, 0, radius + 12).stroke({ width: 4, color: 0x1f5fad, alpha: 0.5 });
      stone.addChild(disc);
      const earned = stars(o.judgement);
      for (let s = 0; s < earned.of; s++) {
        const star = new Graphics().star(0, 0, 5, radius * 0.18, radius * 0.08);
        star.position.set((s - (earned.of - 1) / 2) * radius * 0.42, radius + radius * 0.3);
        if (s < earned.n) star.fill(0xb07a00);
        else star.stroke({ width: 2, color: 0xb07a00 });
        stone.addChild(star);
      }
      layer!.addChild(stone);
      if (state === "achieved" && previous.get(o.outcome) !== "achieved" && previous.size > 0 && !reduced) {
        gsap.from(stone.scale, { x: 0.6, y: 0.6, duration: 0.4, ease: "back.out(2)" });
      }
    });
    previous = new Map(view.outcomes.map((o) => [o.outcome, stateOf(o)]));
  }

  onMount(() => {
    const instance = new Application();
    let alive = true;
    void instance.init({ resizeTo: host, backgroundAlpha: 0, antialias: true, autoDensity: true, resolution: devicePixelRatio }).then(() => {
      if (!alive) return instance.destroy(true, { children: true });
      instance.canvas.setAttribute("aria-hidden", "true");
      host.prepend(instance.canvas);
      layer = new Container();
      instance.stage.addChild(layer);
      instance.stage.interactiveChildren = false;
      pixi = instance;
      instance.renderer.on("resize", draw);
      draw();
    });
    return () => {
      alive = false;
      pixi?.destroy(true, { children: true });
      pixi = undefined;
    };
  });

  $effect(() => {
    void view;
    void app.locale;
    draw();
  });

</script>

<section aria-label={t(app.locale, "quest.map")}>
  <div class="map" bind:this={host}>
    {#each view.outcomes as o, i (o.outcome)}
      {@const p = positions[i]!}
      {@const state = stateOf(o)}
      {@const earned = stars(o.judgement)}
      <button
        class="twin state-{state}"
        style="left: {p.x * 100}%; top: {p.y * 100}%"
        data-outcome={o.outcome}
        data-state={state}
        disabled={o.lock !== null}
        aria-current={view.next === o.outcome ? "step" : undefined}
        onclick={() => onopen(o.outcome)}
      >
        <span class="visually-hidden">{t(app.locale, "outcome.open", { name: o.title })}.</span>
        <span class="label">
          <strong>{o.title}</strong>
          <span>{state === "locked" ? `🔒 ${lockText(o)}` : t(app.locale, `state.${o.judgement.state}`)}</span>
          {#if earned.of > 0}<span class="visually-hidden">{t(app.locale, "stars", earned)}</span>{/if}
        </span>
      </button>
    {/each}
  </div>
  {#if view.next}
    <p class="next">{t(app.locale, "quest.next")}: <strong>{view.outcomes.find((o) => o.outcome === view.next)?.title}</strong></p>
  {/if}
</section>

<style>
  .map { position: relative; width: 100%; aspect-ratio: 16 / 10; background: radial-gradient(circle at 30% 30%, #fff3d6, var(--paper)); border-radius: var(--radius); overflow: hidden; }
  .map :global(canvas) { position: absolute; inset: 0; }
  .twin { position: absolute; transform: translate(-50%, -50%); width: 22%; aspect-ratio: 1; border-radius: 50%; background: transparent; border: none; padding: 0; }
  .twin:disabled { cursor: not-allowed; opacity: 1; }
  .label { position: absolute; top: 100%; inset-inline-start: 50%; transform: translateX(-50%); width: max-content; max-width: 30vw; display: flex; flex-direction: column; font-size: 16px; padding-top: 2.2rem; text-align: center; }
  :global([dir="rtl"]) .label { transform: translateX(50%); }
  .label strong { color: var(--ink); }
  .next { font-size: 24px; }
</style>
