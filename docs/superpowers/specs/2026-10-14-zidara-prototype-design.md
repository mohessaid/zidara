# Zidara prototype: design

Date: 14 October 2026. Deadline: MIT Solve application, 2 November 2026, 12:00 pm EST.
Source design: `docs/proposal.md`. Challenge mapping: `docs/solve-2027.md`.

## 1. Purpose

Solve rejects concept-stage entries. A Prototype is "an initial working version of a solution"
(often 0 users). For finalists, technical feasibility asks for "convincing evidence that [the
technology] has been built and functions as claimed". The prototype exists to make the
proposal's claims true and checkable:

| Claim in the proposal | What proves it in the prototype |
|---|---|
| Every pedagogy reduces to six building blocks | Two very different frameworks (Cambridge-style and mastery) implemented against one trait, with executable user stories for each |
| N frameworks + M experiences cost N + M | Two experiences (Quest, Grid) × two frameworks = four working combinations, no combination-specific code |
| One Rust core compiled several ways | The same kernel test suite passes natively and on `wasm32`; the kernel type-checks for Android and iOS targets |
| Works offline, low bandwidth | The learner finishes a lesson with the network off; evidence syncs later without conflicts |
| Isolation is a data rule | PostgreSQL row-level security; tests show no query, screen or agent crosses a cohort |
| AI agents are governed actors | An assessment agent proposes; policy applies or queues; every action logged and undoable |
| Proven by tests, not trusted | The gauntlet (section 4), run by one command, with numbers in the README |
| Quality on the devices people have | Frame-rate and Lighthouse gates on an economy-class device profile |

Success means: on 30 October the demo runs end to end on a laptop and a phone, `make gauntlet`
is green, and the numbers (coverage, mutants, wasm size, latency under load) are recorded.

## 2. Demo setting

Primary maths, fractions, learners aged 9–11. Arabic first (right to left), French and English.
Offline-first, with quality, on an economy-class phone (see 2.1). Fractions read the same in every language and every
curriculum, so one lesson serves the whole story.

The lesson, "Equivalent fractions", has three outcomes in a small graph:
1. `F1` Recognise a fraction as parts of a whole.
2. `F2` Find equivalent fractions (needs `F1`).
3. `F3` Compare fractions (needs `F2`).

Activities: one short video (≤ 60 s), one reading with audio, one quiz (multiple choice and
numeric), and one "explain your answer" free-text item that the assessment agent pre-marks.

### 2.1 Target devices: economy class, with quality

Most devices in use today are capable. The prototype targets the economy class and does not
trade quality away for the low end:

| Profile | Stands for | Used by |
|---|---|---|
| Economy (the target) | a 2022 mid-range Android: 4 GB RAM, WebGL2, Chrome; 4× CPU throttle and "fast 4G" in Playwright | quality gates in the gauntlet |
| Low end (future) | 2 GB RAM, no reliable WebGL, 3G | a separate "Lite" experience, not a constraint on Quest |

This is the separation the design exists for: a low-end audience gets its own experience on the
same kernel, built when it is needed. **Design rule: the experience layer's technology can be
swapped per audience; the kernel's view-intent contract is what stays fixed.** With AI-assisted
iteration, changing a client technology is a contained job, not a rewrite.

### 2.2 Gamification is the framework, rendered

Points and badges are usually bolted on. Here the game map is the framework's Progression
block and stars are its Judgement block, so rewards come only from evidence of learning, never
from clicks or time:
- under **mastery**, a level opens when the previous outcome reaches the threshold;
- under **Cambridge-style**, levels open on the scheme-of-work calendar and stars follow the
  grade boundaries.

The demo's signature moment: switch the class's framework and the same game redraws itself,
with the same content and the same evidence.

## 3. Architecture

Approach A: a shared Rust core with a thin shell per platform (the HEY model).

```
crates/
  zidara-kernel       pure Rust, no I/O: six blocks, Framework trait, events, decide/fold,
                      rules, governance, view intents
  zidara-frameworks   Cambridge-style and mastery, each behind the Framework trait
  zidara-acceptance   executable user stories (cucumber) over the kernel
  zidara-server       axum + sqlx on PostgreSQL 16, row-level security, JSON API, sync,
                      Claude assessor
  zidara-wasm         wasm-bindgen bridge; exports the kernel to the web shell
web/                  TypeScript PWA shell (Vite + Svelte 5): Quest, Grid, Teacher studio
content/              lesson pack: outcomes (CASE-shaped), activities, quiz, strings ar/fr/en
features/             executable user stories (Gherkin) run against the kernel
```

