# The Zidara story

Newest first. Each entry: what happened, the problem, what we did, the numbers, the lesson.

---

## 2026-11-01 — Lessons that play like a game, judged like a school

**What happened.** The owner asked us to work without stopping on lessons in the style of
Duolingo, Brilliant and Kahoot, to show what the engine can do, with at least one complete track,
possibly about the concept itself. We built: a session player (one step per screen, checked at
once by the kernel), answers given by shading and cutting bars, two toys (folding a paper strip,
a fraction wall), XP and streaks read from the log, synthesised sounds, live rounds run by the
teacher with every phone following, and a second track, "How Zidara works", that teaches the
concept to anyone. The kernel gained live-round commands, question looks and an explore kind
(ADR 0011). Every question of the fractions track, in all its versions, now has a picture, an
input and an explanation in three languages.
**The problems, in the order we made them.**
- *The same trap twice.* The kernel reads lesson packs with serde defaults (no `look` means a
  plain question; no `role` means core). The browser reads the same JSON as written. The first
  quiz question crashed the session ("Something went wrong"), and later the live board's quiz
  menu was empty. Fix: one `lookOf` helper with the kernel's defaults, and `role !==
  "remediation"` instead of `role === "core"`.
- *Svelte state is a proxy.* Saving the quiz's answers failed with "DataCloneError: could not be
  cloned": IndexedDB cannot store Svelte's state proxies. `app.run` now stores a plain snapshot
  of every command.
- *Code that is not there offline.* The session's code became its own chunk; a phone that lost
  the network before opening a lesson could not load it, and the offline browser test hung. The
  learner screen now fetches the session's code at once, in the background.
- *Two tabs are one person.* The browser demo's live round cannot be played in two tabs of one
  browser: they share one signed-in session. Live rounds are shown on the real server instead.
- *We killed our own shell* with `pkill -f "vite preview ..."`: the pattern matched the command
  running it. And a waiting loop of ours ran for an hour because its `pgrep` matched itself.
  Stop processes by PID.
- *Fading is not dimming.* Fading the wrong options on the board to 45% opacity failed the
  colour-contrast check; they now turn slate, still readable.
**The numbers.** 138 kernel and framework tests (13 new), 15 wasm, 34 web unit tests, 13
browser tests on the server (one a two-phone live round), 2 on the demo. A live round on the
hosted server: phones see it after 5.3 and 5.5 s; the board counts both answers 0.3 s after
they arrive; the next question reaches the phone in 0.8 s. The browser's kernel went from 164 to
184 KB gzipped. The demo film is 98 s.
**The lesson.** Two shells reading one JSON file need the same defaults: write them down once on
each side, next to each other, or the second shell meets them in production.

---

## 2026-11-01 — The real server, on a shared machine

**What happened.** The owner offered `s1.nasa9.com` for the full stack, with two rules: no
Docker, and never disturb the projects already on it. We built here, copied the binary and the
web app, gave Zidara its own user, port, database and roles, and added one Caddy file.
**The problem.** `seed` failed with "new row violates row-level security policy for table
tenants". Our deploy guide used the `postgres` superuser as the owner, and a superuser bypasses
row-level security. Our new owner role was not one, and the tables force row-level security on
their owner too.
**What we did.** We gave the owner role `bypassrls`. Only migrations and the seed use that role;
the app's role still bypasses nothing. The guide now has a section for shared servers.
**The numbers.** The 3 smoke tests pass against the live address over HTTPS. The other four
sites on the server answered 200 before and after Caddy reloaded.
**The lesson.** A rehearsal on a fresh database with a superuser hid a rule that a real server
with separate roles brings out. Rehearse with the roles you will really use.

---

## 2026-11-01 — Public, clean, and a link anyone can open

**What happened.** With the other project's material gone and history rewritten, we deleted the
first GitHub repository (it still held the old commits, reachable by hash) and created it again,
public. We also removed that project's framing from the design note. The demo school went up on
Cloudflare Pages.
**The problem.** `wrangler pages project create` now hands new projects to Workers and failed
with "Missing entry-point to Worker script or to assets directory", deploying nothing.
**What we did.** We followed wrangler's own notice and used `--force` for a classic Pages
project, then deployed `web/dist-demo` (57 files, 2.6 MB). The demo's two browser tests ran
against the live address and passed: the whole story with no server, and offline after one
visit.
**The numbers.** A search of every commit for that project's names finds nothing; 2/2 live tests
pass.
**The lesson.** Test the published thing, not only the build: the same Playwright story runs
against any URL with `ZIDARA_URL`.

---

## 2026-11-01 — Another project's pitch, published by mistake

**What happened.** We pushed the code to a new public GitHub repository. Within minutes the owner
saw that it held a pitch made for someone else: its slides, its film and the conversation behind
it. They came into the repo on the first day, as the record of how the design formed.
**The problem.** The secret scan before the push looked for keys and passwords. It did not ask
whose material each file was. A pitch to another organisation is confidential even though it holds
no secret.
**What we did.** We made the repository private at once (it had been public about ten minutes,
with no forks or stars). We moved the material out of the repo, kept a full backup, and rewrote
every commit with `git filter-repo`: the files are gone from all history, and the names in older
versions of three documents are replaced. A search of every commit now finds nothing. The GitHub
repository is to be deleted and created again, because GitHub keeps rewritten commits reachable
by their hashes.
**The numbers.** 78 files removed from history; `.git` went from 46 MB to 9.5 MB.
**The lesson.** Before anything goes public, list every file and ask whose it is, not only
whether it holds a secret. Keep it private until the owner has seen the list.

---

## 2026-10-31 — The gauntlet, green on a shared machine

**What happened.** The full gauntlet ran with the demo school in it, niced, while other projects
loaded the machine (load 15–18 at the end).
**The numbers.** 125 kernel and framework tests, 18 stories (161 steps), 58 server tests, 28 web
unit tests, 12 browser tests on the server and 2 on the demo build; 182/182 branches; 368 mutants
(253 core, 115 server), every viable one caught; Lighthouse 100/100; map median 60 fps. It took
about 75 minutes, most of it mutation testing.
**The lesson.** `nice` kept the run from slowing the other projects, and the frame-rate gate still
held. Coverage and mutation builds leave about 14 GB in `target/`, so clear them after each run.

---

## 2026-10-31 — Four more days, found by reading the page again

**What happened.** Before the last push, we read Solve's challenge page again. It now says
"Solution Deadline: November 6, 2026 12:00pm EST", twice on the page. On 13 October we read 2
November, and the application PDF gave both dates.
**The problem.** Treating 2 November as binding was the safe reading, but our docs stated it as
fact, and the owner's checklist was planned around it.
**What we did.** Every doc now gives 6 November with the date we read it; the checklist is
re-dated (submit 4 November, two days spare). Local `main` now holds the work. The machine was
shared with busy projects, so we cleared 32 GB of our own rebuildable build output first.
**Lesson.** A date read once is a fact about that day. Re-read the source before planning on it.

**The demo school.** With four more days, the most useful thing to build was a way for judges
to try Zidara without a server. The server does three things on top of the kernel: it runs
commands as the signed-in person, shows each person only what they may read, and lets the
agent mark answers. All three are pure rules, so the two the browser lacked (visibility and
rubric marking) moved into the kernel, and the demo uses them through the same wasm as the
phone. The app does not know: the demo answers its `fetch` with the server's paths and codes.
The real build was checked to carry none of it.
**Two time bombs and a sandbox.** The demo's test expected F3 to be locked under
Cambridge-style; F3 opened on 19 October, so the same check in the main browser suite had been
failing silently since then (no one had run it). Both now state the rule for any date. The
Lighthouse runner stopped starting Chrome: the machine's kernel was updated today and AppArmor
now blocks Chromium's sandbox, so it runs without one, as Playwright already does.
**A measurement we did not trust.** The frame-rate test first gave 48.6 fps, with the machine at
load 15 on 8 cores from other projects; the light-experience numbers in the same run had slowed
by the same factor. We waited: at load 3.5 it measured 60 fps, three runs out of three.
**Numbers.** Lighthouse 100/100 for both builds. The browser kernel grew from 150 to 164 KB
compressed (the rubric marker). 7 of 7 viable new mutants caught. 32 GB of our own build output
cleared at the start.
**Lesson.** Tests that read the clock expire. And a slow number on a shared machine is a fact
about the machine until a quiet run says otherwise.

---

## 2026-10-18 — The light experience

**What happened.** The design says low-end phones get their own experience, and until today
that was only a claim. Now a learner whose browser reports data saver, or 2 GB of memory or
less, starts in the table, and the map's game engine is never loaded unless they ask for it.
Same kernel, same views, a different screen: the separation doing what it was built for.
**The problem.** The map was imported by the learner screen, so it came with it on every
phone. Making it load on demand risked the earlier fix (the map stays mounted, because
re-creating it cost a 350–405 ms long task), so it loads once and then stays.
**What we chose not to do.** We could also have saved the bytes, by leaving the map out of
the offline bundle. But on a slow first visit the learner might sign in before the service
worker takes control, and then the map would be missing offline. Saving bytes is a later step
for a data-saver bundle; the offline promise comes first (ADR 0008).
**Proof.** A browser test with data saver on: the table shows, no canvas exists, the screen is
accessible, and one tap loads the map. It failed on the old screen and passes on the new.
**Numbers.** The same phone at 4× CPU throttle, the same screen, two runs: JS heap 3.1–3.3 MB
against 5.5–5.6 MB, main-thread work 710–723 ms against 1 542–1 628 ms, script 143–144 ms
against 517–538 ms. The map still holds 60 fps; Lighthouse is still 100 and 100.
**A mistake, caught by the gauntlet.** The first version passed all 10 browser tests, then
failed 3 in the full gauntlet an hour later. Loading the map on demand had opened a gap: the
learner screen arrives, then the map's code is fetched. If the connection drops in between
(sign in, walk out of coverage), the map never comes and the screen stays blank, with no
message. Before, the map came in one piece with the screen. The earlier pass was luck: the
test went offline a moment later. Now the map starts loading at once; if it fails, the table
takes over with a message ("the map will appear when the connection returns") and the map
button tries again. A new test blocks the map's file on purpose: it showed a blank screen on
the old code and passes on the new. The offline test now waits for the map before going
offline, so it tests offline work and not this race.
**A tool in the way.** A command wrapper on this machine compresses test output, and it
swallowed Playwright's results: the suite exited green, but we could not see which tests had
run. We now read Playwright's JSON report instead of its console.
**The gauntlet, again.** After the fix, every gate ran in order on the final code: 123 core
tests (and on wasm32), 18 stories, 57 server tests, 23 web unit tests, 12 browser tests, 100%
of lines and 180/180 branches, 360 mutants (243 core, 117 server) with none missed, Lighthouse
100/100, the map at 60 fps.
**Lesson.** An architecture claim is worth one working example. Two measured runs were enough
to say "half"; one would not have been. And splitting code adds a network request, and every
network request can fail: a test that passes once has not proven a race is gone.

---

## 2026-10-18 — Rehearsing the deploy, and the evidence a judge can check

**What happened.** We followed `docs/deploy.md` word for word on a fresh PostgreSQL 16 with the
release build, and wrote the evidence Solve's criteria still lacked: a cost model, a pilot
draft, cited impact figures and the owner's dated checklist.
**What the rehearsal caught.** Three steps of the guide failed as written.
- **The wrong `cargo`.** This shell found the distribution's cargo 1.75 before rustup's 1.98,
  and it cannot read the workspace ("`resolver` setting `3` is not valid"). The guide now
  says to use rustup's toolchain.
- **A placeholder secret.** The guide ran `migrate` with `ZIDARA_SECRET=x`, which the server
  refuses (32 bytes at least). The guide now generates the real secret first.
- **A connection we never tested.** The owner's URL used the local socket, which peer
  authentication refuses unless the command runs as the `postgres` user. The guide now uses
  the TCP form we tested.
**A trap for the owner.** The branch `g3-bridge` and the tag `g3-bridge` share a name, and git
resolves a bare `g3-bridge` to the tag, an older commit. A merge with that name would have
silently dropped two days of work. The checklist says `heads/g3-bridge`.
**The record disagreed with itself.** The evidence pack still quoted 117 tests, 53 server
tests and p95 30/70 ms from earlier runs; the README said 121, 57 and 16/33. Both were true
once. The docs now quote the final runs, and the load as a range across runs.
**What we added.** `pnpm smoke` checks any deployment without changing its data (the API
refuses visitors, a learner signs in, the app reloads offline). It passed 3 of 3 against the
rehearsal, and failed 3 of 3 once the server stopped.
**Numbers.** About 0.8 MB to make the app work offline (41 files), plus 0.24 MB for a film.
Hosting about $0.01–0.24 per learner a year; AI marking $0.06 (Haiku 5.5) to $2.25 (Opus 5.5).
**The last known bug.** After a teacher undid an AI mark, its proposal still read "applied".
The fold could not tell which mark came from which proposal, so the mark now names it. The
field is left out when empty, so every stored event and the golden session read exactly as
before. 8 of 8 new mutants caught; coverage still 100% of lines and every branch.
**Checking the researcher.** An agent gathered the impact figures; we re-read one at its
source (Arabic is 0.6% of web content, W3Techs, today) and it held. The owner checks the rest
before citing any.
**Lesson.** A guide nobody has followed is a draft. And when two names can mean two things,
the tool will pick one you did not mean.

---

## 2026-10-16 — The whole gauntlet, start to finish

**What happened.** Every gate ran in sequence on the final code, timed: lint, tests,
stories, wasm, Android and iOS, types, bridge, web, browser, Lighthouse, load, coverage and
mutation. It took about 75 minutes, and the disk was watched the whole time.
**What it caught.** Two failures, both real gaps in the tests rather than the product.
- **Two tests coupled through one learner.** The isolation test assumed Sara had done nothing,
  but the frame-rate test (which runs first) uses Sara. The isolation test now takes Sara's own
  sign-in token and checks that the server refuses class A's data with it (403), which tests
  the rule itself.
- **One line no test reached.** "Answers arrived during a marking pass, run one more" was never
  exercised. A new test holds the one-pass gate, sends a second answer, then releases it.
**Numbers.**
- 121 core tests (also on wasm32), 18 stories, 57 server tests, 21 web unit tests and 9 browser
  tests, all green.
- Coverage: 100% of lines and functions, 180/180 branches.
- Mutation: 358 mutants, every viable one caught.
- Lighthouse 100/100.
- Load: p95 16 ms reads and 33 ms writes for 300 learners on 2 cores.
- No temporary copies left behind.

**Lesson.** Run everything together, in order, sometimes. Each gate passing alone does not prove
the gates don't interfere.

## 2026-10-16 — From 39 to 686 requests a second, one measurement at a time

**What happened.** The first load test of the release server, pinned to 2 CPU cores, gave
39 requests/s and a p95 of 4.2 s. No errors, just slow. Each fix below was justified by a
measurement, and two of the measurements were themselves wrong first.
- **Separate the suspects.** Reads alone: 622/s, p95 93 ms, so writes were the problem. Writes
  with the AI agent off: 162/s. So the agent was about 4× of the cost.
- **The agent storm.** Every answer spawned a marking pass that rescanned *every* answer in the
  class (O(n²)), all fighting over the class lock. Now one pass per class runs at a time, and
  answers that arrive during it trigger exactly one more pass.
- **A fix that made it worse.** Batching all the agent's proposals into one transaction doubled
  p95: it held the class lock for the whole batch and every learner queued behind it. Batches
  of 8 kept the savings without the queue.
- **The kernel copied the whole log on every command.** `decide` cloned every event to fold
  them. It now folds borrowed events, proven equal by a property test.
- **The test was not a school.** Two learners posting 25 answers a second is not a classroom.
  The test now models 10 classes of 30 learners.
- **A gate that passed with no data.** A refactor broke the write script, and k6 marked the write
  gate ✓ with zero samples. Gates now require at least 1 000 writes and 3 000 reads.
- **A gate that measured itself.** Closed-loop readers always saturate the server, so their
  latency just reflects queue depth (Little's law). The gate is now open loop at a stated load:
  150 view loads/s and 50 answers/s. Capacity is recorded separately.
- **The tail.** With all of that, p95 was still 250–390 ms. `perf` needs root on this machine, so
  we ran an A/B instead: with the agent off, p95 fell to 30–48 ms. Marking folds a class's
  history synchronously, and on a 2-core runtime that takes over one of two request workers.
  Marking now runs on the blocking pool, one pass at a time.

**Numbers.** Open loop, 300 learners in 10 classes, agent on: read p95 **30–32 ms**, write p95
**67–71 ms**, 0 failures, and every one of 1 501 answers marked. Capacity: **686 requests/s** on
2 cores.
**Lesson.** Before trusting a number, ask what it measures, how much data is behind it, and
whether the test resembles the world. Three of our wrong turns were in the instrument, not the
code.

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
