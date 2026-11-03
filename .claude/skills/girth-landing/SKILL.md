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
a pitch for judges, funders and schools; calm, exact, "infrastructure" language (context.dev's
structure, not its look); the game keeps the oasis.

## Hard layout rules <!-- src: design-taste-frontend#4-7-layout-discipline-hard-rules-failing-any-of-these-is-shipping-broken-work -->
- Hero fits the first screen: headline ≤ 3 lines, subtext ≤ 20 words, CTAs visible. Max 4 text elements.
- Navigation on one line, ≤ 80 px. One label per intent ("Play a lesson" everywhere).
- No two sections share a layout family; bento cells = content count, with varied materials.
- Max one small uppercase eyebrow per three sections. No split headers, no scroll cues.
- Mobile collapse declared per section; no horizontal page scroll.

## Copy <!-- src: design-taste-frontend#4-9-content-density -->
- No em or en dashes anywhere (a test enforces it). Plain sentences; no "elevate, seamless, unleash".
- Every figure comes from a "verified" row of `docs/impact-facts.md` and names its source.
- Fake-precise numbers are banned; our own numbers come from the gauntlet and the journal.
- Re-read every string in all three languages before shipping; figures and placeholders match.

## Product visuals <!-- src: design-taste-frontend#4-8-image-visual-asset-strategy -->
No div-built fake screenshots. Show the real thing: the live kernel (in a worker), real screen
captures of the app (webp, lazy), real components (Scene).

## Speed and access
- Heavy work off the main thread (the showcase kernel runs in a Web Worker); below-the-fold
  sections use `content-visibility: auto`. Gate: Lighthouse mobile ≥ 90 performance, 100 access.
- Dark mode by tokens; axe in light, dark and Arabic. Scrolling regions are focusable.
- Motion only with a reason, all of it off under reduced motion.

## Share metadata <!-- src: seo-aeo-best-practices#open-graph -->
`og:title`, `og:description`, `og:image` (1200×630, the real hero), `og:url`, `og:type`,
`twitter:card`, canonical URL, in `index.html`.

## Record
After each deliverable, run the taste context's `made` command with its defining choices.
