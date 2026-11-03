# 0015. Pitch the engine, not the game; a dot-matrix face for the front page

Date: 2026-11-03. Status: accepted. Amends ADR 0014 (the front page's message and look).

## Context
The first front page (ADR 0014) led with "The learning game that follows your school's rules".
The owner rejected it the same day: Zidara is an adaptable learning engine and LMS for the AI era
(one kernel, any teaching framework, any experience, AI agents inside approved rules), as
`docs/proposal.md` and the README have always said. The game is one experience built on it, a
demonstration of how far the engine bends. The owner also asked for context.dev's look,
pixel animations included, and found the previous styling weak.

## Decision
- **The message.** The headline is "The adaptable learning engine for the AI era". The page
  follows the proposal's spine: vision (five whys aimed at platforms that lock pedagogy in, the
  sourced figures kept), architecture (three layers plus automation, six building blocks,
  N + M), the live engine, AI agents (what runs today and what is designed, said apart), and
  "Made with Zidara", where the game, live rounds, the studio and the pedagogy switch appear as
  experiences. The first question answered is "Is Zidara a game? No."
- **The live engine proves both axes.** The console switches the framework (the kernel judges
  again) and the experience: a children's path, a secondary grid, a teacher's studio line and
  a text message, all drawn from one `LearnerView`. A browser test asserts every outcome keeps
  its state across the four drawings.
- **The look.** context.dev's language: a centred hero over streams of dots, the product in a
  blue dotted frame, sections ruled by thin lines with crosses, monospace labels, dot-matrix
  drawings that each argue one point, a dotted wordmark. One accent (#2450d3).
- **Pixels, hand-written.** context.dev draws its own canvases (no library ships them); Rive
  needs authored files and the shader packages are React-bound. `web/src/landing/pixels.ts` is
  one 2D canvas per drawing: sprites stamped per dot, DPR capped at 2, 30 frames a second at
  most, nothing drawn off screen, started on idle, a single still frame under reduced motion.

## Consequences
- The page's claims match the proposal; a reader meets the engine before any demo.
- Screen captures of the game wait for its restyle; the showcase uses drawings meanwhile.
