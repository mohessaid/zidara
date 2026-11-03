# 0013. The oasis, and a lens that shows the same work under another school's rules

Date: 2026-11-02. Status: accepted. Amends ADR 0012 (the look) and returns to the art bible's
identity.

## Context
The owner looked at the guest experience of 1 November and called it a copy of a template:
white ground, outlined cards, nothing that said who it was for or why it was different. They
were right twice over. ADR 0012 had swapped the art bible's world ("a hand-drawn path across a
sunlit landscape", warm paper) for the look of a well-known app, and the guest flow never showed
what makes Zidara different: the game is drawn from a pedagogy the engine runs, and switching
the pedagogy redraws the same learner's progress. A visitor opening the link saw a lesson app.

We also found that the chosen typeface had never rendered: an inline style on `<body>` in
`index.html` (for the splash) set `system-ui`, which beat the face set on `:root`.

## Decision
- **The oasis.** Learner screens stand on sand (`#fff4dc`), under a morning sky, with dunes,
  water and date palms drawn in SVG (`play/Scene.svelte`, `play/Palm.svelte`). Ink is the night
  (`#2a1b3d`); palm green, oasis teal, terracotta, zellige blue and date gold carry meaning. The
  tokens and their contrast ratios are in `web/src/app.css`; every text colour is at least 4.9:1
  on sand.
- **Shapes from the learners' own world.** The two courses are horseshoe-arch doorways; the
  levels are eight-pointed stars (the khatam of the region's tiles), with a darker copy below
  that they sink into when pressed.
- **The oasis grows with evidence.** One palm per outcome the framework says is achieved: on the
  path's scene, beside the level, and on the finish screen ("A palm grew in your oasis!"). It is
  never earned by taps, time or streaks (art bible: honesty).
- **The pedagogy lens.** Under the path, "School rules: Mastery | Cambridge-style". Choosing the
  framework the class does not use replays the learner's own log in a throwaway kernel under
  that framework (`lib/lens.ts`) and redraws the path from its view. Nothing is run, saved or
  sent; the class keeps its teacher's framework, and the note says so. A framework switch
  already in the log is dropped from the replay, so the lens always shows the one asked for.
- **Juice, within the rules.** A right answer: Bizra cheers, twelve small stars burst and
  vanish in 600 ms, "+10" rises. The finish: points count up, stars land one by one, a palm
  grows. All of it is off under reduced motion. Nothing loops forever on the path: the guide is
  still there, and the next level wears a fixed halo, because a loop costs every frame on an
  economy phone.
- **The font fix.** `index.html` no longer sets a font; `body` inherits the app's face.

## Consequences
- A visitor sees the concept, not only a lesson: play one level, switch the rules, and watch the
  same answers judged differently, gold under mastery, a band under Cambridge-style.
- The lens is a client-side replay of a log the learner can already see; it adds no endpoint
  and no data. It is tested in `tests/unit/lens.test.ts` and in the guest browser test.
- More is drawn on the path (a scene, palms, a trail). The frame-rate and budget tests still
  gate it; see the journal for the numbers.
- The teacher studio takes the new colours but keeps its calm layout.
