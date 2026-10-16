# Goals to 2 November 2026

The Solve deadline is 2 November 2026, 12:00 pm EST. Each goal has a done-when test; a goal is
done only when that test passes and the record (changelog, journal) is updated. Tackled in order.

| # | Goal | Target | Done when | Status |
|---|---|---|---|---|
| G1 | **Kernel** (M1) | 14 Oct | `make gauntlet` green; two frameworks, governed agent, command replay | done 14 Oct (tag `m1-kernel`) |
| G2 | **Server** (M2): PostgreSQL with row-level security, `POST /api/commands`, sync, the assessor agent | 19 Oct | integration tests on real PostgreSQL prove: no cross-cohort reads (API, raw SQL, agent), learners see only their own events, the log is append-only, a replayed envelope applies once, concurrent commands stay consistent; the Claude assessor is tested against a fake API | done 16 Oct (tag `m2-server`) |
| G3 | **Bridge** (M3a): the kernel in the browser via wasm, typed with generated TypeScript | 21 Oct | the same command run in Node (wasm) and on the server gives the same view; `.wasm` size measured | done 16 Oct (tag `g3-bridge`): golden session matches in native, Node and server; 143 KB gzipped |
| G4 | **Web shell** (M3b): Quest (PixiJS map), Grid, Teacher studio; offline PWA; Arabic, French, English | 26 Oct | Playwright: offline lesson then sync, teacher accepts a queued answer, framework switch redraws the map, cohort isolation in the UI, RTL; axe 0 violations; 60 fps and Lighthouse ≥ 90 on the economy profile | done 16 Oct (tag `g4-web`): 8 Playwright tests, axe clean, median 58–59 fps, Lighthouse 94–100 / 100 |
| G5 | **Content and fairness**: question variants (no memorised retakes), the lesson video, audio, art | 28 Oct | balance check passes (no degenerate strategy); every asset licensed and credited | done 16 Oct: quiz versions enforced by the kernel, decaying-average recovery (ADR 0007), the lesson film in 3 languages (240 KB each), credits; narration waits for a licensed voice |
| G6 | **Demo**: seeded classes, load numbers, a public link if it helps, the ≤ 2 min video | 30 Oct | k6 numbers recorded; demo script rehearsed; video passes the audio-only check | demo recorded and composed (`pitch/demo/zidara-demo.mp4`); load: 300 learners on 2 cores, p95 30/70 ms, capacity 686 req/s (`make load`). Left: public hosting (owner's decision) |
| G7 | **Application**: Solve text from `docs/solve-2027.md`, evidence, submission | 1 Nov | every criterion answered with evidence; submitted before the deadline | |

Needs the owner (cannot be done by an agent):
- A partner or pilot setting, with a letter or quote (`docs/solve-2027.md` §Before 2 November).
- "Our advantage" in the owner's own words: communities, settings, relationships.
- Name, trademark and domain checks (`docs/naming.md`).
- The voice for the video, and the decision to publish a public demo link.
- The assessor model: Claude Opus 5.5 by default (`ZIDARA_ASSESSOR_MODEL`); Claude Haiku 5.5 costs
  about 40× less per token. The kernel's policy gates either.

Known limits to carry (G5/G6):
- The lesson pack holds the answer key, so a determined learner could read it in the browser.
  Marks are still computed from responses by the kernel on the server; question variants (G5)
  reduce the value of reading it.
- The hidden Quest map keeps rendering while an activity panel is open (chosen on measurements;
  revisit if battery tests say so).
- The first level transition after start-up can take one 50–70 ms frame (shader compilation).

Balance questions for G5:
- ~~Under mastery, full marks after two weak tries still read "needs support"~~: done 16 Oct,
  decaying average (ADR 0007).
- ~~Memorised retakes~~: done 16 Oct, quiz versions rotated and enforced by the kernel.

Deferred minors found later:
- The rubric's rationale is English text ("met 1 of 2 criteria in 3 words"), even in the Arabic
  studio. It should be a structured result the studio translates.

Deferred minors from the M1 review (fold into the goal that touches the code):
- Undoing a remediation assignment is re-assigned on the next weak attempt (G4: decide with the studio UX).
- A rejected proposal's answer can never be re-proposed (G4: decide with the studio UX).
- After undoing agent evidence, its proposal still reads "applied" (G2/G4: studio view).
- ~~The rubric matches substrings ("same" in "sesame")~~: done 15 Oct (Latin word starts; Arabic unchanged).
