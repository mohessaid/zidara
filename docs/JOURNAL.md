# The Zidara story

Newest first. Each entry: what happened, the problem, what we did, the numbers, the lesson.

---

## 2026-10-16 — We filled the owner's disk

**What happened.** A mutation run died with "No space left on device". The laptop had 98 GB
free when the session began; it had 1.7 GB. The load average was 24 on 8 cores, and the browser
tests ran eight times slower than usual.
**The problem, and it was ours.** Each mutation job builds a complete copy of the workspace in
`/tmp`, with full debug info: 2–5 GB each. Runs we had stopped or that collided left six of
those copies behind (18 GB). The project's own build cache had grown to 28 GB. This is someone's
working machine, shared with other projects. It is the same kind of mistake the owner had
already flagged on another project ("contributors have other projects on the same machine").
**What we did.**
- Stopped our own run.
- Checked with `lsof` that no process, ours or another session's, held each temporary copy,
  then deleted the orphans.
- Cleared only regenerable caches (incremental and coverage builds): 32 GB free again.
- Added a `mutants` build profile without debug info or incremental state, so each copy is
  several times smaller.
- The skill now says: check `df` before heavy runs, and never stack them.

**Lesson.** A test that leaves its scaffolding behind is a leak, even when it passes.
Clean-up is part of the work, and so is the machine it runs on.

## 2026-10-16 — Fairness: no more memorised retakes, and recovery that counts

**What happened.** Two balance problems from the backlog were fixed in the kernel, where the
rules live.
- **Retakes.** A quiz had one fixed set of questions, so a learner could retake it until the
  answers were memorised. Each practice quiz now has three versions. The kernel decides which
  version each attempt must use, rotating per learner and per activity, and refuses any other.
  A modified phone cannot keep choosing the version it knows. Phone and server compute the
  same turn, so offline work still syncs. Every new answer was checked independently (exact
  fractions) before it entered the pack.
- **Recovery.** Mastery used the mean of the last three tries, so a child who got everything
  right after remediation still read "needs support". It now uses a decaying average (65% on
  the newest try), the standards-based-grading default, recorded with sources in ADR 0007.

**Numbers.** 117 kernel and framework tests, 17 stories, 53 server tests, 21 web unit tests. A
new browser test proves a retake shows different questions. The golden session changed in one
judged value (the recovery) and four envelopes (the versions).
**Lesson.** Fairness rules belong in the kernel. In the UI they would be one phone's opinion;
in the kernel they are the same rule on every device.

## 2026-10-16 — The web shell: a lesson a child can take offline

**What happened.** G4 built the app learners and teachers use, in Arabic first. The Quest path
is drawn by PixiJS, with every stone a real button underneath for keyboards and screen
readers. There is a grid view, an activity player and a teacher studio. Everything runs on the
kernel in wasm, offline first, as an installable app.
**Numbers.** Eight browser tests on the real server and a fresh database:
- RTL;
- offline answer → the map updates with no network → it syncs when the network returns;
- the app reopens with no network at all;
- the other class sees nothing;
- an unsure answer reaches the teacher, who accepts it;
- a framework switch redraws the path.

axe finds no WCAG violations on any screen. Lighthouse mobile: 94–100 performance and 100
accessibility. On an economy-phone profile the map holds a median of 58–59 fps through a level
transition.

## 2026-10-16 — Measuring a phone with a CPU pretending to be a GPU

**What happened.** The first frame-rate gate failed badly: 22 fps, and a 436 ms freeze when
stepping back onto the map.
**What we tried first.** A guess: the map was re-created on every return, so we kept it
mounted. It got *worse* (595 ms). We stopped guessing and profiled: our code was ~100 ms;
2.4 s was the browser's own native work. Headless Chromium had no GPU and drew WebGL in
software (SwiftShader). We were measuring this laptop's CPU pretending to be a graphics chip.
**What we did.** Gave the test browser the laptop's real GPU, an Intel HD 630 (roughly a
mid-range phone's class), and kept the CPU throttled 4×. Then the data settled it:
- re-creating the map on each return costs 350–405 ms on real hardware too, so keeping it
  mounted was right after all; software rendering had hidden that cost;
- pausing the hidden map, or slowing it to 1 fps, moved a GPU upload into the first frame back
  (50–80 ms);
- leaving it running gave no long task in 8 of 9 transitions.

