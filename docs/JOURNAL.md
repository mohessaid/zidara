# The Zidara story

Newest first. Each entry: what happened, the problem, what we did, the numbers, the lesson.

---

## 2026-10-14 — M1: the kernel is proven

**What happened.** The kernel and both frameworks went green on every gate, on the day the
plan was written.
**Numbers.**
- 109 tests (unit and property), the same 109 passing natively and on wasm32 in Node.
- 17 user stories, 151 steps.
- 100% of lines and functions covered; 118/118 branches.
- 190 mutants: 150 caught, 40 unviable, 0 missed.
- Android and iOS type-checks pass.
- Code: 2,043 lines of source, 3,206 lines of tests, 154 lines of stories.
- Gauntlet time: lint 0 s, tests 4 s, stories under 1 s, wasm 31 s, coverage 62 s, mutants 13 min.
**Lesson.** The pure decide/fold core made the gates cheap: no mocks, no database, no clock.
Every rule is a function of plain values, so 100% branch coverage and zero surviving mutants
took tests, not heroics.

## 2026-10-14 — The game found a cheat before any child did

**What happened.** While distilling a game studio's balance-check skill, its first question
was "can a player reach the goal without playing?". Applied to us: can a learner reach
"achieved" without learning?
**The problem.** Yes. The demo lesson has one fixed quiz per outcome. A child can retake it
until the answers are memorised, and mastery is the mean of the last three tries.
**What we did.** Recorded it as a known degenerate strategy in the project skill. The fix
(question variants per outcome, and a minimum spacing between attempts) goes into the M3/M4
content work, with a balance check before the demo.
**Lesson.** Gamification and assessment share one weakness: whatever you reward gets gamed.
Ask the designer's question of the pedagogy.

## 2026-10-14 — One skill, not thirteen

**What happened.** Thirteen third-party skills had been installed for the project: Postgres,
testing, Svelte, PixiJS, accessibility, game design and video. The owner set a rule: take what
helps, ditch the rest, and compile it into the project's own skill.
**What we did.** Read every one and kept only rules that apply here, with our numbers (48 px
targets for children, 8 500 bp agent threshold, `SET LOCAL` for RLS, never call the model
inside a transaction). Then deleted the originals.
**Numbers.** 63,862 words → 3,871 words (−94%), in one `SKILL.md` and six reference files.
**Lesson.** Generic guidance costs attention every time it loads. Distilled guidance is cheaper,
and it is also better, because it already knows the project.

## 2026-10-14 — Four small traps in one afternoon

**What happened.** Executing M1 surfaced four tooling traps, none in our code.
- **The branch gate lied.** It reported 11/12 branches in `lesson.rs` while every branch
  record was hit. `Lesson::new` takes `impl Into<String>`, so it is compiled twice, and LLVM's
  totals count each copy separately. The gate now reads the merged per-line records.
- **The sabotage that didn't land.** To prove the stories could fail, we changed the agent's
  threshold with `sed`. All 17 stories still passed, which was suspicious. `cargo fmt` had
  split the line, so `sed` had matched nothing. Redone properly: 3 stories failed, the three
  that depend on the agent applying a mark.
- **nextest vs cucumber.** Mutation testing ran through nextest, which cannot list cucumber's
  custom test binary; every mutant "failed" with exit code 104. Mutants now use `cargo test`,
  which also lets the stories kill mutants.
- **Two output traps.** The token-saving `rtk` hook truncated `make` output (we counted 23
  wasm tests where 109 had run). And `pkill -f "cargo mutants"` killed our own shell, whose
  command line contained the pattern.
**Lesson.** When a gate says green and you expected red, the gate is the first suspect.
Every one of these was caught by asking "is this number plausible?".

## 2026-10-14 — The first gate held

**What happened.** M0 went green: workspace, `Score`, and every gauntlet step wired.
**Numbers.** 9 tests, passing natively and on wasm32 in Node; Android and iOS type-checks
pass; 8/8 branches covered; 27 mutants, 23 caught, 4 unviable, 0 missed.
**Lesson.** Wire every gate while the code is one file. Adding a gate to a finished kernel
means arguing with code that was never shaped to pass it.

## 2026-10-14 — Commands, not events

**What happened.** While planning, the kernel became a pure core: `decide(actor, command,
history) → events` and `fold(events) → state`.
**The problem.** The first sketch had a storage trait in the kernel. The server's database is
async and the browser's IndexedDB is async; a sync trait fits neither. And syncing *events*
from a phone means trusting the phone: a modified client could send evidence it never earned.
**What we did.** Stores moved to the shells. Offline, the phone queues *commands*; back online,
the server replays them through the same `decide`, with the same permission checks.
**Lesson.** A client that can only ask cannot forge. And "the phone and the server agree"
became a test we can run, not a hope.

## 2026-10-14 — Why not raylib?

**What happened.** The owner asked why the game map would use PixiJS and not raylib.
**What we did.** Checked rather than answered from habit. Raylib is small and fast, but it
draws glyphs one by one, without shaping or right-to-left ordering, so Arabic letters would not
join; a canvas is invisible to screen readers; and its web build needs the Emscripten target, a
second wasm toolchain beside wasm-bindgen.
**Lesson.** The deciding reasons were the audience's (Arabic, accessibility), not the
engine's speed. Raylib stays a candidate for a native shell later.

## 2026-10-14 — Economy class, with quality

**What happened.** The first spec rejected game engines on download size and framed the
target as low-end phones.
**The problem.** The owner corrected it: most devices in use today are capable. Target the
economy class with quality; if low-end users need something, build an experience for them.
**What we did.** The spec now has a device profile (a 2022 mid-range Android, 4 GB RAM, WebGL2)
and quality gates (60 fps, Lighthouse ≥ 90). Low end is a named future "Lite" experience.
**Lesson.** That correction cost an afternoon, not a rewrite, because experiences sit on a
fixed kernel contract. The separation paid for itself before any code existed.

## 2026-10-14 — The toolchain enforced the kernel rule

**What happened.** A throwaway probe built a tiny kernel for four targets on day one.
**The problem.** `uuid` with the `v7` feature failed to build for `wasm32-unknown-unknown`:
making an id needs a clock and randomness, which plain wasm32 does not have. Then `proptest`
failed too: it pulls `getrandom`, which needs the `wasm_js` backend in a browser or Node.
**What we did.** The kernel uses `uuid` as a type only; shells generate ids through an `IdGen`
trait. Test-only `getrandom` gets `wasm_js`.
**Numbers.** After the fix, the same 2 probe tests passed natively and on wasm32 in Node; the
Android and iOS type-checks passed.
**Lesson.** Build for the hardest target on day one. It found an architecture leak for free.

## 2026-10-14 — Day zero

**What happened.** The repo was created around the design written on 10–11 October. Scope
for MIT Solve (deadline 2 November 2026, 12:00 pm EST): the prototype the challenge requires,
plus one governed AI agent. Setting: primary maths, equivalent fractions, ages 9–11, Arabic
first with French and English, and gamification that is the framework itself (the map is the
Progression block, stars are Judgement).
**Numbers.** 19 days to the deadline. 13 project skills installed.
**Lesson.** Solve asks for "convincing evidence it functions as claimed". The test gauntlet is
that evidence, so it is built first, not last.