### 3.1 The kernel rule

`zidara-kernel` and `zidara-frameworks` depend on no database, async runtime, network or system
clock. The kernel is a pure core with two functions; shells do all I/O around them:

```rust
pub fn decide(actor: &Actor, command: Command, history: &[Event], env: &Env, ids: &mut dyn IdGen)
    -> Result<Vec<Event>, KernelError>;              // what happens next, or why not
pub fn fold(events: &[Event]) -> CohortState;        // what is true now
```

`Env` carries the time, tenant, cohort, lesson, frameworks and agent policy. `IdGen` is the only
trait a shell implements (UUIDv7 on server and client; a counter in tests). Stores stay in the
shells, so the async server and the IndexedDB client never need a sync storage trait. The
assessment model is not called by the kernel either: the agent is an actor that sends a
`ProposeAssessment` command, like any person.

If the database leaks into the kernel, the offline client cannot exist; CI enforces the rule by
building the kernel for `wasm32-unknown-unknown`, `aarch64-linux-android` and `aarch64-apple-ios`.

### 3.2 The six building blocks

| Block | Kernel type | Notes |
|---|---|---|
| Outcomes | `OutcomeGraph` (ids, titles per locale, prerequisites) | imported from CASE-shaped JSON; cycles rejected |
| Activities | `Activity { kind: Video \| Reading \| Quiz \| OpenResponse, outcomes }` | content is data |
| Evidence | `Evidence { learner, outcome, activity, score: Score, kind: Formative \| Summative, actor, at }` | append-only; exported as xAPI statements |
| Judgement | `Framework::judge(outcome, &[Evidence]) -> Judgement` | `Score` is an integer in basis points (0–10 000): no floats, deterministic everywhere |
| Progression | `Framework::progression(&ProgressContext) -> Vec<Unlock>` | mastery gates or calendar |
| Roles | `Role` + `Permission`; `Actor = Human(UserId) \| Agent(AgentId) \| System` | one permission check for people and agents |

```rust
pub trait Framework {
    fn id(&self) -> FrameworkId;
    fn judge(&self, outcome: &Outcome, evidence: &[Evidence]) -> Judgement;
    fn progression(&self, ctx: &ProgressContext) -> Vec<Unlock>;
    fn rules(&self) -> &[Rule];
}
```

- **Mastery:** judgement = mean of the last 3 scored attempts; mastered at ≥ 80%; progression
  opens an outcome when all its prerequisites are mastered; rule: below 60% after 2 attempts →
  assign remediation and notify the teacher.
- **Cambridge-style:** judgement = grade band from configured boundaries over all summative
  evidence (bands are data); progression opens outcomes by scheme-of-work week; rule: band
  below the expected one at a checkpoint → notify the teacher.

Thresholds, boundaries and calendars are configuration data; the logic is compiled code.

### 3.3 Events and automation

Everything that happens is an `Event` with an actor, a tenant, a cohort and a time:
`EvidenceRecorded`, `ActivityAssigned`, `TeacherNotified`, `ProposalMade`, `ProposalApplied`,
`ProposalQueued`, `ProposalDecided`, `Undone { target }`. State is a fold over events.
Undo never deletes: it appends `Undone`, and folds skip undone events.

The rules engine is a pure function `(rules, new event, state) -> Vec<Action>`. Actions become
events by the actor `System`.

### 3.4 The governed assessment agent

1. A learner submits an "explain your answer" response.
2. The agent (actor `Agent("assessor")`, role `AssessmentAgent`) calls the `Assessor` port and
   returns a proposal: score, confidence, rationale.
3. The policy decides, in kernel code:
   - formative and confidence ≥ 8 500 basis points (85%) → applied as formative evidence;
   - otherwise → queued for the teacher;
   - summative → never applied by an agent; the permission check refuses it.
4. The teacher studio lists queued proposals; the teacher accepts, edits or rejects; any applied
   agent action can be undone.