**Two more mistakes found on the way.** The measuring code double-counted frames (impossible
"179 fps"), because each run left its frame counter running. And one run on a busy laptop is
noise, so the gate now takes the median of three.
**Lesson.** Profile before you fix. Check that the instrument measures the thing, not itself.
A number that's too good is a bug report too.

## 2026-10-16 — From 51 to 100 by moving nothing but timing

**What happened.** Lighthouse scored performance 51: first paint 2.8 s, largest paint 6.4 s.
**What we did, one measured step at a time.**
- Split the code: PixiJS loads only on the learner's screen, not the sign-in page. Score 81.
- Painted a static splash and deferred the service-worker script. Score 87.
- Tested the obvious suspect, the Arabic web font, by removing it: still 87, so not guilty.
- The real cause: the 143 KB kernel download started during first paint, and Lighthouse counted
  everything in flight as blocking. The kernel now loads when someone signs in, or quietly two
  seconds later. **Score 100**, largest paint 1.4 s.

**Lesson.** Rule suspects out with data before fixing them. The font looked guilty and wasn't.

## 2026-10-16 — Bugs the browser tests caught that unit tests could not

- **A screen-reader name hid the visible name.** Sign-in buttons had `aria-label="Sign in as
  Amina"`, but showed "Amina 4 A". axe flagged WCAG 2.5.3: a voice-control user saying the
  visible words would not match. Now the prefix is hidden text inside the button.
- **The quiz crashed before it showed.** Answer slots were created in an effect, which runs
  after the first render, but the template bound to them during it. The button just did
  nothing. Fixed, and an error boundary now shows a friendly message instead of silence.
- **A teacher's switch could leave late.** If a sync was already running, the next one was
  skipped instead of queued. It wasn't lost (the outbox keeps it), but it waited for the
  next sync. Syncs now queue.
- **Replaying the outbox on reload would have doubled offline work.** Caught by reading the
  code before it shipped: the saved log already holds those answers.
- **A test killed itself, again.** `pgrep -f` matched its own command line. Port-based PID
  lookup now. And `kill $!` killed `npx` but not the Vite server it started; the script now
  starts Vite directly.

## 2026-10-16 — One lesson, three machines, the same answer

**What happened.** G3 put the kernel in the browser. To prove the claim at the heart of the
design ("one core, compiled several ways"), we recorded one scripted lesson on the native
kernel. It covers a quiz, two weak tries, remediation, an Arabic explanation, a perfect retry
and a framework switch. We then replayed it through the optimised wasm build in Node and
through the server's HTTP API on PostgreSQL with row-level security.
**Numbers.** Six steps; the learner's view matched on all three at every step. The bridge is
143 KB gzipped after wasm-opt: the whole kernel, both frameworks and the marking, smaller
than one photo. Changing one number in the recording makes the Node replay fail.
**Lesson.** "It's the same code" is a belief. "It gives the same answer, here and here and
here" is evidence. The golden file turns the architecture into a test.

## 2026-10-16 — The recording found a pedagogy question

**What happened.** Writing the golden session, we expected a learner who gets full marks after
two weak tries to be "in progress". The kernel said "needs support".
**The problem.** The kernel was right by the rule. Mastery averages the last three tries:
(33% + 33% + 100%) / 3 = 56%, below the 60% support line. A child who has just understood
still carries the label for two more good tries.
**What we did.** Pinned the true behaviour in the test and logged it for the G5 balance check:
perhaps the latest try should count double, or a perfect try after remediation should clear
support.
**Lesson.** Writing down what you expect is how you find out what the rules really say.

## 2026-10-16 — Two tests that could not see their bug

**What happened.** The rerun of the server's mutation tests found two survivors.
- **The Claude client.** Replacing the whole HTTP call with an empty reply went unnoticed. The
  Claude tests lived in a file whose other tests need Docker, and mutation testing skips that
  file for speed. Moved the three HTTP tests, which need no database, to their own file, which
  mutation testing now runs.
- **The key-hiding `Debug`.** The test checked that printing the keys did not show the
  secret. Printing nothing at all also passes that test. It now checks the exact output.
**Also.** Running two mutation campaigns at once in one directory crashed the second: they
share `mutants.out`. One at a time.
**Lesson.** "The secret is not shown" and "the right thing is shown" are different tests.

## 2026-10-16 — Crafting a skill on demand

