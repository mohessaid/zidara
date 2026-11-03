<script lang="ts">
  // zidara.ta9in.com's front page (ADR 0014, ADR 0015). Zidara is pitched as docs/proposal.md
  // says: an adaptable learning engine, one kernel under any framework under any experience, with
  // AI agents inside approved rules. The demo course is shown as one experience among many. The
  // look (ADR 0016): a dark, high-contrast page with heavy Alexandria headings, one saffron accent,
  // a console in the hero, dot-matrix drawings that argue each point, and no numbered labels.
  import { onMount } from "svelte";
  import { app } from "../lib/app.svelte.ts";
  import { LANDING } from "../lib/i18n/landing.ts";
  import LanguagePicker from "../components/LanguagePicker.svelte";
  import LiveEngine from "./LiveEngine.svelte";
  import Mark from "./Mark.svelte";
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
      <a class="brand" href="#top" aria-label="Zidara"><Mark size={24} /><span lang="en">zidara</span></a>
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
        <p class="badge"><Mark size={14} />{copy.hero.badge}</p>
        <h1>{copy.hero.before}<em>{copy.hero.accent}</em>{copy.hero.after}</h1>
        <p class="lead">{copy.hero.lead}</p>
        <Ask copy={copy.ask} />
        <div class="ctas">
          <a class="cta" href="#engine">{copy.hero.primary}<span class="arrow" aria-hidden="true">→</span></a>
          <a class="ghost" href="#play" data-play>{copy.hero.secondary}</a>
        </div>
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
          <div class="root frame" use:reveal>
            <div class="frame-in">
              <Mark size={34} />
              <div>
                <h3>{copy.why.rootTitle}</h3>
                <p>{copy.why.root}</p>
              </div>
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
          {#each copy.ai.flow as step, i (step)}<li class:person={i === copy.ai.flow.length - 1}><span class="step" lang="en" dir="ltr">{i + 1}</span>{step}</li>{/each}
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
      <div class="end-cta frame" use:reveal>
        <div class="frame-in end-in">
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
      <p><Mark size={18} /> <strong lang="en">zidara</strong> {copy.footer.line}</p>
      <p><a href={REPO} rel="noopener" target="_blank">{copy.footer.code}</a> <span class="made-tag">{copy.footer.made}</span></p>
    </div>
    <p class="words" lang={app.locale}>
      {#each copy.footer.words as w, i (w)}<span class="w" style:--n={i} style:--of={copy.footer.words.length}>{w}</span>{#if i < copy.footer.words.length - 1}<span class="sep">{app.locale === "ar" ? "، " : ", "}</span>{/if}{/each}
    </p>
    {/if}
  </footer>
</div>

<style>
  /* The page's design system (ADR 0016). Dark and high-contrast in every scheme, like a stage:
     near-black ground, white headings in heavy Alexandria, muted grey for reading, one saffron
     accent. Contrast on --l-bg: ink 18.9, muted 8.6, accent 11.6; ink on the accent 11.6. */
  .landing {
    --l-bg: #09090b;
    --l-surface: #111114;
    --l-raise: #18181c;
    --l-ink: #f5f5f4;
    --l-muted: #a8a8ae;
    --l-line: #26262b;
    --l-line-strong: #3a3a41;
    --l-tint: #141417;
    --l-accent: #f7b63c;          /* saffron: one accent, the whole page */
    --l-accent-ink: #f7b63c;
    --l-on-accent: #09090b;
    --l-locked: #4a4a52;
    --l-mono: ui-monospace, "SF Mono", "JetBrains Mono", Menlo, Consolas, monospace;
    --l-display: "Alexandria Variable", "Alexandria", "Readex Pro Variable", system-ui, sans-serif;
    --px-ring: #2a2a30;
    --px-dim: #6b6b74;
    --px-on: #f7b63c;
    --px-ink: #e7e7e4;
    font-family: "Readex Pro Variable", "Readex Pro", system-ui, sans-serif;
    font-size: 17px;
    line-height: 1.65;
    color: var(--l-ink);
    background: var(--l-bg);
    min-height: 100dvh;
    color-scheme: dark;
  }
  main { max-width: none; margin: 0; padding: 0; }
  .wrap { max-width: 1200px; margin: 0 auto; padding: 0 24px; }
  a { color: inherit; }
  /* The app's buttons press into a light lip; on this dark page they stand flat. */
  :global(.landing button) { box-shadow: none; }
  :global(.landing :focus-visible) { outline: 3px solid var(--l-accent); outline-offset: 3px; border-radius: 6px; }

  /* Navigation. */
  .nav { position: sticky; top: 0; z-index: 20; background: color-mix(in srgb, var(--l-bg) 88%, transparent); backdrop-filter: blur(10px); border-bottom: 1px solid var(--l-line); }
  .bar { display: flex; align-items: center; gap: 24px; min-height: 64px; }
  .brand { display: inline-flex; align-items: center; gap: 8px; font-weight: 700; font-size: 20px; letter-spacing: -0.02em; text-decoration: none; }
  nav { display: flex; gap: 20px; }
  nav a { text-decoration: none; color: var(--l-muted); font-size: 14.5px; }
  nav a:hover { color: var(--l-ink); }
  .end { margin-inline-start: auto; display: flex; align-items: center; gap: 10px; }
  .end :global(select) { min-height: 40px; border: 1px solid var(--l-line); border-bottom-width: 1px; border-radius: 999px; background: var(--l-surface); color: var(--l-ink); font-weight: 500; font-size: 14px; }

  /* Buttons: pills; cards are 16 px; frames 22 px. */
  .cta {
    display: inline-flex; align-items: center; gap: 10px; min-height: 50px; padding: 0 22px; border-radius: 999px;
    background: var(--l-accent); color: var(--l-on-accent); font-weight: 600; text-decoration: none; white-space: nowrap;
    transition: transform 160ms ease, background-color 160ms ease;
  }
  .cta:hover { transform: translateY(-1px); background: var(--l-ink); color: var(--l-bg); }
  .cta:active { transform: translateY(1px) scale(0.98); }
  .cta.small { min-height: 40px; padding: 0 16px; font-size: 14.5px; }
  .ghost { display: inline-flex; align-items: center; gap: 10px; min-height: 50px; padding: 0 22px; border-radius: 999px; border: 1px solid var(--l-line-strong); background: var(--l-surface); color: var(--l-ink); text-decoration: none; font-weight: 500; white-space: nowrap; }
  .ghost:hover { border-color: var(--l-ink); }
  .arrow { display: inline-block; transition: transform 160ms ease; }
  .cta:hover .arrow, .ghost:hover .arrow, .made a:hover .arrow { transform: translateX(3px); }
  :global([dir="rtl"]) .arrow { transform: scaleX(-1); }
  :global([dir="rtl"]) .cta:hover .arrow, :global([dir="rtl"]) .ghost:hover .arrow, :global([dir="rtl"]) .made a:hover .arrow { transform: scaleX(-1) translateX(3px); }

  /* Headings in heavy Alexandria; Arabic keeps its natural spacing (tracking breaks the joins). */
  h1, h2, h3 { font-family: var(--l-display); letter-spacing: -0.025em; }
  h1:lang(ar), h2:lang(ar), h3:lang(ar), :global([lang="ar"]) :is(h1, h2, h3) { letter-spacing: 0; }
  h2 { font-size: clamp(32px, 4vw, 52px); line-height: 1.12; margin: 0 0 14px; font-weight: 800; max-width: 22ch; }
  h3 { font-weight: 700; }
  .sub { color: var(--l-muted); max-width: 60ch; margin: 0 0 44px; font-size: 18px; }
  section { scroll-margin-top: 72px; }
  /* Below the fold, the browser skips layout and paint until a section nears the screen. */
  .ruled, .end-wrap { content-visibility: auto; contain-intrinsic-size: auto 900px; }

  /* Sections are separated by one thin line and room; no labels, no crosses. */
  .ruled { border-top: 1px solid var(--l-line); padding-block: 112px; position: relative; }

  /* Hero: centred words over the streams, then the engine in its blue frame. */
  .hero { position: relative; padding: 104px 0 80px; overflow: hidden; }
  /* One soft pool of saffron light behind the words; no picture to wait for. */
  .hero::before { content: ""; position: absolute; inset: -20% 10% auto; height: 620px; z-index: 0; pointer-events: none; background: radial-gradient(closest-side, rgb(247 182 60 / 0.13), transparent); }
  .pitch, .engine { position: relative; z-index: 1; }
  .pitch { text-align: center; display: grid; justify-items: center; }
  .badge { display: inline-flex; align-items: center; gap: 8px; margin: 0 0 22px; padding: 6px 14px; border: 1px solid var(--l-line); border-radius: 999px; font-size: 13.5px; color: var(--l-muted); background: var(--l-surface); }
  h1 { font-size: clamp(40px, 5.8vw, 78px); line-height: 1.1; margin: 0 0 24px; font-weight: 900; letter-spacing: -0.02em; max-width: 16ch; }
  h1 em { font-style: normal; color: var(--l-accent-ink); }
  .lead { font-size: 19px; color: var(--l-muted); max-width: 56ch; margin: 0 0 36px; }
  .ctas { display: flex; flex-wrap: wrap; justify-content: center; gap: 12px; margin-top: 28px; }
  .engine { max-width: 880px; margin-top: 72px; scroll-margin-top: 80px; }
  .engine-head { text-align: center; margin-bottom: 20px; }
  .engine-hold { min-height: 520px; border-radius: 22px; border: 1px solid var(--l-line); background: var(--l-surface); }
  .engine-head h2 { font-size: clamp(22px, 2.4vw, 28px); max-width: none; margin-bottom: 6px; }
  .engine-head p { margin: 0 auto; color: var(--l-muted); max-width: 60ch; font-size: 16px; }

  .facts { border-top: 1px solid var(--l-line); }
  .facts ul { list-style: none; margin: 0 auto; display: grid; grid-template-columns: repeat(6, 1fr); }
  .facts li { display: flex; gap: 10px; align-items: flex-start; padding: 20px 14px; font-size: 14.5px; font-weight: 500; color: var(--l-muted); border-inline-start: 1px solid var(--l-line); line-height: 1.4; }
  .facts li:last-child { border-inline-end: 1px solid var(--l-line); }
  .tick { width: 7px; height: 7px; margin-top: 6px; flex: none; background: var(--l-accent); }

  .split { display: grid; grid-template-columns: 1fr 1.7fr; gap: 56px; }
  .side { align-self: start; position: sticky; top: 96px; }

  /* Vision: the five whys. */
  .ladder { list-style: none; margin: 0; padding: 0; display: grid; }
  .ladder li { display: grid; grid-template-columns: 44px 1fr; gap: 18px; padding: 20px 0; border-bottom: 1px dashed var(--l-line); }
  .rung { width: 44px; height: 44px; display: grid; place-items: center; border-radius: 10px; border: 1px solid color-mix(in srgb, var(--l-accent) 40%, var(--l-line)); font: 600 15px/1 var(--l-mono); color: var(--l-accent-ink); background: var(--l-tint); }
  .ladder h3 { margin: 6px 0 6px; font-size: 20px; font-weight: 500; }
  .ladder p { margin: 0; color: var(--l-muted); }
  .cite { font-size: 13px; color: var(--l-accent-ink); text-decoration: none; margin-inline-start: 4px; vertical-align: super; }
  .sources { margin-top: 28px; font-size: 13.5px; color: var(--l-muted); }
  .sources h3 { font: 500 12.5px/1 var(--l-mono); text-transform: uppercase; letter-spacing: 0.06em; margin: 0 0 10px; }
  .sources ol { padding-inline-start: 20px; margin: 4px 0 0; }
  .sources li { margin-bottom: 6px; }

  /* The blue dotted frame (context.dev's product frame), for the root and the last call. */
  .frame { padding: 1px; border-radius: 20px; background: linear-gradient(140deg, var(--l-accent), var(--l-line) 45%); position: relative; margin-top: 28px; }
  .frame-in { background: var(--l-surface); border-radius: 19px; padding: 26px 28px; display: flex; gap: 18px; align-items: flex-start; }
  .root h3 { margin: 0 0 6px; font-size: 22px; font-weight: 500; }
  .root p { margin: 0; color: var(--l-muted); }

  /* Architecture: the layer diagram, then six cards, each with its drawing. */
  .diagram { display: grid; grid-template-columns: 1fr 1.3fr; gap: 0; border: 1px solid var(--l-line); border-radius: 16px; overflow: hidden; margin-bottom: 16px; }
  .legend { padding: 28px; border-inline-end: 1px solid var(--l-line); }
  .legend h3 { margin: 0 0 16px; font-size: 18px; font-weight: 500; }
  .legend ol { list-style: none; margin: 0; padding: 0; display: grid; gap: 10px; }
  .layer { display: grid; gap: 2px; padding: 12px 14px; border-radius: 10px; border: 1px solid var(--l-line); }
  .layer strong { font: 600 13px/1.2 var(--l-mono); text-transform: uppercase; letter-spacing: 0.05em; color: var(--l-accent-ink); }
  .layer span { font-size: 14.5px; color: var(--l-muted); line-height: 1.45; }
  .l2 { background: var(--l-tint); }
  .l3 { border-style: dashed; }
  .art { position: relative; background-color: var(--l-tint); background-image: radial-gradient(circle, color-mix(in srgb, var(--l-accent) 14%, transparent) 1px, transparent 1.4px); background-size: 9px 9px; border-radius: 12px; aspect-ratio: 36 / 22; --px-inset: 10px; }
  .art.big { border-radius: 0; aspect-ratio: auto; min-height: 380px; --px-inset: 24px; }
  .art.tall { aspect-ratio: 30 / 40; }
  /* Real screens of the app, framed like the drawings: the top of each screen, cropped to fit. */
  .shot { padding: 10px 10px 0; overflow: hidden; }
  .shot img { display: block; width: 100%; height: 100%; object-fit: cover; object-position: top; border-radius: 8px 8px 0 0; border: 1px solid var(--l-line); border-bottom: 0; }
  .cards { display: grid; grid-template-columns: repeat(3, 1fr); gap: 16px; }
  .card, .made li, .agents li { min-width: 0; }
  .card { border: 1px solid var(--l-line); border-radius: 16px; padding: 12px 12px 24px; background: var(--l-surface); }
  .card h3 { margin: 18px 12px 6px; font-size: 19px; font-weight: 500; }
  .card p { margin: 0 12px; color: var(--l-muted); font-size: 15.5px; }

  /* AI agents. */
  .flow { list-style: none; padding: 0; margin: 0 0 28px; display: flex; flex-wrap: wrap; gap: 10px; align-items: center; }
  .flow li { display: inline-flex; align-items: center; gap: 10px; padding: 8px 14px 8px 8px; border-radius: 999px; border: 1px solid var(--l-line); font-size: 15px; background: var(--l-surface); }
  .flow li:not(:last-child)::after { content: "→"; margin-inline-start: 6px; color: var(--l-muted); }
  :global([dir="rtl"]) .flow li:not(:last-child)::after { content: "←"; }
  .step { width: 26px; height: 26px; display: grid; place-items: center; border-radius: 50%; background: var(--l-tint); font: 600 12.5px/1 var(--l-mono); color: var(--l-accent-ink); }
  .flow li.person { border-color: var(--l-accent); color: var(--l-accent-ink); font-weight: 600; }
  .flow li.person .step { background: var(--l-accent); color: var(--l-on-accent); }
  .agents { list-style: none; padding: 0; margin: 0; display: grid; grid-template-columns: repeat(4, 1fr); gap: 16px; }
  .agents li { border: 1px solid var(--l-line); border-radius: 16px; padding: 22px; background: var(--l-surface); }
  .agents h3 { margin: 14px 0 6px; font-size: 18px; font-weight: 500; }
  .agents p { margin: 0; color: var(--l-muted); font-size: 15px; }
  .status { display: inline-block; padding: 4px 9px; border-radius: 6px; font: 500 12px/1.2 var(--l-mono); border: 1px dashed var(--l-line); color: var(--l-muted); }
  .status.today { border: 1px solid var(--l-accent); color: var(--l-accent-ink); background: var(--l-tint); }

  /* Made with Zidara. */
  .made { list-style: none; margin: 0; padding: 0; display: grid; grid-template-columns: repeat(4, 1fr); gap: 16px; }
  .made a { display: grid; height: 100%; align-content: start; text-decoration: none; border: 1px solid var(--l-line); border-radius: 16px; padding: 12px 12px 20px; background: var(--l-surface); transition: border-color 160ms ease, transform 160ms ease; }
  .made a:hover { border-color: var(--l-accent); transform: translateY(-2px); }
  .made h3 { margin: 16px 10px 6px; font-size: 18px; font-weight: 500; }
  .made p { margin: 0 10px 14px; color: var(--l-muted); font-size: 15px; }
  .open { margin: auto 10px 0; display: inline-flex; gap: 8px; align-items: center; font-weight: 600; color: var(--l-accent-ink); }

  /* Proof. */
  .proof code { font-size: 0.92em; background: var(--l-tint); padding: 2px 8px; border-radius: 6px; font-family: var(--l-mono); }
  .stats { display: grid; grid-template-columns: repeat(5, 1fr); margin: 0 0 40px; border-block: 1px solid var(--l-line); }
  .stats div { padding: 24px 20px; border-inline-start: 1px solid var(--l-line); }
  .stats div:last-child { border-inline-end: 1px solid var(--l-line); }
  .stats dt { white-space: nowrap; font-size: clamp(30px, 3.2vw, 44px); font-weight: 500; letter-spacing: -0.04em; color: var(--l-accent-ink); font-variant-numeric: tabular-nums; }
  .stats dd { margin: 4px 0 0; color: var(--l-muted); font-size: 14.5px; line-height: 1.45; }

  /* Questions. */
  .qa details { border-bottom: 1px solid var(--l-line); }
  .qa summary { cursor: pointer; list-style: none; min-height: 64px; display: flex; align-items: center; justify-content: space-between; gap: 16px; font-weight: 500; font-size: 18px; }
  .qa summary::-webkit-details-marker { display: none; }
  .qa summary::after { content: "+"; font: 400 22px/1 var(--l-mono); color: var(--l-accent-ink); transition: transform 200ms ease; }
  .qa details[open] summary::after { transform: rotate(45deg); }
  .qa p { margin: 0 0 20px; color: var(--l-muted); max-width: 65ch; }

  .end-wrap { padding-bottom: 96px; }
  .end-in { padding: 44px; justify-content: space-between; align-items: center; flex-wrap: wrap; gap: 24px; }
  .end-in h2 { margin-bottom: 6px; }
  .end-in p { margin: 0; color: var(--l-muted); }
  .end-actions { display: flex; flex-wrap: wrap; gap: 12px; align-items: center; }

  .foot { border-top: 1px solid var(--l-line); overflow: hidden; }
  .row-foot { display: flex; flex-wrap: wrap; justify-content: space-between; gap: 12px; padding-block: 28px; font-size: 15px; color: var(--l-muted); }
  .row-foot p { margin: 0; display: inline-flex; gap: 8px; align-items: center; flex-wrap: wrap; }
  .made-tag { padding: 2px 10px; border-radius: 999px; background: var(--l-tint); color: var(--l-accent-ink); font-weight: 500; }
  /* The closing phrase: corner brackets travel from word to word (mastrilo's footer, our words). */
  .words { max-width: 1200px; margin: 0 auto; padding: 56px 24px 72px; text-align: center; font: 900 clamp(30px, 6vw, 80px)/1.15 var(--l-display); letter-spacing: -0.03em; }
  .words:lang(ar) { letter-spacing: 0; }
  .w { position: relative; display: inline-block; padding: 0 0.12em; }
  .w::before, .w::after { content: ""; position: absolute; inset: -0.04em 0; border: 2px solid var(--l-accent); opacity: 0; pointer-events: none; }
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
    .agents, .made { grid-template-columns: repeat(2, 1fr); }
    .stats { grid-template-columns: repeat(3, 1fr); }
  }
  @media (max-width: 860px) {
    nav { display: none; }
    .split { grid-template-columns: 1fr; gap: 8px; }
    .side { position: static; }
    .diagram { grid-template-columns: 1fr; }
    .legend { border-inline-end: 0; border-bottom: 1px solid var(--l-line); }
    .art.big { min-height: 0; aspect-ratio: 64 / 40; }
    .cards { grid-template-columns: 1fr 1fr; }
  }
  @media (max-width: 620px) {
    .wrap { padding: 0 16px; }
    .bar { gap: 8px; }
    .brand span { display: none; }
    .hero { padding-top: 56px; }
    .lead { font-size: 17.5px; }
    .engine { margin-top: 48px; padding: 0 10px; }
    .facts ul { grid-template-columns: 1fr 1fr; }
    .facts li { border-inline: 0 !important; padding: 14px 12px; }
    .cards, .agents, .made { grid-template-columns: 1fr; }
    .stats { grid-template-columns: 1fr 1fr; }
    .stats div { border-inline: 0 !important; border-bottom: 1px solid var(--l-line); }
    .ruled { padding-block: 72px; }
    .end-in { padding: 24px; }
    .frame-in { flex-direction: column; padding: 22px; }
  }
</style>
