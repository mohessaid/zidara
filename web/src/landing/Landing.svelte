<script lang="ts">
  // zidara.ta9in.com's front page (ADR 0014, ADR 0015). Zidara is pitched as docs/proposal.md
  // says: an adaptable learning engine, one kernel under any framework under any experience, with
  // AI agents inside approved rules. The demo course is shown as one experience among many. The
  // look (ADR 0017): a light page in the demo's own tokens, heavy Alexandria headings in ink, one
  // saffron accent used as a highlighter, a console in the hero, dot-matrix drawings that argue
  // each point. No logo, no gradients, no badges, no numbered labels.
  import { onMount } from "svelte";
  import { app } from "../lib/app.svelte.ts";
  import { LANDING } from "../lib/i18n/landing.ts";
  import LanguagePicker from "../components/LanguagePicker.svelte";
  import LiveEngine from "./LiveEngine.svelte";
  import Ask from "./Ask.svelte";
  import Pixel from "./Pixel.svelte";
  import { countUp, reveal } from "./reveal.ts";
  import * as S from "./scenes.ts";

  const copy = $derived(LANDING[app.locale]);
  const REPO = "https://github.com/mohessaid/zidara";
  const CARD_SCENES = [S.blocks, S.frameworks, S.experiences, S.agents, S.offline, S.walls];
  // The hero mounts first; each section below follows in its own later task, so a slow phone never
  // spends one long task building the whole page (Lighthouse TBT, ADR 0015). A link to a section
  // (/#how) still lands on it once it exists.
  const STAGES = 7;
  let stage = $state(0);
  onMount(() => {
    const idle = (globalThis as { requestIdleCallback?: (f: () => void, o?: { timeout: number }) => number }).requestIdleCallback;
    const next = (f: () => void) => (idle ? idle(f, { timeout: 300 }) : setTimeout(f, 30));
    const grow = () => {
      stage += 1;
      if (stage < STAGES) return next(grow);
      requestAnimationFrame(() => {
        const target = location.hash.length > 1 ? document.getElementById(location.hash.slice(1)) : null;
        if (target && target.closest(".landing")) target.scrollIntoView();
      });
    };
    next(grow);
  });
</script>

