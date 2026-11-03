# 0014. A front page that pitches the idea, the game at #play, and English by default

Date: 2026-11-03. Status: accepted. Amends ADR 0012 (the front door) and the prototype
spec's default interface language.

## Context
The owner asked that zidara.ta9in.com present Zidara's vision professionally, in the manner of a
pitch (the "five whys"), taking context.dev as the reference for structure, and only then lead
into the game; that the page say all of it is made with Zidara; and that English be the default.
Until now `/` opened straight into the game's front door, in Arabic.

## Decision
- **Routes.** `/` is the front page (`web/src/landing/`). `#play` is the game's front door
  (guests); `#classes` the demo classes. Any other hash is an anchor on the front page. An
  installed app opens `#play`.
- **The page.** Hero with a live proof; five whys, each figure from a verified row of
  `docs/impact-facts.md` with its source linked; how it works (six blocks, three layers, N + M,
  governed agents, phones, isolation); "Everything here is made with Zidara" with real screen
  captures; the gauntlet's numbers; questions; a last call to play. Own face (Readex Pro, Arabic
  and Latin), one accent (zellige blue), light and dark. The game keeps the oasis.
- **The live proof.** The hero's switch runs the real kernel in a Web Worker: one learner's
  same first quiz, judged under Mastery (level 1 mastered, level 2 opens) and Cambridge-style
  (top band, not an assessment; level 3 waits for the calendar). Fixed ids and dates, nothing sent.
- **English by default.** With no saved choice, the interface is English; Arabic and French are
  one choice away and remembered. The demo classes and the tests still tell the school's story
  in Arabic, which also proves right to left on every screen.

## Consequences
- A visitor reads the why before the game; a reader can check every figure at its source.
- The front page must hold the Lighthouse gate on its own: the kernel moved off the main thread
  and below-the-fold sections skip rendering until seen (journal, 3 November).
- Browser tests that tell the school's story in Arabic now choose Arabic up front (`arabic()` in
  `tests/e2e/helpers.ts`), and the guest flow starts at `#play`.
