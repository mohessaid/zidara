---
name: girth-gameui
description: Use when designing or restyling Zidara's learner screens (front door, path, session, finish), adding motion, celebrations or character animation, or critiquing whether the UI looks templated.
metadata:
  girth:
    sources:
      - path-or-repo: /home/mohessaid/.claude/plugins/cache/claude-plugins-official/frontend-design/1.1.1/skills/frontend-design
        sections: [Ground your designs in the subject matter, Design principles, "Process: plan, review against the brief, build, critique", Restraint and self-critique]
      - path-or-repo: /home/mohessaid/.claude/skills/design-taste-frontend
        sections: [0.D Anti-Default Discipline, 4.5 Interactive UI States, 6.B Reduced Motion (mandatory), Micro-Interactions & Effects]
    crafted_for: Game-like learning UI for kids on economy phones, vivid, juicy, not templated
    created: 2026-11-02
    last_used: 2026-11-03
---
## Ground it in the subject <!-- src: frontend-design#ground-your-designs-in-the-subject-matter -->
The subject's world, materials and vernacular are where distinctive choices come from: a toy for
8–11-year-olds looks nothing like a dashboard. Open with the most characteristic thing in that
world (an image, an animation, an interactive moment), not a headline over cards.

## The house look (ADR 0016)
The learner screens are a lesson in the manner of Brilliant, owner's call, "don't overdo it":
white ground, ink, Alexandria headings, one saffron accent, one idea per screen, chunky answer
cards with a firm lip, Check pinned at the bottom, a green or orange panel that says why. No
mascot or scenery. Every colour measured: text 4.5:1, control edges (`--edge`) 3:1.

## Tells of a generated design <!-- src: frontend-design#process--plan--review-against-the-brief--build--critique, design-taste-frontend#0-d-anti-default-discipline -->
Never spend free choices on defaults: the SaaS-card kit (identical rounded cards, one radius,
the same soft grey shadow), white page + cards, all-caps eyebrows, meta joined with " · ",
"→" on buttons, any gradient, badges and pill chips, glassmorphism, infinite micro-loops
everywhere, Inter. A disabled button keeps a 3:1 dashed edge; every bar track a 3:1 edge.
Plan first (4–6 named hex colours, type roles, ASCII layout, principles), then review the plan:
any part you would produce for any similar brief gets revised before code.

## Restraint <!-- src: frontend-design#restraint-and-self-critique, frontend-design#design-principles -->
Spend boldness in one place; keep the rest quiet. One orchestrated moment beats scattered
fade-ups. Motion that answers the person's action (confirm, unlock, reveal) is welcome when it
shows what changed. Screenshot and critique as you build; remove one accessory.

## States and touch <!-- src: design-taste-frontend#4-5-interactive-ui-states -->
Build every state: loading (skeleton in the final shape, no spinner), empty, error, success.
`:active` pushes physically (translate 1–4 px or scale .98). Every button's text passes AA
against its own fill; primary labels fit one line; one label per intent on a screen.

## Reduced motion (mandatory) <!-- src: design-taste-frontend#6-b-reduced-motion--mandatory- -->
Any motion beyond a fade MUST honour `prefers-reduced-motion`: gate CSS animations behind
`@media (prefers-reduced-motion: no-preference)` or disable them under `reduce`. Infinite loops,
parallax and physics collapse to static.

## Juice that fits Zidara
- Celebrate evidence only (a kernel verdict), never taps; ≤ 400 ms then stillness (art bible).
- Prefer CSS transforms/opacity and SVG; Svelte `svelte/motion` springs for numbers and
  meters. No canvas engine: economy phones, screen readers.
- Confetti/particles: a fixed small count of DOM/SVG nodes, removed after the burst.

## Record
After each deliverable, run the taste context's `made` command with its defining choices.
