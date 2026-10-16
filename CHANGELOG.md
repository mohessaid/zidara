# Changelog

All notable changes to Zidara are recorded here. The format follows
[Keep a Changelog 1.1.0](https://keepachangelog.com/en/1.1.0/).

## [Unreleased]

### Added

- The kernel in the browser (`zidara-wasm`): `Core` runs the same `decide` as the server over
  a local log, with a JSON contract; `Shell` exposes it to JavaScript through wasm-bindgen.
  Event ids derive from the envelope id, and time comes from the shell, so offline sessions
  are deterministic. 446 KB, 143 KB gzipped, after wasm-opt.
- A golden session, recorded by the native kernel and replayed through the wasm build in Node
  and through the server on PostgreSQL; every learner view must match (`make bridge`,
  `cargo test -p zidara-server --test golden`).
- TypeScript types for the web shell, generated from the kernel by ts-rs (`make types`, which
  fails if the committed types are stale).
- `zidara_frameworks::demo_registry()`: one definition of the demo frameworks and their
  calendar, shared by server and phone.
- The server (`zidara-server`), an axum shell around the kernel:
  - PostgreSQL schema with row-level security forced on every table. The app role sets
    only who is asking; the database derives what they see from memberships. Learners see
    their own events and name only; teachers and the agent see their cohorts; nobody sees
    another cohort. The event log is append-only for the app.
  - One write path, `POST /api/cohorts/{id}/commands`, for online work and offline sync:
    a per-cohort lock, an idempotent envelope ledger, `decide`, and one insert, in a short
    transaction. Envelope times are clamped between the sender's latest event and now.
  - `GET /api/me`, `/api/packs/{id}`, `/api/cohorts/{id}/events` (keyset pages),
    `/learner-view`, `/studio` (teachers) and `/xapi` (xAPI statements, teachers).
  - The assessment agent marks open answers after the request returns, outside any
    transaction, and proposes through the same command path. Claude (Messages API,
    structured output, refusal fallbacks) when `ANTHROPIC_API_KEY` is set; any model failure
    falls back to the rubric with confidence capped, so a teacher reviews it.
  - Demo sign-in tokens, `zidara-server migrate | seed | serve`, `compose.yaml` and
    `make serve` for a local run; the demo school seed (two isolated cohorts).
- Rust workspace and the gauntlet: `make gauntlet` runs lint, tests, acceptance stories,
  wasm32 tests, Android and iOS checks, 100% line, function and branch coverage, and mutation
  testing. A CI workflow mirrors them (unverified until the repo has a remote). Mutation
  testing uses `cargo test`, because nextest cannot list the cucumber binary.
- `Score`: scores as integers in basis points, with fractions and means rounded half up.
- The kernel (`zidara-kernel`): outcomes as a prerequisite graph, activities and quiz marking
  (including equivalent fractions), evidence, roles and the permission table, judgement and
  progression through the `Framework` trait, rules (remediation and teacher notices), an
  append-only event log with undo, `decide` for every change and `fold` for every read,
  learner and studio view intents, and lessons loaded from validated packs.
- A governed assessment agent: deterministic rubric marking that ignores Arabic diacritics,
  a policy that applies only confident formative marks and queues the rest for a teacher, and
  undo for anything an agent did.
- Offline sync by command replay: envelopes with idempotent ids, replayed through the same
  `decide` on the server.
- Two frameworks (`zidara-frameworks`): mastery (80% over the last three tries, remediation
  below 60% after two) and Cambridge-style (calendar unlocks, grade boundaries, summative
  checkpoints).
- The demo lesson "Equivalent fractions" in Arabic, French and English.
- Seventeen executable user stories (`features/`) for both frameworks, the agent and sync.
- The project skill `.claude/skills/zidara/`, compiled from the useful parts of thirteen
  third-party skills.
- Design spec, M0–M1 implementation plan, ground rules (`AGENTS.md`), decision records and
  the project journal.

### Changed

- Rubric marking matches Latin-script terms at word starts ("same" no longer matches
  "sesame"); Arabic roots and symbols still match inside words.
- The `Framework` and `FrameworkRegistry` traits require `Send + Sync`, so a school's
  frameworks can be shared across server threads.
