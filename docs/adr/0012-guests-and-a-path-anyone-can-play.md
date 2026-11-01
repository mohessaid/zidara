# 0012. Guests start in one tap; the path is plain HTML, not a game engine

Date: 2026-11-01. Status: accepted. Amends ADR 0010 (the nightly reset) and the art bible
(palette, type, the path).

## Context
The owner opened the published app and saw a list of invented pupils, then, after picking one,
a blank screen where the map should be. Headless Chromium reproduced it: the PixiJS map drew
nothing in the first seconds, and on any machine without a working WebGL path it never would.
What they asked for: no list of pupils, let visitors start as guests and take lessons, and make
it look like the products people know (Duolingo, Brilliant, Kahoot).

## Decision
- **Guests.** `POST /api/guest` creates a person marked `guest` who is a learner in every
  *open* class (`cohorts.open`), and returns a token. The app role still cannot write users or
  memberships: only `private.join_as_guest(max)`, a `security definer` function, can, only into
  open classes, and only while fewer than `GUEST_LIMIT` (2 000) guests exist. It needs demo
  mode, like the demo sign-in. Two classes are open: "Discover Zidara" and "Open fractions".
  The browser demo does the same in local storage.
- **The nightly reset** is `private.reset_demo()`, run by the owner: events go, then guests'
  memberships, then guests. The seeded people and classes stay. The app role cannot run it.
- **The front door** shows a guide, one sentence of promise, and the two courses as big cards;
  a tap starts a guest in that course. The demo classes (named people, teachers, live rounds)
  stay one small link away (`#classes`), because live rounds need a teacher.
- **The path is HTML and CSS.** A unit banner, round levels winding down the screen, the next
  level glowing with a START bubble, gold when mastered, grey and locked until its rule opens
  it, and a trophy at the end. PixiJS, GSAP and the table view are removed: the path is light
  enough for every phone, so the separate light experience is no longer needed.
- **A guide.** Bizra (بذرة, "seed"), drawn in SVG: it speaks the readings in a bubble, cheers
  mastery and encourages after a miss. It is never sad (art bible).
- **Look.** White ground, chunky buttons with a solid edge they sink into, one rounded face for
  Arabic and Latin (Baloo Bhaijaan 2, SIL OFL), bright colours kept to WCAG AA: white on every
  button colour is at least 3:1 at the 19 px bold buttons use, and every colour used for text
  is at least 4.5:1 on white.
- A guest has two courses; the menu switches between them, and the session remembers the last
  one open.

## Consequences
- Anyone can try the product in one tap, on the real server, with their work kept apart from
  everyone else's by the same row-level security as a pupil's.
- An open endpoint writes rows: the cap and the nightly reset bound it. A guest token lasts
  12 hours; after that a returning guest starts again as a new guest. Acceptable for a sandbox.
- The browser bundle lost PixiJS and GSAP. The frame-rate test now measures the path, and no
  longer needs a GPU to be honest.
- Teachers' screens kept their layout; they take the new colours and type.
