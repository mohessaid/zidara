---
name: girth-landing
description: Use when designing, writing or checking Zidara's public front page (zidara.ta9in.com at /), its pitch copy, sections, share metadata, or its speed and accessibility gates.
metadata:
  girth:
    sources:
      - path-or-repo: /home/mohessaid/.claude/skills/design-taste-frontend
        sections: [BRIEF INFERENCE, Layout Discipline, Content Density, AI TELLS, FINAL PRE-FLIGHT CHECK]
      - path-or-repo: /home/mohessaid/.claude/skills/seo-aeo-best-practices
        sections: [Open Graph]
    crafted_for: A professional landing page that pitches Zidara (why, how, proof) and leads into the game
    created: 2026-11-03
    last_used: 2026-11-03
---
## Read the brief first <!-- src: design-taste-frontend#0-brief-inference-read-the-room-before-anything-else -->
State one line before building: page kind, audience, vibe, aesthetic family. Zidara's front page:
a pitch for schools, education systems and builders. **Zidara is the adaptable learning engine / LMS for the
AI era (kernel, any framework, any experience, governed agents; `docs/proposal.md`). Never pitch
it as a game: the game is one experience that shows the engine bends** (owner, ADR 0015). Look
(ADR 0017): light, the demo's tokens, ink Alexandria headings, one cobalt accent (#2445e0,
7.0 both ways on white; owner's pick). **No logo, gradients or badges** (owner: AI tells).
Start from the owner's references, not from taste.

## Hard layout rules <!-- src: design-taste-frontend#4-7-layout-discipline-hard-rules-failing-any-of-these-is-shipping-broken-work -->
- Hero fits the first screen: headline ≤ 3 lines, subtext ≤ 20 words, CTAs visible. Max 4 text elements.
- Navigation on one line, ≤ 80 px. One label per intent.
- No two sections share a layout family; bento cells = content count, with varied materials.
- No numbered section labels or eyebrow kickers (owner: an AI-site pattern); numbers only for a real sequence. No split headers, no scroll cues.
- Mobile collapse declared per section; no horizontal page scroll.

## Copy <!-- src: design-taste-frontend#4-9-content-density -->
- No em or en dashes anywhere (a test enforces it). Plain sentences; no "elevate, seamless, unleash".
- Every figure comes from a "verified" row of `docs/impact-facts.md` and names its source.
- Fake-precise numbers are banned; our own numbers come from the gauntlet and the journal.
- Re-read every string in all three languages before shipping; figures and placeholders match.

## Product visuals <!-- src: design-taste-frontend#4-8-image-visual-asset-strategy -->
No div-built fake screenshots. Show the real thing: the live kernel (in a worker) on both axes,
real captures of the app. Drawings are dot-matrix scenes (`landing/scenes.ts`) that argue a point;
the canvas never sits in layout flow (absolute, out of the grid) or it widens columns.

## Speed and access
- Heavy work off the main thread (the showcase kernel runs in a Web Worker); below-the-fold
  sections use `content-visibility: auto`. Gate: Lighthouse mobile ≥ 90 performance, 100 access.
- Contrast measured, never assumed: text 4.5:1 (3:1 large), every control edge 3:1 or none;
  `lowContrast` runs in every `expectAccessible`. Write only measured ratios in comments.
- axe in light, dark and Arabic. Scrolling regions are focusable. No tracking on Arabic.
- Motion only with a reason, all of it off under reduced motion.

## Share metadata <!-- src: seo-aeo-best-practices#open-graph -->
`og:title`, `og:description`, `og:image` (1200×630, the real hero), `og:url`, `og:type`,
`twitter:card`, canonical URL, in `index.html`.

## Record
After each deliverable, run the taste context's `made` command with its defining choices.
