# Zidara art bible

Pillars: **clarity** (a child knows what to do next without reading), **warmth** (the screen
is a friendly place to try and fail), **honesty** (the game shows what the learner can do,
never more).

## 1. Identity

A hand-drawn path across a sunlit landscape, where every stone you cross is something you
can now do.

## 2. Mood per state

| State | Mood | On screen |
|---|---|---|
| Learning (in progress) | curious, calm | the stone glows softly; the path ahead is drawn in dashes |
| Success (achieved) | proud, short-lived | the stone fills with colour; a star lands (≤ 400 ms), then stillness |
| Struggle (needs support) | supported, never failed | a guide stands by the stone holding a lantern: "let's try another way" |
| Locked | anticipation, not exclusion | the stone is outlined with a small padlock and a reason ("after F1", "next week") |

No red crosses, no losing sounds, no countdowns. Mistakes are a turn in the path, not a wall.

## 3. Shape language

- Learner screens: round, soft corners (radius 16 px), pebble shapes, thick friendly strokes
  (3 px).
- Teacher studio: structured grids, 8 px radius, thin rules (1 px). Calm and dense.

## 4. Colour system

Background `#FFF8EC` (warm paper); surfaces `#FFFFFF`; ink `#1F2A44`. Contrast ratios are
computed against the background and against white.

| Meaning | Colour | On paper | Use |
|---|---|---|---|
| Ink (all text) | `#1F2A44` | 13.5 | text, strokes |
| Achieved | `#1E7B4F` (green) | 5.0 | filled stone, check icon, label |
| In progress | `#1F5FAD` (blue) | 6.0 | glowing outline, progress label |
| Needs support | `#B23B2E` (clay) | 5.6 | lantern icon, label; never alone |
| Locked | `#5B6475` (slate) | 5.6 | padlock, outline, reason text |
| The agent | `#6B3FA0` (violet) | 7.0 | anything an AI did or proposed, with a robot badge |
| Star | `#B07A00` (gold) | 3.5 | **graphics only**, never text |
| Focus ring | `#0B57D0` | 6.0 | 2 px outline + 2 px offset on every focusable element |

Each state is colour **and** shape **and** words: achieved = filled + check + "done";
locked = outline + padlock + the reason. White text sits on the green, blue, clay and violet
(5.3–7.4).

## 5. Typography

- Noto Sans Arabic for Arabic, and its Latin glyphs for French and English, so the two
  scripts share one rhythm. Weights 400 and 700 only.
- Learner sizes: body 18 px, labels 16 px (minimum), key game info 24 px, titles 28 px.
  Studio: body 16 px, table 15 px, minimum 14 px.
- Line height 1.6 for Arabic, which needs room for the dots and marks above and below.

## 6. Iconography

One style: rounded line icons, 3 px stroke, 24 px grid, ink coloured unless the icon carries
a state. Every icon has a text label or an accessible name. Set: check, padlock, lantern,
star, play, book, pencil, robot (agent), undo, switch.

## 7. Asset standards

- Vector first: map stones, paths and icons are drawn by code (PixiJS `Graphics`) or SVG.
- Raster art (the landscape): WebP, one spritesheet per lesson, at most 2048 × 2048, with
  names like `lesson-<id>/<thing>-<variant>.webp`.
- Budget: **1.5 MB of art per lesson pack**, video excluded.
- Every asset has a source and a licence in `content/CREDITS.md`.