**What happened.** The bridge needed wasm-bindgen and ts-rs know-how that the project skill
lacked. Girth found nothing local, fetched two public Rust/wasm skills, and took the ts-rs
facts from its documentation. The result is a 3 KB `girth-wasm` skill.
**What we learned using it.** ts-rs warns that it ignores `serde(transparent)`, but renders the
newtypes correctly anyway; and types generated by `macro_rules!` need the derive inside the
macro. Both facts are now in the skill.

## 2026-10-15 — The server goes live on a real database

**What happened.** G2: an axum server on PostgreSQL 16, running the kernel for every change.
Then the real binary was started against a Docker database and driven with curl.
**Numbers.**
- A learner's quiz is applied; her view comes back in Arabic, with F2 unlocked by mastering F1.
- The teacher's studio shows "1 of 2 achieved".
- A learner from the other cohort, and a learner trying the studio, both get 403.
- The median learner view takes 9 ms, on a debug build.
**Lesson.** The kernel did not change shape to get there; only one line in its contract did
(frameworks must be `Send + Sync` to be shared across server threads).

## 2026-10-15 — The database, not the server, decides who sees what

**What happened.** The spec had the server tell PostgreSQL which cohorts a request could
touch. A ten-minute probe proved a stronger design: the server says only *who* is asking
(`app.user_id`), and policies derive everything else from memberships.
**The problem.** A pooled connection remembers an *empty* setting after a transaction, so the
naive cast of the setting to a uuid would crash the next request. And a policy on
memberships that reads memberships recurses forever.
**What we did.** `nullif(..., '')` makes an empty setting mean "nobody". Two tiny
`security definer` functions read memberships for the policies. Learners now see only their
own events and their own name: no classmates' list, which matters for minors.
**Numbers.** 9 isolation tests. Sabotage (make the app role the table owner): 5 of the 9 fail.
**Lesson.** Put the rule where the data is. A server bug can now at worst act as the wrong
person; it cannot widen what anyone sees.

## 2026-10-15 — A race test that passed when it should have failed

**What happened.** The test for "two weak attempts at once assign remediation only once"
passed. With the lock that guarantees it deliberately removed, it still passed 2 runs out of 3.
**What we did.** Eight concurrent attempts instead of two caught it 2 runs out of 3; still not
proof. So we added a deterministic test of the mechanism itself: hold the cohort lock in
another transaction, assert that a command waits, release it, and assert that it completes.
Without the lock it fails every time.
**Lesson.** Concurrency bugs hide from tests that hope to hit them. Test the mechanism, not
the luck.

## 2026-10-15 — Mutation testing found a date that will never exist

**What happened.** The xAPI export formats timestamps with a classic days-to-date algorithm,
tested against six dates checked independently in Python. Mutation testing changed one `+`
in the century correction, and no test noticed.
**The problem.** None of the six dates fell where that correction matters: the first day after
a skipped century leap day. With the mutation, the code prints **2100-02-29**, a date the
calendar does not have.
**What we did.** Added 2100-03-01 and 2400-02-29 as test vectors. The mutant now fails.
**Lesson.** Hand-picked examples cluster around the dates we think about. Mutants go looking
where we didn't.

## 2026-10-15 — The advisor saved the Arabic answers

**What happened.** A deferred fix said: stop "same" matching "sesame" by matching rubric terms
at word starts only. Reviewing the plan, the advisor pointed out that this would break Arabic.
Roots sit *inside* conjugated words: `ضرب` (multiply) in `نضرب` (we multiply).
**What we did.** Word starts for Latin-script terms only. Arabic terms and symbols keep matching
anywhere. Then the first implementation treated `×` as a Latin letter, because it sits in the
Latin-1 code block; a test caught it.
**Lesson.** "Word" is not a universal unit. Ask the language before applying a rule.

## 2026-10-15 — Scripted edits that went wrong

**What happened.** Two scripted edits failed silently. `cargo fmt` had reflowed a function, so
a patch matched nothing. And a slice that should have held the Makefile's mutants section was
empty, because the end marker came earlier in the file. Replacing an empty string inserted
the new text between every character.
**What we did.** Restored the Makefile from git and wrote it out whole. The skill now says:
assert that the old text exists, never replace an empty string, and rewrite whole functions
instead of patching them.
**Lesson.** The cheapest edit to make is the most expensive one to trust.

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
