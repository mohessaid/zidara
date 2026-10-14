# Goals to 2 November 2026

The Solve deadline is 2 November 2026, 12:00 pm EST. Each goal has a done-when test; a goal is
done only when that test passes and the record (changelog, journal) is updated. Tackled in order.

| # | Goal | Target | Done when | Status |
|---|---|---|---|---|
| G1 | **Kernel** (M1) | 14 Oct | `make gauntlet` green; two frameworks, governed agent, command replay | done 14 Oct (tag `m1-kernel`) |
| G2 | **Server** (M2): PostgreSQL with row-level security, `POST /api/commands`, sync, the assessor agent | 19 Oct | integration tests on real PostgreSQL prove: no cross-cohort reads (API, raw SQL, agent), learners see only their own events, the log is append-only, a replayed envelope applies once, concurrent commands stay consistent; the Claude assessor is tested against a fake API | |
| G3 | **Bridge** (M3a): the kernel in the browser via wasm, typed with generated TypeScript | 21 Oct | the same command run in Node (wasm) and on the server gives the same view; `.wasm` size measured | |
| G4 | **Web shell** (M3b): Quest (PixiJS map), Grid, Teacher studio; offline PWA; Arabic, French, English | 26 Oct | Playwright: offline lesson then sync, teacher accepts a queued answer, framework switch redraws the map, cohort isolation in the UI, RTL; axe 0 violations; 60 fps and Lighthouse ≥ 90 on the economy profile | |
| G5 | **Content and fairness**: question variants (no memorised retakes), the lesson video, audio, art | 28 Oct | balance check passes (no degenerate strategy); every asset licensed and credited | |
| G6 | **Demo**: seeded classes, load numbers, a public link if it helps, the ≤ 2 min video | 30 Oct | k6 numbers recorded; demo script rehearsed; video passes the audio-only check | |
| G7 | **Application**: Solve text from `docs/solve-2027.md`, evidence, submission | 1 Nov | every criterion answered with evidence; submitted before the deadline | |

Needs the owner (cannot be done by an agent):
- A partner or pilot setting, with a letter or quote (`docs/solve-2027.md` §Before 2 November).
- "Our advantage" in the owner's own words: communities, settings, relationships.
- Name, trademark and domain checks (`docs/naming.md`).
- The voice for the video, and the decision to publish a public demo link.

Deferred minors from the M1 review (fold into the goal that touches the code):
- Undoing a remediation assignment is re-assigned on the next weak attempt (G2: decide).
- A rejected proposal's answer can never be re-proposed (G2).
- After undoing agent evidence, its proposal still reads "applied" (G2/G4: studio view).
- The rubric matches substrings ("same" in "sesame"): use word boundaries (G2).