Assessors: the kernel's `assess` (deterministic keyword rubric, Arabic diacritics ignored; used
in all tests and in CI) and `ClaudeAssessor` on the server (Claude Haiku 5.5, `claude-haiku-5-5`, enabled when
an API key is set). The agent's governance is the innovation; the model is replaceable.

### 3.5 Offline and sync

- The lesson pack (content, quiz, strings, video) is cached by a service worker.
- Offline, the web shell runs commands through the wasm kernel against its local history in
  IndexedDB; judgement and the game map update locally, and each command goes to an outbox.
- Back online, the shell posts the outbox. Each command travels in an `Envelope { id, at,
  command }`. The server replays it through the same `decide`, with the same permission checks.
  A client can send commands but never events, so it cannot forge evidence. The envelope id is
  a client-made UUIDv7; the server stores applied ids, so a retry is harmless.
- `at` is when the learner acted, capped at the server's clock. Folds order events by `at`, so
  late-synced work lands where it happened. Judgement is recomputed, never merged.
  Offline edits of shared records (the hard case in the proposal) are out of scope.
- Agent pre-marking of offline answers happens at sync time on the server.

### 3.6 Server, data and isolation

- PostgreSQL 16 in Docker (Compose file with no restart policy; started only for work and tests).
- Tables: `tenants`, `cohorts`, `users`, `memberships`, `class_settings` (framework per cohort),
  `content`, `events` (append-only: no UPDATE or DELETE grants).
- Every row carries `tenant_id` and `cohort_id`. The app connects as a non-owner role with RLS
  forced. Each request sets `app.user_id`, `app.tenant_id` and `app.cohort_ids` with
  `SET LOCAL` inside its transaction; policies read them with `current_setting`.
- Sign-in for the prototype: seeded demo users and a server-signed token. OIDC is out of scope
  and named as such.
- API (data only, JSON): `GET /api/me`, `GET /api/packs/:id`, `POST /api/events` (batch sync),
  `GET /api/learner/view`, `GET /api/studio`, `POST /api/proposals/:id/decision`,
  `POST /api/events/:id/undo`, `PUT /api/cohorts/:id/framework`, `GET /api/xapi/statements`.

### 3.7 Web shell and experiences

- Vite + TypeScript + Svelte 5, installable PWA. Calls the kernel only through the wasm bridge
  and the server only through the API.
- The Quest map is drawn with PixiJS v8 (WebGL); motion with GSAP; text, RTL and accessibility
  stay in the DOM. The deciding reasons are the audience's, not download size: Arabic needs
  real shaping and bidi, which the browser's text engine gives for free and canvas engines do
  not (raylib draws glyphs without shaping; Godot and Bevy need their own shaping set up), and
  a canvas-only screen is invisible to screen readers. Raylib also needs the Emscripten target,
  a second wasm toolchain beside wasm-bindgen. Godot, Bevy and raylib stay candidates for later
  experiences or native shells; the design rule in 2.1 allows it.
- Bridge types are generated from the Rust types (`ts-rs`), so the contract cannot drift.
- The kernel returns **view intents** ("F2 in progress at 64%, F3 locked until F2 mastered"); it
  never says how to draw them. Experiences render intents:
  - **Quest** (ages 9–11): a path map, levels, stars, a mascot nudge.
  - **Grid** (older learners): a dense outcome-by-evidence mastery grid.
  - **Teacher studio**: who is stuck on which outcome, the agent queue, undo, framework switch.
  - **Lite** (future, named so the N + M list is honest): DOM only, no WebGL, for low-end
    devices. Not built for Solve.
- Design tokens; Arabic first with `dir="rtl"`; French and English. WCAG 2.2 AA is the floor.

## 4. Testing: the gauntlet

`make gauntlet` runs all of it; every part fails the build on regression.

