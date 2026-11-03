<script lang="ts">
  // zidara.ta9in.com's front page (ADR 0014): what Zidara is, why it exists (five whys, each
  // figure sourced), how it works, the proof, and the way into the game at #play. Its own face
  // (Readex Pro) and its own calm tokens; the game keeps the oasis. The live switch in the hero
  // is the real kernel. Built from context.dev's structure, not its look.
  import "@fontsource-variable/readex-pro";
  import { app } from "../lib/app.svelte.ts";
  import { LANDING } from "../lib/i18n/landing.ts";
  import LanguagePicker from "../components/LanguagePicker.svelte";
  import Scene from "../play/Scene.svelte";
  import LiveSwitch from "./LiveSwitch.svelte";
  import Mark from "./Mark.svelte";
  import { countUp, reveal } from "./reveal.ts";

  const copy = $derived(LANDING[app.locale]);
  const REPO = "https://github.com/mohessaid/zidara";
</script>

<div class="landing">
  <header class="nav">
    <div class="wrap bar">
      <a class="brand" href="#top" aria-label="Zidara"><Mark size={26} /><span lang="en">zidara</span></a>
      <nav aria-label={copy.nav.menu}>
        <a href="#why">{copy.nav.why}</a>
        <a href="#how">{copy.nav.how}</a>
        <a href="#proof">{copy.nav.proof}</a>
      </nav>
      <div class="end">
        <LanguagePicker />
        <a class="cta small" href="#play" data-play>{copy.nav.play}</a>
      </div>
    </div>
  </header>

  <main id="top">
    <section class="wrap hero">
      <div class="pitch">
        <p class="made"><Mark size={16} />{copy.footer.made}</p>
        <h1>{copy.hero.before}<em>{copy.hero.accent}</em>{copy.hero.after}</h1>
        <p class="lead">{copy.hero.lead}</p>
        <div class="ctas">
          <a class="cta" href="#play" data-play>{copy.nav.play}<span class="arrow" aria-hidden="true">→</span></a>
          <a class="ghost" href="#how">{copy.hero.more}</a>
        </div>
      </div>
      <div class="demo"><LiveSwitch /></div>
    </section>

    <section class="facts">
      <ul class="wrap">
        {#each copy.facts as fact (fact)}<li>{fact}</li>{/each}
      </ul>
    </section>

    <section class="wrap why" id="why" aria-labelledby="why-title">
      <h2 id="why-title" use:reveal>{copy.why.title}</h2>
      <p class="sub" use:reveal>{copy.why.lead}</p>
      <ol class="ladder">
        {#each copy.why.steps as step, i (i)}
          <li use:reveal style:--i={i}>
            <span class="rung" aria-hidden="true">{i + 1}</span>
            <div>
              <h3>{step.q}</h3>
              <p>
                {step.a}
                {#each step.cite as n (n)}<a class="cite" href={`#src-${n}`} aria-label={`${copy.why.sources} ${n}`}>[{n}]</a>{/each}
              </p>
            </div>
          </li>
        {/each}
      </ol>
      <div class="root" use:reveal>
        <Mark size={40} tone="light" />
        <div>
          <h3>{copy.why.rootTitle}</h3>
          <p>{copy.why.root}</p>
        </div>
      </div>
      <div class="sources">
        <h3>{copy.why.sources}</h3>
        <ol>
          {#each copy.sources as source, i (source.href)}
            <li id={`src-${i + 1}`}><a href={source.href} rel="noopener" target="_blank">{source.label}</a></li>
          {/each}
        </ol>
      </div>
    </section>

    <section class="how" id="how" aria-labelledby="how-title">
      <div class="wrap">
        <h2 id="how-title" use:reveal>{copy.how.title}</h2>
        <p class="sub" use:reveal>{copy.how.lead}</p>
        <div class="bento">
          <article class="tile blocks" use:reveal>
            <h3>{copy.how.blocksTitle}</h3>
            <ul>
              {#each copy.how.blocks as block, i (block.title)}
                <li style:--i={i}><strong>{block.title}</strong><span>{block.body}</span></li>
              {/each}
            </ul>
          </article>
          <article class="tile layers" use:reveal>
            <h3>{copy.how.layersTitle}</h3>
            <ol>
              {#each copy.how.layers as layer, i (layer.title)}
                <li class={`l${i}`}><strong>{layer.title}</strong><span>{layer.body}</span></li>
              {/each}
            </ol>
          </article>
          <article class="tile agents" use:reveal>
            <h3>{copy.how.tiles[1]!.title}</h3>
            <p>{copy.how.tiles[1]!.body}</p>
            <ol class="flow">
              {#each copy.how.flow as step, i (step)}<li class:person={i === copy.how.flow.length - 1}>{step}</li>{/each}
            </ol>
          </article>
          <article class="tile formula" use:reveal>
            <p class="big" lang="en" dir="ltr">N + M</p>
            <h3>{copy.how.tiles[0]!.title}</h3>
            <p>{copy.how.tiles[0]!.body}</p>
          </article>
          <article class="tile phones" use:reveal>
            <div class="art"><Scene height={170} palms={3} /></div>
            <h3>{copy.how.tiles[2]!.title}</h3>
            <p>{copy.how.tiles[2]!.body}</p>
          </article>
          <article class="tile walls" use:reveal>
            <h3>{copy.how.tiles[3]!.title}</h3>
            <p>{copy.how.tiles[3]!.body}</p>
          </article>
        </div>
      </div>
    </section>

    <section class="wrap made-with" id="made" aria-labelledby="made-title">
      <h2 id="made-title" use:reveal><Mark size={34} />{copy.made.title}</h2>
      <p class="sub" use:reveal>{copy.made.lead}</p>
      <!-- On phones the row scrolls sideways, so a keyboard must reach and scroll it too (axe:
           scrollable-region-focusable). A focusable labelled region is the WCAG pattern for this. -->
      <!-- svelte-ignore a11y_no_noninteractive_tabindex -->
      <div class="shots" role="region" aria-label={copy.made.title} tabindex="0">
      <ul class="phones-row">
        {#each copy.made.shots as shot, i (shot.src)}
          <li use:reveal style:--i={i}>
            <figure>
              <div class="device"><img src={shot.src} alt={shot.alt} width="540" height="1099" loading="lazy" decoding="async" /></div>
              <figcaption>{shot.caption}</figcaption>
            </figure>
          </li>
        {/each}
      </ul>
      </div>
    </section>

    <section class="proof" id="proof" aria-labelledby="proof-title">
      <div class="wrap">
        <h2 id="proof-title" use:reveal>{copy.proof.title}</h2>
        <p class="sub" use:reveal>{copy.proof.lead} <code lang="en" dir="ltr">make gauntlet</code></p>
        {#key app.locale}<dl class="stats">
          {#each copy.proof.stats as stat (stat.label)}
            <div use:reveal>
              <dt use:countUp>{stat.value}</dt>
              <dd>{stat.label}</dd>
            </div>
          {/each}
        </dl>{/key}
        <a class="ghost" href={REPO} rel="noopener" target="_blank">{copy.proof.code}<span class="arrow" aria-hidden="true">→</span></a>
      </div>
    </section>

    <section class="wrap faq" aria-labelledby="faq-title">
      <h2 id="faq-title">{copy.faq.title}</h2>
      <div class="qa">
        {#each copy.faq.items as item (item.q)}
          <details>
            <summary>{item.q}</summary>
            <p>{item.a}</p>
          </details>
        {/each}
      </div>
    </section>

    <section class="wrap">
      <div class="end-cta" use:reveal>
        <div>
          <h2>{copy.end.title}</h2>
          <p>{copy.end.lead}</p>
        </div>
        <div class="end-actions">
          <a class="cta" href="#play" data-play>{copy.nav.play}<span class="arrow" aria-hidden="true">→</span></a>
          <a class="quiet" href="#classes">{copy.end.classes}</a>
        </div>
      </div>
    </section>
  </main>

  <footer class="foot">
    <div class="wrap row-foot">
      <p><Mark size={18} /> <strong lang="en">zidara</strong> {copy.footer.line}</p>
      <p><a href={REPO} rel="noopener" target="_blank">{copy.footer.code}</a> <span class="made-tag">{copy.footer.made}</span></p>
    </div>
    <p class="word" aria-hidden="true" lang="en">zidara</p>
  </footer>
</div>

<style>
  .landing {
    --l-bg: #fbfbfd;
    --l-surface: #ffffff;
    --l-ink: #1f1733;
    --l-muted: #5d5670;
    --l-line: #e6e3ee;
    --l-tint: #eef2fa;
    --l-accent: #1b4f9c;          /* zellige blue: the page's one accent */
    --l-accent-ink: #1b4f9c;
    --l-on-accent: #ffffff;
    --l-frame: #1b4f9c;
    --l-frame-dot: #4f7fcb;
    --l-on-frame: #ffffff;
    --l-locked: #c9c4d6;
    --l-shadow: rgb(27 79 156 / 0.45);
    --l-dot: rgb(27 79 156 / 0.16);
    font-family: "Readex Pro Variable", "Readex Pro", system-ui, sans-serif;
    font-size: 17px;
    line-height: 1.6;
    color: var(--l-ink);
    background: var(--l-bg);
    min-height: 100dvh;
  }
  @media (prefers-color-scheme: dark) {
    .landing {
      --l-bg: #0f0d18;
      --l-surface: #17141f;
      --l-ink: #eeeaf7;
      --l-muted: #aaa3bd;
      --l-line: #2b2638;
      --l-tint: #1f1b2b;
      --l-accent: #8fb4ff;
      --l-accent-ink: #9dbdff;
      --l-on-accent: #0f0d18;
      --l-frame: #16315f;
      --l-frame-dot: #2c56a0;
      --l-locked: #3b3549;
      --l-shadow: rgb(0 0 0 / 0.6);
      --l-dot: rgb(143 180 255 / 0.14);
    }
  }
  main { max-width: none; margin: 0; padding: 0; }
  .wrap { max-width: 1240px; margin: 0 auto; padding: 0 24px; }
  a { color: inherit; }
  :global(.landing :focus-visible) { outline: 3px solid var(--l-accent); outline-offset: 3px; border-radius: 6px; }

  /* Navigation: one line, 64 px, sticky over a blurred page. */
  .nav { position: sticky; top: 0; z-index: 20; background: color-mix(in srgb, var(--l-bg) 86%, transparent); backdrop-filter: blur(10px); border-bottom: 1px solid var(--l-line); }
  .bar { display: flex; align-items: center; gap: 24px; min-height: 64px; }
  .brand { display: inline-flex; align-items: center; gap: 8px; font-weight: 700; font-size: 21px; letter-spacing: -0.02em; text-decoration: none; }
  nav { display: flex; gap: 22px; }
  nav a { text-decoration: none; color: var(--l-muted); font-size: 15px; }
  nav a:hover { color: var(--l-ink); }
  .end { margin-inline-start: auto; display: flex; align-items: center; gap: 10px; }
  .end :global(select) { min-height: 40px; border: 1px solid var(--l-line); border-bottom-width: 1px; border-radius: 999px; background: var(--l-surface); color: var(--l-ink); font-weight: 500; font-size: 14px; }

  /* Buttons: pills everywhere; cards are 20 px; nothing else is round. */
  .cta {
    display: inline-flex; align-items: center; gap: 10px; min-height: 52px; padding: 0 24px; border-radius: 999px;
    background: var(--l-accent); color: var(--l-on-accent); font-weight: 600; text-decoration: none; white-space: nowrap;
    box-shadow: 0 10px 24px -14px var(--l-shadow); transition: transform 160ms ease, box-shadow 160ms ease;
  }
  .cta:hover { transform: translateY(-1px); box-shadow: 0 14px 28px -14px var(--l-shadow); }
  .cta:active { transform: translateY(1px) scale(0.98); }
  .cta.small { min-height: 40px; padding: 0 16px; font-size: 15px; }
  .ghost { display: inline-flex; align-items: center; gap: 10px; min-height: 52px; padding: 0 22px; border-radius: 999px; border: 1px solid var(--l-line); background: var(--l-surface); text-decoration: none; font-weight: 500; white-space: nowrap; }
  .ghost:hover { border-color: var(--l-accent); }
  .arrow { display: inline-block; transition: transform 160ms ease; }
  .cta:hover .arrow, .ghost:hover .arrow { transform: translateX(3px); }
  :global([dir="rtl"]) .arrow { transform: scaleX(-1); }
  :global([dir="rtl"]) .cta:hover .arrow, :global([dir="rtl"]) .ghost:hover .arrow { transform: scaleX(-1) translateX(3px); }

  h1, h2, h3 { letter-spacing: -0.02em; }
  h2 { font-size: clamp(30px, 4vw, 46px); line-height: 1.12; margin: 0 0 12px; font-weight: 600; max-width: 22ch; }
  .sub { color: var(--l-muted); max-width: 60ch; margin: 0 0 40px; font-size: 18px; }
  section { scroll-margin-top: 80px; }
  /* Below the fold, the browser skips layout and paint until a section nears the screen: the
     page's first task on a slowed phone fell from 186 ms (ADR 0014, journal). */
  .why, .how, .made-with, .proof, .faq, .end-cta { content-visibility: auto; contain-intrinsic-size: auto 900px; }

  /* Hero: words on one side, the live engine on the other; a dot field behind, like a grid of tiles. */
  .hero { display: grid; grid-template-columns: 1.2fr 1fr; gap: 64px; align-items: center; padding-top: 72px; padding-bottom: 72px; position: relative; }
  .hero::before {
    content: ""; position: absolute; inset: 0 -40px; z-index: 0; pointer-events: none;
    background-image: radial-gradient(var(--l-dot) 1.4px, transparent 1.7px); background-size: 14px 14px;
    mask-image: radial-gradient(ellipse 60% 70% at 85% 40%, #000 20%, transparent 70%);
  }
  .pitch, .demo { position: relative; z-index: 1; }
  .made { display: inline-flex; align-items: center; gap: 8px; margin: 0 0 20px; padding: 6px 12px; border: 1px solid var(--l-line); border-radius: 999px; font-size: 14px; color: var(--l-muted); background: var(--l-surface); }
  h1 { font-size: clamp(38px, 4.6vw, 62px); line-height: 1.06; margin: 0 0 20px; font-weight: 600; letter-spacing: -0.035em; }
  h1 em { font-style: normal; color: var(--l-accent-ink); }
  .lead { font-size: 20px; color: var(--l-muted); max-width: 40ch; margin: 0 0 32px; }
  .ctas { display: flex; flex-wrap: wrap; gap: 12px; }

  .facts { border-block: 1px solid var(--l-line); background: var(--l-surface); }
  .facts ul { list-style: none; margin: 0 auto; display: grid; grid-template-columns: repeat(5, 1fr); }
  .facts li { padding: 20px 16px; font-size: 15px; font-weight: 500; color: var(--l-muted); border-inline-start: 1px solid var(--l-line); }
  .facts li:last-child { border-inline-end: 1px solid var(--l-line); }

  /* The five whys: a ladder that steps in towards the root. */
  .why { padding-block: 112px 96px; }
  .ladder { list-style: none; margin: 0; padding: 0; display: grid; gap: 4px; position: relative; }
  .ladder li { display: grid; grid-template-columns: 48px 1fr; gap: 20px; padding: 18px 0; margin-inline-start: calc(var(--i) * 3.2%); position: relative; }
  .rung { width: 48px; height: 48px; display: grid; place-items: center; border-radius: 50%; background: var(--l-surface); border: 1px solid var(--l-line); font-weight: 600; color: var(--l-accent-ink); position: relative; z-index: 1; }
  .ladder h3 { margin: 4px 0 6px; font-size: 21px; font-weight: 600; }
  .ladder p { margin: 0; color: var(--l-muted); max-width: 70ch; }
  .cite { font-size: 13px; color: var(--l-accent-ink); text-decoration: none; margin-inline-start: 4px; vertical-align: super; }
  .root {
    margin-top: 28px; margin-inline-start: 16%; display: flex; gap: 20px; align-items: flex-start; padding: 28px 32px; border-radius: 20px;
    background: var(--l-frame); color: #fff; position: relative; overflow: hidden;
  }
  .root::after { content: ""; position: absolute; inset: 0; background-image: radial-gradient(rgb(255 255 255 / 0.13) 1.3px, transparent 1.6px); background-size: 12px 12px; mask-image: linear-gradient(90deg, transparent 30%, #000); pointer-events: none; }
  .root h3 { margin: 0 0 8px; font-size: 24px; font-weight: 600; }
  .root p { margin: 0; color: rgb(255 255 255 / 0.85); max-width: 60ch; }
  .sources { margin-top: 32px; font-size: 14px; color: var(--l-muted); }
  .sources h3 { font-size: 15px; font-weight: 600; margin: 0 0 8px; }
  .sources ol { padding-inline-start: 20px; margin: 4px 0 0; }
  .sources li { margin-bottom: 6px; }

  /* How it works: a bento with rhythm, each tile its own material. */
  .how { padding-block: 104px; background: var(--l-surface); border-block: 1px solid var(--l-line); }
  .bento { display: grid; grid-template-columns: repeat(6, 1fr); gap: 16px; }
  .tile { border: 1px solid var(--l-line); border-radius: 20px; padding: 28px; background: var(--l-bg); overflow: hidden; position: relative; }
  .tile h3 { margin: 0 0 8px; font-size: 20px; font-weight: 600; }
  .tile p { margin: 0; color: var(--l-muted); }
  .blocks { grid-column: span 4; }
  .blocks ul { list-style: none; padding: 0; margin: 18px 0 0; display: grid; grid-template-columns: repeat(3, 1fr); gap: 10px; }
  .blocks li { display: grid; gap: 2px; padding: 14px; border-radius: 12px; background: var(--l-surface); border: 1px solid var(--l-line); }
  .blocks li strong { font-weight: 600; color: var(--l-accent-ink); }
  .blocks li span { font-size: 14px; color: var(--l-muted); line-height: 1.4; }
  .layers { grid-column: span 2; background: var(--l-tint); }
  .layers ol { list-style: none; padding: 0; margin: 18px 0 0; display: grid; gap: 8px; }
  .layers li { padding: 12px 14px; border-radius: 12px; display: grid; gap: 2px; }
  .layers li span { font-size: 14px; line-height: 1.4; }
  .l0 { background: var(--l-surface); border: 1px dashed var(--l-line); }
  .l1 { background: color-mix(in srgb, var(--l-accent) 22%, var(--l-surface)); }
  .l2 { background: var(--l-frame); color: #fff; }
  .agents { grid-column: span 3; }
  .agents::after { content: ""; position: absolute; inset-inline-end: -30px; bottom: -30px; width: 160px; height: 160px; background-image: radial-gradient(color-mix(in srgb, #7c3fc4 40%, transparent) 1.4px, transparent 1.7px); background-size: 11px 11px; mask-image: radial-gradient(circle, #000 30%, transparent 70%); }
  .flow { list-style: none; padding: 0; margin: 20px 0 0; display: flex; flex-wrap: wrap; gap: 8px; position: relative; z-index: 1; }
  .flow li { padding: 6px 12px; border-radius: 999px; border: 1px solid color-mix(in srgb, #7c3fc4 45%, var(--l-line)); color: var(--l-ink); font-size: 14px; background: var(--l-surface); }
  .flow li:not(:last-child)::after { content: "→"; margin-inline-start: 10px; color: var(--l-muted); display: inline-block; }
  :global([dir="rtl"]) .flow li:not(:last-child)::after { transform: scaleX(-1); }
  .flow li.person { border-color: var(--l-accent); color: var(--l-accent-ink); font-weight: 600; }
  .formula { grid-column: span 3; background: var(--l-frame); color: #fff; }
  .formula p { color: rgb(255 255 255 / 0.85); }
  .formula .big { font-size: 56px; line-height: 1; font-weight: 600; letter-spacing: -0.04em; margin: 0 0 16px; color: #fff; }
  .phones { grid-column: span 4; padding-top: 0; }
  .phones .art { margin: 0 -28px 20px; }
  .walls { grid-column: span 2; background: var(--l-tint); }

  /* Made with Zidara: the real screens, in a row of phones. */
  .made-with { padding-block: 112px 96px; }
  .made-with h2 { display: flex; gap: 14px; align-items: center; max-width: none; }
  .phones-row { list-style: none; margin: 0; padding: 0; display: grid; grid-template-columns: repeat(4, 1fr); gap: 24px; }
  .phones-row li:nth-child(even) { margin-top: 48px; }
  .device { border-radius: 30px; padding: 8px; background: var(--l-ink); box-shadow: 0 30px 50px -30px var(--l-shadow); }
  .device img { display: block; width: 100%; height: auto; border-radius: 23px; }
  .phones-row figure { margin: 0; }
  .phones-row figcaption { margin-top: 14px; font-size: 15px; color: var(--l-muted); }

  /* Proof: numbers in a row, nothing boxed. */
  .proof { padding-block: 104px; background: var(--l-surface); border-block: 1px solid var(--l-line); }
  .proof code { font-size: 0.95em; background: var(--l-tint); padding: 2px 8px; border-radius: 6px; }
  .stats { display: grid; grid-template-columns: repeat(5, 1fr); gap: 0; margin: 0 0 40px; }
  .stats div { padding: 4px 20px 0; border-inline-start: 1px solid var(--l-line); }
  .stats dt { white-space: nowrap; font-size: clamp(30px, 3.4vw, 46px); font-weight: 600; letter-spacing: -0.04em; color: var(--l-accent-ink); font-variant-numeric: tabular-nums; }
  .stats dd { margin: 4px 0 0; color: var(--l-muted); font-size: 15px; line-height: 1.45; }

  /* Questions: the title beside the answers. */
  .faq { padding-block: 104px; display: grid; grid-template-columns: 1fr 2fr; gap: 48px; }
  .qa details { border-bottom: 1px solid var(--l-line); }
  .qa summary { cursor: pointer; list-style: none; min-height: 64px; display: flex; align-items: center; justify-content: space-between; gap: 16px; font-weight: 600; font-size: 18px; }
  .qa summary::-webkit-details-marker { display: none; }
  .qa summary::after { content: "+"; font-size: 24px; font-weight: 400; color: var(--l-accent-ink); transition: transform 200ms ease; }
  .qa details[open] summary::after { transform: rotate(45deg); }
  .qa p { margin: 0 0 20px; color: var(--l-muted); max-width: 65ch; }

  .end-cta {
    display: flex; flex-wrap: wrap; gap: 24px; align-items: center; justify-content: space-between; padding: 48px; margin-bottom: 96px;
    border-radius: 20px; border: 1px solid var(--l-line); background: var(--l-surface); position: relative; overflow: hidden;
  }
  .end-cta::before { content: ""; position: absolute; inset: 0; background-image: radial-gradient(var(--l-dot) 1.4px, transparent 1.7px); background-size: 14px 14px; mask-image: linear-gradient(90deg, transparent 35%, #000 90%); pointer-events: none; }
  .end-cta > * { position: relative; }
  .end-cta h2 { margin-bottom: 6px; }
  .end-cta p { margin: 0; color: var(--l-muted); }
  .end-actions { display: flex; flex-wrap: wrap; gap: 16px; align-items: center; }
  .quiet { color: var(--l-muted); font-size: 15px; min-height: 44px; display: inline-flex; align-items: center; }

  .foot { border-top: 1px solid var(--l-line); overflow: hidden; }
  .row-foot { display: flex; flex-wrap: wrap; justify-content: space-between; gap: 12px; padding-block: 28px; font-size: 15px; color: var(--l-muted); }
  .row-foot p { margin: 0; display: inline-flex; gap: 8px; align-items: center; flex-wrap: wrap; }
  .made-tag { padding: 2px 10px; border-radius: 999px; background: var(--l-tint); color: var(--l-accent-ink); font-weight: 500; }
  .word {
    margin: 0; text-align: center; font-weight: 700; font-size: clamp(96px, 24vw, 340px); line-height: 0.8; letter-spacing: -0.05em;
    color: transparent; background-image: radial-gradient(var(--l-accent) 1.6px, transparent 2px); background-size: 9px 9px;
    -webkit-background-clip: text; background-clip: text; opacity: 0.55; user-select: none; direction: ltr;
  }

  /* Motion with a reason: the hero arrives in order; sections rise once as they come into view. */
  @media (prefers-reduced-motion: no-preference) {
    .pitch > * { animation: rise 700ms cubic-bezier(.16,1,.3,1) backwards; }
    .pitch > :nth-child(2) { animation-delay: 80ms; }
    .pitch > :nth-child(3) { animation-delay: 160ms; }
    .pitch > :nth-child(4) { animation-delay: 240ms; }
    .demo { animation: rise 900ms 200ms cubic-bezier(.16,1,.3,1) backwards; }
    :global(.landing [data-reveal]) { opacity: 0; transform: translateY(18px); transition: opacity 700ms cubic-bezier(.16,1,.3,1), transform 700ms cubic-bezier(.16,1,.3,1); transition-delay: calc(var(--i, 0) * 70ms); }
    :global(.landing [data-reveal="in"]) { opacity: 1; transform: none; }
  }
  @keyframes rise { from { opacity: 0; transform: translateY(16px); } }

  /* Tablets and phones: one column, every section declared. */
  @media (max-width: 960px) {
    .hero { grid-template-columns: 1fr; gap: 40px; padding-top: 48px; }
    .facts ul { grid-template-columns: repeat(2, 1fr); padding: 0; }
    .facts li { border-bottom: 1px solid var(--l-line); }
    .bento { grid-template-columns: 1fr 1fr; }
    .blocks, .phones { grid-column: span 2; }
    .layers, .agents, .formula, .walls { grid-column: span 1; }
    .phones-row { grid-template-columns: repeat(2, 1fr); }
    .stats { grid-template-columns: repeat(3, 1fr); row-gap: 32px; }
    .faq { grid-template-columns: 1fr; gap: 8px; }
  }
  @media (max-width: 720px) {
    .wrap { padding: 0 16px; }
    nav { display: none; }
    .bar { gap: 8px; }
    .brand span { display: none; }
    .lead { font-size: 18px; }
    .facts ul { grid-template-columns: 1fr; }
    .facts li { border-inline: 0 !important; padding: 14px 16px; }
    .ladder li { margin-inline-start: 0; grid-template-columns: 40px 1fr; gap: 14px; }
    .rung { width: 40px; height: 40px; }
    .root { margin-inline-start: 0; padding: 24px; flex-direction: column; }
    .bento { grid-template-columns: 1fr; }
    .blocks, .phones, .layers, .agents, .formula, .walls { grid-column: span 1; }
    .blocks ul { grid-template-columns: 1fr 1fr; }
    .shots { overflow-x: auto; scroll-snap-type: x mandatory; margin-inline: -16px; padding: 0 16px 8px; }
    .phones-row { display: flex; gap: 16px; }
    .phones-row li { flex: 0 0 68%; scroll-snap-align: center; margin-top: 0 !important; }
    .stats { grid-template-columns: 1fr 1fr; }
    .end-cta { padding: 28px; }
    .why, .made-with { padding-block: 72px; }
    .how, .proof, .faq { padding-block: 72px; }
  }
</style>
