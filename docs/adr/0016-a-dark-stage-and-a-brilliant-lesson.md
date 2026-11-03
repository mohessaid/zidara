# 0016. A dark stage for the pitch, a light lesson for the demo, contrast measured

Date: 2026-11-03. Status: accepted. Supersedes the look in ADR 0015 (its message stands).

## Context
The owner found the dot-matrix restyle better but not there: the numbered section labels
("01 / vision") read as a pattern of AI-made sites, the colours and the font were weak, and the
demo should feel like Brilliant without overdoing it. They pointed to mastrilo.com's heading
face, colours, contrast and motion. Then: "pay attention to color contrast", which kept slipping.

## Decision
- **Front page: a dark, high-contrast stage.** Near-black ground, white headings, grey reading
  text, one saffron accent (#f7b63c), mastrilo's register but not its blue. No numbered labels
  or crosses; numbers only where there is a real sequence (the five whys).
- **Type.** mastrilo's face (Thmanyah Sans) is free to use but behind an email form and may not
  be redistributed, so it cannot live in a public repo. Of nine open Arabic and Latin faces,
  Alexandria (SIL OFL) at 800 to 900 came closest; it sets every heading. Readex Pro stays for
  reading. Only the Arabic and Latin subsets are precached.
- **Motion, small and CSS.** A console in the hero types a request and the engine answers layer
  by layer; corner brackets travel along the closing phrase; the dot drawings stay, recoloured.
  The hero's dot streams and the dotted wordmark go.
- **Demo: a light lesson in the manner of Brilliant.** Course cards with flat badges, one column
  of levels with a Start card, one idea per screen, Check pinned at the bottom, a green or
  orange panel that says why. Bizra, the oasis scene, palms and confetti are removed.
- **Contrast is measured.** `lowContrast` (tests/e2e/helpers.ts) walks every visible text and
  control edge: text 4.5:1 (3:1 large) against what is really behind it, gradients and
  translucent layers included; any drawn control edge 3:1. It runs inside every
  `expectAccessible`, and tests/static/contrast.spec.ts tours each screen; a planted-mistake
  test proves it fails when it should.

## Consequences
- Lighthouse on `/` rose from 90 to 98 to 99: two canvases fewer than the heavy hero they replaced.
- Every colour token carries its measured ratio; `--edge` (3.4:1 on white) is for anything you
  press or type into, `--line` only for separators.
- The art bible's oasis is retired from the screens; the Solve film will need new captures.