| Part | Tool | Scope and gate |
|---|---|---|
| Unit | `cargo nextest`, Vitest | every crate; shell logic |
| Property | `proptest` | evidence order does not change judgement; sync merge is idempotent and commutative; undo-then-fold equals never-applied |
| Acceptance | `cucumber` (Gherkin in `features/`) | user stories against the kernel and in-memory store, no server |
| Integration | `testcontainers` + real PostgreSQL 16 | RLS: cross-cohort reads return nothing, even for raw SQL and the agent role; sync idempotency; append-only grants |
| End to end | Playwright | offline lesson then sync; teacher accepts a queued proposal; framework switch redraws the game; cohort isolation in the UI; RTL |
| Accessibility | `@axe-core/playwright` | zero WCAG 2.2 AA violations on every screen |
| Mutation | `cargo-mutants`; Stryker on shell logic | every mutant killed in kernel, frameworks, server domain code; exclusions listed with a reason |
| Coverage | `cargo-llvm-cov --branch` (nightly) | 100% lines and branches for kernel and frameworks; server and shell gates set from the first measurement, never lowered |
| Portability | `wasm-bindgen-test`; `cargo check --target` | the kernel suite passes on wasm32; kernel type-checks for Android and iOS |
| Quality | Playwright on the economy profile; Lighthouse | Quest map holds 60 fps with no long task over 50 ms during a level transition; Lighthouse performance ≥ 90 and accessibility = 100 |
| Load | k6 | p95 latency of `GET /api/learner/view` and `POST /api/events` on a 2-vCPU limit; recorded, not gated |

Toolchain facts proven on 14 October (throwaway probe): the same tests pass natively and on
`wasm32-unknown-unknown` under `wasm-bindgen-test-runner` 0.2.129 in Node; the kernel type-checks
for `aarch64-linux-android` and `aarch64-apple-ios`. Two traps: `uuid` with `v7` does not build
for plain wasm32 (generating ids needs a clock and randomness, so the kernel uses `uuid` as a
type only and adapters generate ids), and `proptest` needs `getrandom` with `wasm_js` on wasm32.
Acceptance stories (`cucumber`) run natively only; unit and property tests run on both.

Server choices: sqlx with runtime-checked queries (`query_as` + `FromRow`), so the code builds
without a database and the integration tests on real PostgreSQL prove every query. The app
connects as a non-owner role `zidara_app`, and every table also has `FORCE ROW LEVEL SECURITY`.

Tests are written first (TDD). Code review is not the proof; the gauntlet is.

## 5. The record

- `CHANGELOG.md`: Keep a Changelog, updated with every change.
- `docs/JOURNAL.md`: the story, newest first, one entry per notable event: what happened,
  the problem, what we did, numbers, the lesson. Mistakes go in.
- `docs/adr/`: one short decision record per real decision (kernel rule, integer scores,
  event-sourced evidence, agent policy, RLS with `SET LOCAL`).
- `AGENTS.md`: ground rules for anyone, human or agent, working in the repo.
- Commit messages are short and plain, with no AI attribution lines.
- Project skills in `.claude/skills/` guide the work: Postgres and RLS (Supabase), property-based
  and mutation testing (Trail of Bits), Rust testing, Svelte 5, PixiJS, WCAG 2.2, game UI,
  art bible, playtest and balance checks (game studio), technical video script.

## 6. Milestones

| Dates | Milestone | Done when |
|---|---|---|
| 14–15 Oct | M0 Foundations | workspace, gauntlet commands, CI file, docs skeleton, empty kernel green on every target |
| 15–18 Oct | M1 Kernel | six blocks, both frameworks, rules, governance; acceptance stories green; 100% coverage; mutants killed; wasm suite green |
| 19–22 Oct | M2 Server | schema, RLS, API, sync, Claude assessor; integration tests green |
| 22–26 Oct | M3 Shell | bridge, Quest, Grid, studio, offline, ar/fr/en; Playwright and axe green |
| 27–29 Oct | M4 Demo | lesson content and video, seed data, load numbers, demo script and recording |
| 30 Oct–1 Nov | M5 Buffer | fixes, README numbers, evidence for the application |

Each media job uses the tool that fits it best (an openly licensed clip, a generated video, a
web tool), chosen at M4; no tool is fixed in advance.

Stretch, only if M3 lands early: a third experience, a Three.js "island" for capable devices
with the Quest as fallback. It would cost one experience and no kernel change, which is the
N + M claim proven a third time.

If M1 runs more than two days late, the Grid experience and the Cambridge calendar unlocks
are cut first; the offline sync and the governed agent stay.

## 7. Out of scope

Native Kotlin and Swift apps (the kernel's Android and iOS type-checks stand in), OIDC, LTI 1.3,
QTI and SCORM import, Open Badges, offline edits of shared records, multi-node deployment,
public hosting (can be added in M4 if a live link helps the application).
