# Gamification and game UI

Distilled from the game-studio skills (art bible, balance check, playtest report) and the
game-UI design skill. Generic game advice was dropped: no controllers, TVs or HUD-in-combat.

## The principle

The game is the framework, rendered. The map is the Progression block (locks come from
`OutcomeView.lock`), and stars are the Judgement block (from `judgement.band.rank`/`of`).
Rewards come only from evidence of learning: never from clicks, time spent or streaks alone.
Switching the framework redraws the same game. That is the demo's signature moment.

Motivation that lasts comes from competence, autonomy and belonging, not points.
- **Competence**: visible mastery, honest feedback.
- **Autonomy**: choice of the next open level, and of the activity type within it.
- **Belonging**: a class goal, never a leaderboard that ranks children.

## Game UI rules

- If the learner notices the UI, something is wrong. Every element earns its space.
- Animation is communication: a level unlocking, a star landing. Keep it short (≤ 400 ms) and
  skippable, with reduced motion respected.
- Design at a reference size (a 360 × 800 phone) and scale. Test on the economy profile;
  celebrate on a flagship.
- Canvas labels need an outline or a shadow for contrast on art. Real text lives in the DOM.
- A button behaves the same everywhere: the same position for "next", the same gesture
  for "back".
- Feedback after an answer is immediate and specific ("2/4 and 1/2 are equal: you multiplied
  both by 2"), never just "wrong".

## Visual identity (the art bible, short form)

Before M3 art starts, write `docs/art-bible.md` with these sections, each tied to a pillar
(clarity, warmth, honesty):
1. Identity statement, in one sentence.
2. Mood for each game state: learning, success, struggle, locked.
3. Shape language: round and friendly for learners, structured for the studio.
4. Colour system: every semantic colour explained (achieved, in progress, needs support,
   locked, the agent). Each passes contrast and is never the only signal.
5. Typography: Arabic-first type, and a size hierarchy following `accessibility.md`.
6. Iconography: one style, with a label for every icon.
7. Asset standards: spritesheet sizes, formats, the naming scheme, the 1.5 MB budget per
   lesson pack's art.

## Balance check (run before the demo)

Domain: progression. For each check, state the target, the measured value and a verdict:
HEALTHY, CONCERNS or CRITICAL.
- **Degenerate strategies.** Can a learner reach "achieved" without learning? Known risk:
  retaking the same quiz until the answers are memorised. Fix: question variants per outcome
  and a minimum spacing between attempts.
- **Dead zones.** Is there a stretch with no visible progress? Example: three weak tries under
  mastery with no new star. The "developing" band and remediation must give something to do.
- **Curve.** How many attempts does a typical learner need per outcome under each framework?
  Simulate with the kernel: property tests over random learners.
- **Notices.** Does a teacher get flooded? Measure notices per learner per week in the seeded
  demo.

## Playtest report (one per session)

```markdown
# Playtest YYYY-MM-DD
- Build: <commit>  Device: <model, Android version>  Language: ar|fr|en
- Tester: <age band, role>  Session: first time | returning  Duration: <min>
## First 5 minutes
- Understood the goal? yes/no/partly  Found the first level unaided? yes/no
- Emotional response: engaged | confused | bored | frustrated | delighted
## What worked / pain points / confusion / moments of delight
## Bugs (steps, expected, actual)
## Numbers: levels completed, stars, attempts per outcome, time per activity
## Changes to make (each with an owner and a test that will prove it)
```

No personal data about children: use age bands and session ids, never names.