<div class="landing">
  <header class="nav">
    <div class="wrap bar">
      <a class="brand" href="#top" lang="en">zidara</a>
      <nav aria-label={copy.nav.menu}>
        <a href="#why">{copy.nav.why}</a>
        <a href="#how">{copy.nav.how}</a>
        <a href="#engine">{copy.nav.engine}</a>
        <a href="#ai">{copy.nav.ai}</a>
        <a href="#made">{copy.nav.made}</a>
      </nav>
      <div class="end">
        <LanguagePicker />
        <a class="cta small" href="#play" data-play>{copy.nav.play}</a>
      </div>
    </div>
  </header>

  <main id="top">
    <section class="hero">
      <div class="wrap pitch">
        <h1>{copy.hero.before}<em>{copy.hero.accent}</em>{copy.hero.after}</h1>
        <p class="lead">{copy.hero.lead}</p>
        <Ask copy={copy.ask} />
        <div class="ctas">
          <a class="cta" href="#engine">{copy.hero.primary}<span class="arrow" aria-hidden="true">→</span></a>
          <a class="ghost" href="#play" data-play>{copy.hero.secondary}</a>
        </div>
        <p class="note">{copy.hero.badge}</p>
      </div>
      <div class="wrap engine" id="engine">
        <div class="engine-head">
          <h2>{copy.engine.title}</h2>
          <p>{copy.engine.lead}</p>
        </div>
        <!-- The console follows the words in the next task; its frame holds the space meanwhile. -->
        {#if stage >= 1}<LiveEngine />{:else}<div class="engine-hold" aria-hidden="true"></div>{/if}
      </div>
    </section>

    {#if stage >= 1}
    <section class="facts" aria-label={copy.nav.how}>
      <ul class="wrap">
        {#each copy.facts as fact (fact)}<li><span class="tick" aria-hidden="true"></span>{fact}</li>{/each}
      </ul>
    </section>
    {/if}

    {#if stage >= 2}
    <section class="ruled why" id="why" aria-labelledby="why-title">
      <div class="wrap split">
        <div class="side">
          <h2 id="why-title" use:reveal>{copy.why.title}</h2>
          <p class="sub" use:reveal>{copy.why.lead}</p>
        </div>
        <div>
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
            <h3>{copy.why.rootTitle}</h3>
            <p>{copy.why.root}</p>
          </div>
          <div class="sources">
            <h3>{copy.why.sources}</h3>
            <ol>
              {#each copy.sources as source, i (source.href)}
                <li id={`src-${i + 1}`}><a href={source.href} rel="noopener" target="_blank">{source.label}</a></li>
              {/each}
            </ol>
          </div>
        </div>
      </div>
    </section>
    {/if}

    {#if stage >= 3}
    <section class="ruled how" id="how" aria-labelledby="how-title">
      <div class="wrap">
        <h2 id="how-title" use:reveal>{copy.how.title}</h2>
        <p class="sub" use:reveal>{copy.how.lead}</p>
        <div class="diagram" use:reveal>
          <div class="legend">
            <h3>{copy.how.layersTitle}</h3>
            <ol>
              {#each copy.how.layers as layer, i (layer.title)}
                <li class={`layer l${i}`}><strong>{layer.title}</strong><span>{layer.body}</span></li>
              {/each}
            </ol>
          </div>
          <div class="art big"><Pixel scene={S.engine} cols={S.ENGINE.cols} rows={S.ENGINE.rows} still={3} /></div>
        </div>
        <div class="cards">
          {#each copy.how.cards as card, i (card.title)}
            <article class="card" use:reveal style:--i={i % 3}>
              <div class="art"><Pixel scene={CARD_SCENES[i]!} cols={S.CARD.cols} rows={S.CARD.rows} /></div>
              <h3>{card.title}</h3>
              <p>{card.body}</p>
            </article>
          {/each}
        </div>
      </div>
    </section>
    {/if}

    {#if stage >= 4}
    <section class="ruled ai" id="ai" aria-labelledby="ai-title">
      <div class="wrap">
        <h2 id="ai-title" use:reveal>{copy.ai.title}</h2>
        <p class="sub" use:reveal>{copy.ai.lead}</p>
        <ol class="flow" use:reveal>
          {#each copy.ai.flow as step, i (step)}<li class:person={i === copy.ai.flow.length - 1}><span class="step" lang="en" dir="ltr">{i + 1}</span><span>{step}</span></li>{/each}
        </ol>
        <ul class="agents">
          {#each copy.ai.agents as agent, i (agent.name)}
            <li use:reveal style:--i={i}>
              <span class="status" class:today={agent.today}>{agent.today ? copy.ai.today : copy.ai.designed}</span>
              <h3>{agent.name}</h3>
              <p>{agent.body}</p>
            </li>
          {/each}
        </ul>
      </div>
    </section>
    {/if}

    {#if stage >= 5}
    <section class="ruled made-with" id="made" aria-labelledby="made-title">
      <div class="wrap">
        <h2 id="made-title" use:reveal>{copy.made.title}</h2>
        <p class="sub" use:reveal>{copy.made.lead}</p>
        <ul class="made">
          {#each copy.made.items as item, i (item.title)}
            <li use:reveal style:--i={i}>
              <a href={item.href}>
                <div class="art tall shot"><img src={item.shot} alt={item.alt} loading="lazy" decoding="async" /></div>
                <h3>{item.title}</h3>
                <p>{item.body}</p>
                <span class="open">{copy.made.open}<span class="arrow" aria-hidden="true">→</span></span>
              </a>
            </li>
          {/each}
        </ul>
      </div>
    </section>
    {/if}

    {#if stage >= 6}
    <section class="ruled proof" id="proof" aria-labelledby="proof-title">
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
    {/if}

    {#if stage >= 7}
    <section class="ruled faq" aria-labelledby="faq-title">
      <div class="wrap split">
        <div class="side">
          <h2 id="faq-title">{copy.faq.title}</h2>
        </div>
        <div class="qa">
          {#each copy.faq.items as item (item.q)}
            <details>
              <summary>{item.q}</summary>
              <p>{item.a}</p>
            </details>
          {/each}
        </div>
      </div>
    </section>
    {/if}

    {#if stage >= 7}
    <section class="wrap end-wrap">
      <div class="end-cta" use:reveal>
        <div class="end-in">
          <div>
            <h2>{copy.end.title}</h2>
            <p>{copy.end.lead}</p>
          </div>
          <div class="end-actions">
            <a class="cta" href="#play" data-play>{copy.end.primary}<span class="arrow" aria-hidden="true">→</span></a>
            <a class="ghost" href="#classes">{copy.end.classes}</a>
          </div>
        </div>
      </div>
    </section>
    {/if}
  </main>

  <footer class="foot">
    {#if stage >= STAGES}
    <div class="wrap row-foot">
      <p><strong lang="en">zidara</strong> {copy.footer.line}</p>
      <p><a href={REPO} rel="noopener" target="_blank">{copy.footer.code}</a> <span>· {copy.footer.made}</span></p>
    </div>
    <p class="words" lang={app.locale}>
      {#each copy.footer.words as w, i (w)}<span class="w" style:--n={i} style:--of={copy.footer.words.length}>{w}</span>{#if i < copy.footer.words.length - 1}<span class="sep">{app.locale === "ar" ? "، " : ", "}</span>{/if}{/each}
    </p>
    {/if}
  </footer>
</div>

<style>
  /* The page's design system (ADR 0017): light, in the demo's own tokens, so the pitch and the
     product are one system. White ground, ink headings in heavy Alexandria, grey for reading,
     saffron only as a highlighter and in the drawings, never as text or as the face of a control.
     No logo, no gradients, no badges. Contrast, measured: on white ink 19.2, muted 7.4, accent
     text 6.3; on --l-surface ink 17.4, muted 6.7, accent text 5.8; white on ink 19.2; ink on
     saffron 10.7. Control edges (--l-line-strong) 3.4 on white, 3.1 on the surface; --l-line is
     only for separators. tests/static/contrast.spec.ts measures the page. */
  .landing {
    --l-bg: #ffffff;
    --l-surface: #f4f4f5;
    --l-raise: #e4e4e7;
    --l-ink: #0f0f10;
    --l-muted: #55555d;
    --l-line: #e4e4e7;
    --l-line-strong: #8a8a93;    /* the edge of every control: 3.4 on white */
    --l-tint: #f4f4f5;
    --l-accent: #f7b63c;          /* saffron: a highlighter and the drawings' lit dots */
    --l-accent-ink: #8a5300;      /* saffron as words */
    --l-on-accent: #0f0f10;
    --l-locked: #8a8a93;
    --l-mono: ui-monospace, "SF Mono", "JetBrains Mono", Menlo, Consolas, monospace;
    --l-display: "Alexandria Variable", "Alexandria", "Readex Pro Variable", system-ui, sans-serif;
    --px-ring: #e4e4e7;
    --px-dim: #a1a1aa;
    --px-on: #f7b63c;
    --px-ink: #0f0f10;
    font-family: "Readex Pro Variable", "Readex Pro", system-ui, sans-serif;
    font-size: 17px;
    line-height: 1.65;
    color: var(--l-ink);
    background: var(--l-bg);
    min-height: 100dvh;
    color-scheme: light;
  }
  main { max-width: none; margin: 0; padding: 0; }
  .wrap { max-width: 1200px; margin: 0 auto; padding: 0 24px; }
  a { color: inherit; }
  :global(.landing button) { box-shadow: none; }
  :global(.landing :focus-visible) { outline: 3px solid var(--l-ink); outline-offset: 3px; border-radius: 6px; }

  /* Navigation: the name in words, the sections, the language, one button. */
  .nav { position: sticky; top: 0; z-index: 20; background: var(--l-bg); border-bottom: 1px solid var(--l-line); }
  .bar { display: flex; align-items: center; gap: 24px; min-height: 64px; }
  .brand { font: 900 22px/1 var(--l-display); letter-spacing: -0.03em; text-decoration: none; }
  nav { display: flex; gap: 22px; }
  nav a { text-decoration: none; color: var(--l-muted); font-size: 15px; font-weight: 500; }
  nav a:hover { color: var(--l-ink); text-decoration: underline; text-underline-offset: 4px; }
  .end { margin-inline-start: auto; display: flex; align-items: center; gap: 10px; }
  .end :global(select) { min-height: 40px; border: 2px solid var(--l-line-strong); border-radius: 999px; background: var(--l-bg); color: var(--l-ink); font-weight: 500; font-size: 14px; }

  /* Buttons: ink pills for the one thing to do, outlined pills for the other. */
  .cta {
    display: inline-flex; align-items: center; gap: 10px; min-height: 52px; padding: 0 24px; border-radius: 999px;
    background: var(--l-ink); color: #fff; font-weight: 600; text-decoration: none; white-space: nowrap;
    transition: transform 160ms ease, background-color 160ms ease;
  }
  .cta:hover { background: #26262a; transform: translateY(-1px); }
  .cta:active { transform: translateY(1px) scale(0.98); }
  .cta.small { min-height: 40px; padding: 0 18px; font-size: 14.5px; }
  .ghost { display: inline-flex; align-items: center; gap: 10px; min-height: 52px; padding: 0 24px; border-radius: 999px; border: 2px solid var(--l-line-strong); background: var(--l-bg); color: var(--l-ink); text-decoration: none; font-weight: 600; white-space: nowrap; }
  .ghost:hover { border-color: var(--l-ink); }
  .arrow { display: inline-block; transition: transform 160ms ease; }
  .cta:hover .arrow, .ghost:hover .arrow, .made a:hover .arrow { transform: translateX(3px); }
  :global([dir="rtl"]) .arrow { transform: scaleX(-1); }
  :global([dir="rtl"]) .cta:hover .arrow, :global([dir="rtl"]) .ghost:hover .arrow, :global([dir="rtl"]) .made a:hover .arrow { transform: scaleX(-1) translateX(3px); }

  /* Headings in heavy Alexandria; Arabic keeps its natural spacing (tracking breaks the joins). */
  h1, h2, h3 { font-family: var(--l-display); letter-spacing: -0.025em; color: var(--l-ink); }
  h1:lang(ar), h2:lang(ar), h3:lang(ar), :global([lang="ar"]) :is(h1, h2, h3) { letter-spacing: 0; }
  h2 { font-size: clamp(32px, 4vw, 54px); line-height: 1.1; margin: 0 0 14px; font-weight: 900; max-width: 22ch; }
  h3 { font-weight: 700; }
  .sub { color: var(--l-muted); max-width: 60ch; margin: 0 0 44px; font-size: 18px; }
  section { scroll-margin-top: 72px; }
  /* Below the fold, the browser skips layout and paint until a section nears the screen. */
  .ruled, .end-wrap { content-visibility: auto; contain-intrinsic-size: auto 900px; }
  .ruled { border-top: 1px solid var(--l-line); padding-block: 112px; position: relative; }
  /* Every other section sits on the grey surface, so the page has rhythm without ornaments. */
  .why, .ai, .proof { background: var(--l-surface); }

  /* Hero: the claim, the console that proves it, then the live engine. */
  .hero { position: relative; padding: 96px 0 80px; }
  .pitch { text-align: center; display: grid; justify-items: center; }
  h1 { font-size: clamp(42px, 6.2vw, 84px); line-height: 1.05; margin: 0 0 24px; font-weight: 900; letter-spacing: -0.035em; max-width: 15ch; }
  /* The one highlighted phrase: a flat saffron marker under ink words (ink on saffron 10.7). */
  h1 em { font-style: normal; background: var(--l-accent); box-decoration-break: clone; -webkit-box-decoration-break: clone; padding: 0 0.12em; border-radius: 0.12em; }
  .lead { font-size: 19px; color: var(--l-muted); max-width: 56ch; margin: 0 0 36px; }
  .ctas { display: flex; flex-wrap: wrap; justify-content: center; gap: 12px; margin-top: 28px; }
  .note { margin: 18px 0 0; font-size: 15px; color: var(--l-muted); }
  .engine { max-width: 880px; margin-top: 80px; scroll-margin-top: 80px; }
  .engine-head { text-align: center; margin-bottom: 20px; }
  .engine-hold { min-height: 520px; border-radius: 20px; border: 1px solid var(--l-line); background: var(--l-bg); }
  .engine-head h2 { font-size: clamp(22px, 2.4vw, 30px); max-width: none; margin-bottom: 6px; font-weight: 800; }
  .engine-head p { margin: 0 auto; color: var(--l-muted); max-width: 60ch; font-size: 16px; }

  .facts { border-top: 1px solid var(--l-line); }
  .facts ul { list-style: none; margin: 0 auto; display: grid; grid-template-columns: repeat(6, 1fr); }
  .facts li { display: flex; gap: 10px; align-items: flex-start; padding: 22px 14px; font-size: 15px; font-weight: 500; color: var(--l-ink); border-inline-start: 1px solid var(--l-line); line-height: 1.4; }
  .facts li:last-child { border-inline-end: 1px solid var(--l-line); }
  .tick { width: 8px; height: 8px; margin-top: 6px; flex: none; background: var(--l-ink); border-radius: 50%; }

  .split { display: grid; grid-template-columns: 1fr 1.7fr; gap: 56px; }
  .side { align-self: start; position: sticky; top: 96px; }

  /* Vision: the five whys, numbered because they are a real sequence. */
  .ladder { list-style: none; margin: 0; padding: 0; display: grid; }
  .ladder li { display: grid; grid-template-columns: 56px 1fr; gap: 16px; padding: 22px 0; border-bottom: 1px solid var(--l-raise); }
  .rung { font: 900 40px/1 var(--l-display); color: var(--l-ink); }
  .ladder h3 { margin: 4px 0 6px; font-size: 21px; font-weight: 700; }
  .ladder p { margin: 0; color: var(--l-muted); }
  .cite { font-size: 13px; color: var(--l-accent-ink); text-decoration: none; margin-inline-start: 4px; vertical-align: super; }
  .sources { margin-top: 28px; font-size: 14px; color: var(--l-muted); }
  .sources h3 { font: 600 13px/1 var(--l-mono); text-transform: uppercase; letter-spacing: 0.06em; margin: 0 0 10px; color: var(--l-muted); }
  .sources ol { padding-inline-start: 20px; margin: 4px 0 0; }
  .sources li { margin-bottom: 6px; }
  /* The root cause, set apart by a thick ink rule, not a frame. */
  .root { margin-top: 32px; padding: 6px 0 6px 22px; border-inline-start: 6px solid var(--l-ink); }
  :global([dir="rtl"]) .root { padding: 6px 22px 6px 0; }
  .root h3 { margin: 0 0 6px; font-size: 24px; font-weight: 800; }
  .root p { margin: 0; color: var(--l-ink); font-size: 18px; }

  /* Architecture: the layer diagram, then six cards, each with its drawing. */
  .diagram { display: grid; grid-template-columns: 1fr 1.3fr; border: 1px solid var(--l-raise); border-radius: 16px; overflow: hidden; margin-bottom: 16px; }
  .legend { padding: 28px; border-inline-end: 1px solid var(--l-raise); }
  .legend h3 { margin: 0 0 16px; font-size: 19px; }
  .legend ol { list-style: none; margin: 0; padding: 0; display: grid; gap: 10px; }
  .layer { display: grid; gap: 2px; padding: 12px 14px; border-radius: 10px; border: 1px solid var(--l-raise); }
  .layer strong { font: 600 13px/1.2 var(--l-mono); text-transform: uppercase; letter-spacing: 0.05em; color: var(--l-accent-ink); }
  .layer span { font-size: 15px; color: var(--l-muted); line-height: 1.45; }
  .l2 { background: var(--l-surface); }
  .l3 { border-style: dashed; border-color: var(--l-line-strong); }
  .art { position: relative; background: var(--l-surface); border-radius: 12px; aspect-ratio: 36 / 22; --px-inset: 10px; }
  .art.big { border-radius: 0; aspect-ratio: auto; min-height: 380px; --px-inset: 24px; }
  .art.tall { aspect-ratio: 30 / 40; }
  /* Real screens of the app, cropped to the top of each screen. */
  .shot { padding: 12px 12px 0; overflow: hidden; }
  .shot img { display: block; width: 100%; height: 100%; object-fit: cover; object-position: top; border-radius: 8px 8px 0 0; border: 1px solid var(--l-raise); border-bottom: 0; }
  .cards { display: grid; grid-template-columns: repeat(3, 1fr); gap: 32px 24px; }
  .card, .made li, .agents li { min-width: 0; }
  .card h3 { margin: 18px 0 6px; font-size: 20px; }
  .card p { margin: 0; color: var(--l-muted); font-size: 16px; }

  /* AI agents: the chain of hands, then the four agents as plain columns. */
  .flow { list-style: none; padding: 0; margin: 0 0 40px; display: flex; flex-wrap: wrap; gap: 6px 4px; align-items: baseline; font-size: 17px; font-weight: 500; }
  .flow li { display: inline-flex; align-items: baseline; gap: 8px; }
  .flow li:not(:last-child)::after { content: "→"; margin: 0 10px 0 6px; color: var(--l-muted); }
  :global([dir="rtl"]) .flow li:not(:last-child)::after { content: "←"; }
  .step { font: 700 13px/1 var(--l-mono); color: var(--l-muted); }
  .flow li.person { font-weight: 800; }
  .flow li.person > :not(.step) { background: var(--l-accent); }
  .agents { list-style: none; padding: 0; margin: 0; display: grid; grid-template-columns: repeat(4, 1fr); gap: 0; border-top: 2px solid var(--l-ink); }
  .agents li { padding: 22px 22px 8px 0; border-inline-end: 1px solid var(--l-raise); padding-inline-start: 22px; }
  .agents li:first-child { padding-inline-start: 0; }
  .agents li:last-child { border-inline-end: 0; }
  .agents h3 { margin: 8px 0 6px; font-size: 19px; }
  .agents p { margin: 0; color: var(--l-muted); font-size: 15.5px; }
  /* Today or designed: said in words, with a tick for what runs today. */
  .status { font: 600 13px/1.2 var(--l-mono); color: var(--l-muted); text-transform: uppercase; letter-spacing: 0.04em; }
  .status.today { color: #17773f; }
  .status.today::before { content: "✓ "; }

  /* Made with Zidara: real screens, then words; no card around them. */
  .made { list-style: none; margin: 0; padding: 0; display: grid; grid-template-columns: repeat(4, 1fr); gap: 24px; }
  .made a { display: grid; height: 100%; align-content: start; text-decoration: none; border-radius: 14px; }
  .made a:hover h3 { text-decoration: underline; text-underline-offset: 4px; }
  .made a:hover .art { background: var(--l-raise); }
  .made h3 { margin: 16px 0 6px; font-size: 19px; }
  .made p { margin: 0 0 12px; color: var(--l-muted); font-size: 15.5px; }
  .open { margin-top: auto; display: inline-flex; gap: 8px; align-items: center; font-weight: 700; color: var(--l-ink); }

  /* Proof. */
  .proof code { font-size: 0.92em; background: var(--l-bg); padding: 2px 8px; border-radius: 6px; font-family: var(--l-mono); color: var(--l-ink); }
  .stats { display: grid; grid-template-columns: repeat(5, 1fr); margin: 0 0 40px; border-block: 2px solid var(--l-ink); }
  .stats div { padding: 24px 20px; border-inline-start: 1px solid var(--l-raise); }
  .stats div:first-child { border-inline-start: 0; padding-inline-start: 0; }
  .stats dt { white-space: nowrap; font: 900 clamp(32px, 3.4vw, 48px)/1.1 var(--l-display); letter-spacing: -0.04em; color: var(--l-ink); font-variant-numeric: tabular-nums; }
  .stats dd { margin: 6px 0 0; color: var(--l-muted); font-size: 15px; line-height: 1.45; }

  /* Questions. */
  .qa details { border-bottom: 1px solid var(--l-raise); }
  .qa summary { cursor: pointer; list-style: none; min-height: 64px; display: flex; align-items: center; justify-content: space-between; gap: 16px; font-weight: 600; font-size: 18px; }
  .qa summary::-webkit-details-marker { display: none; }
  .qa summary::after { content: "+"; font: 400 24px/1 var(--l-mono); color: var(--l-ink); transition: transform 200ms ease; }
  .qa details[open] summary::after { transform: rotate(45deg); }
  .qa p { margin: 0 0 20px; color: var(--l-muted); max-width: 65ch; }

  /* The last call: an ink block, the one dark surface on the page (white on ink 19.2). */
  .end-wrap { padding-block: 96px; }
  .end-cta { border-radius: 24px; background: var(--l-ink); color: #fff; }
  .end-in { padding: 56px; display: flex; justify-content: space-between; align-items: center; flex-wrap: wrap; gap: 24px; }
  .end-in h2 { margin-bottom: 8px; color: #fff; }
  .end-in p { margin: 0; color: #d4d4d8; }
  .end-actions { display: flex; flex-wrap: wrap; gap: 12px; align-items: center; }
  .end-cta .cta { background: #fff; color: var(--l-ink); }
  .end-cta .cta:hover { background: var(--l-accent); }
  .end-cta .ghost { background: var(--l-ink); color: #fff; border-color: #a1a1aa; }
  .end-cta .ghost:hover { border-color: #fff; }

  .foot { border-top: 1px solid var(--l-line); overflow: hidden; }
  .row-foot { display: flex; flex-wrap: wrap; justify-content: space-between; gap: 12px; padding-block: 28px; font-size: 15px; color: var(--l-muted); }
  .row-foot p { margin: 0; display: inline-flex; gap: 8px; align-items: center; flex-wrap: wrap; }
  .row-foot strong { color: var(--l-ink); font-family: var(--l-display); font-weight: 900; }
  /* The closing phrase: corner brackets travel from word to word (mastrilo's footer, our words). */
  .words { max-width: 1200px; margin: 0 auto; padding: 56px 24px 72px; text-align: center; font: 900 clamp(30px, 6vw, 84px)/1.15 var(--l-display); letter-spacing: -0.035em; }
  .words:lang(ar) { letter-spacing: 0; }
  .w { position: relative; display: inline-block; padding: 0 0.12em; }
  .w::before, .w::after { content: ""; position: absolute; inset: -0.04em 0; border: 3px solid var(--l-ink); opacity: 0; pointer-events: none; }
  .w::before { border-inline-end: 0; width: 0.22em; inset-inline-end: auto; }
  .w::after { border-inline-start: 0; width: 0.22em; inset-inline-start: auto; }
  .sep { color: var(--l-muted); }

  /* Motion with a reason: the hero arrives in order; sections rise once as they come into view. */
  @media (prefers-reduced-motion: no-preference) {
    .pitch > * { animation: rise 700ms cubic-bezier(.16,1,.3,1) backwards; }
    .pitch > :nth-child(2) { animation-delay: 80ms; }
    .pitch > :nth-child(3) { animation-delay: 160ms; }
    .pitch > :nth-child(4) { animation-delay: 240ms; }
    .engine { animation: rise 900ms 260ms cubic-bezier(.16,1,.3,1) backwards; }
    /* Sections slide into place without fading: text is never caught half transparent (and so
       never measured below its real contrast). */
    :global(.landing [data-reveal]) { transform: translateY(18px); transition: transform 700ms cubic-bezier(.16,1,.3,1); transition-delay: calc(var(--i, 0) * 70ms); }
    :global(.landing [data-reveal="in"]) { transform: none; }
  }
  @keyframes rise { from { opacity: 0; transform: translateY(16px); } }
  @media (prefers-reduced-motion: no-preference) {
    .w::before, .w::after { animation: frame calc(var(--of) * 2.4s) calc(var(--n) * 2.4s) infinite steps(1); }
  }
  @media (prefers-reduced-motion: reduce) { .w:first-child::before, .w:first-child::after { opacity: 1; } }
  @keyframes frame { 0% { opacity: 1; } 33.34% { opacity: 0; } }

  @media (max-width: 1080px) {
    nav { gap: 14px; }
    nav a:nth-child(5) { display: none; }
    .facts ul { grid-template-columns: repeat(3, 1fr); padding: 0; }
    .facts li { border-bottom: 1px solid var(--l-line); }
    .agents { grid-template-columns: repeat(2, 1fr); }
    .agents li, .agents li:first-child { padding-inline: 0 22px; border-inline-end: 0; border-bottom: 1px solid var(--l-raise); padding-bottom: 22px; }
    .made { grid-template-columns: repeat(2, 1fr); }
    .stats { grid-template-columns: repeat(3, 1fr); }
    .stats div { padding-inline-start: 0; border-inline-start: 0; }
  }
  @media (max-width: 860px) {
    nav { display: none; }
    .split { grid-template-columns: 1fr; gap: 8px; }
    .side { position: static; }
    .diagram { grid-template-columns: 1fr; }
    .legend { border-inline-end: 0; border-bottom: 1px solid var(--l-raise); }
    .art.big { min-height: 0; aspect-ratio: 64 / 40; }
    .cards { grid-template-columns: 1fr 1fr; }
  }
  @media (max-width: 620px) {
    .wrap { padding: 0 16px; }
    .bar { gap: 8px; }
    .hero { padding-top: 56px; }
    .lead { font-size: 17.5px; }
    .engine { margin-top: 56px; }
    .facts ul { grid-template-columns: 1fr 1fr; }
    .facts li { border-inline: 0 !important; padding: 14px 4px; }
    .cards, .agents, .made { grid-template-columns: 1fr; }
    .stats { grid-template-columns: 1fr 1fr; }
    .stats div { border-bottom: 1px solid var(--l-raise); }
    .ruled { padding-block: 72px; }
    .end-in { padding: 28px; }
    .ladder li { grid-template-columns: 40px 1fr; }
    .rung { font-size: 30px; }
  }
</style>
