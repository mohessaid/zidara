# Changelog

All notable changes to Zidara are recorded here. The format follows
[Keep a Changelog 1.1.0](https://keepachangelog.com/en/1.1.0/).

## [Unreleased]

### Added

- The light experience: with data saver on, or a phone reporting 2 GB of memory or less, a learner
  starts in the table and the map's game engine is not loaded until asked for (ADR 0008). On a throttled
  phone: 40% less JS memory and half the main-thread work. If the map's code cannot arrive
  (the connection dropped after sign-in), the table takes over with a message and the map
  button tries again.
- `pnpm smoke` (`web/playwright.deploy.config.ts`): checks a running deployment at `ZIDARA_URL`
  without changing its data: the API refuses visitors, a learner signs in, and the app reloads
  with no network.
- `docs/cost-model.md` (cost per learner: measured capacity plus stated assumptions),
  `docs/pilot-plan.md` (a draft for the owner), `docs/SUBMIT.md` (the owner's dated checklist),
  `docs/impact-facts.md` (public figures with sources, for the owner to check and choose from).
- One process for the whole prototype: `ZIDARA_WEB_DIR` makes the server also serve the built
  web app (unknown `/api/` paths stay JSON 404s). `docs/deploy.md`: build, database roles,
  systemd, Caddy.
- The Solve pitch video (`tools/video/pitch.mjs` → `pitch/solve/zidara-solve-pitch.mp4`, 100 s,
  silent with English on screen): problem, the recorded demo, how it works, proof from tests.
- `tools/load/`: k6 load and capacity tests against a release server pinned to 2 cores.
- The 26-second demo of the real app (`web/tests/demo`, `scripts/compose-demo.py` →
  `pitch/demo/zidara-demo.mp4`).
- The lesson film "What is a fraction?" in Arabic, French and English, drawn from code
  (`tools/video`), with a transcript as its text alternative; `content/CREDITS.md`.
- `docs/solve-evidence.md`: the facts behind each Solve question, for the owner to write from.
- Quiz versions: each practice quiz has three versions; the kernel rotates them per learner and
  activity and refuses any other (`WrongVariant`), so retakes are not memory tests. Exposed as
  `next_variant` / `nextVariant` to the phone; the web shell shows the version due.
- The web shell (`web/`): Vite, Svelte 5, TypeScript 6, an installable offline-first PWA, in
  Arabic (right to left), French and English.
  - Learners: the Quest path (PixiJS map, with a DOM button for every stone), a grid view, and
    an activity player for video, reading, quizzes (choice and fractions) and open answers.
    Stars come only from the framework's judgement band.
  - Teachers: the studio, with class progress, the agent's queue (accept or reject), alerts,
    the agent's own marks with undo, and the framework switch.
  - Offline: every command runs through the kernel in wasm first, waits in an IndexedDB outbox,
    and syncs through `/commands`; the server's log then replaces the local one. The service
    worker precaches the shell, the kernel and the fonts.
  - Proof: Vitest (kernel wrapper, store and sync, i18n, layout); eight Playwright tests on the
    real server; axe with no WCAG violations; an economy-phone frame-rate gate; Lighthouse
    mobile (`make web e2e lighthouse`).
- Server: `/api/me` returns each cohort's framework and tenant; `/api/cohorts/{id}/roster`
  gives teachers learners' names.
- `docs/art-bible.md`: the visual identity, with contrast-checked colours.
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

- `docs/deploy.md`, after a rehearsal on a fresh database: Rust from rustup (a distribution's
  `cargo` is too old), a real `ZIDARA_SECRET` for `migrate` and `seed` (a placeholder is
  refused), the owner role over TCP, and the browser smoke check.
- The docs quote the measured numbers of the final runs (they still quoted earlier ones).
- Assessments carry what the rubric found as data (`met`), so the studio shows it in the
  teacher's language; a model's free-text rationale is shown as written.
- Remediation a teacher undid is not assigned again by the rules: the teacher has the last word.
- Server load: one agent marking pass per class at a time, proposals applied in batches of 8,
  marking on the blocking pool behind a one-pass gate, and `apply_batch` for a phone's whole
  outbox (one lock and one history read). The kernel folds borrowed events (`fold_refs`).
  At 150 views/s and 50 answers/s across 300 learners on 2 cores: p95 30 ms (read), 70 ms
  (write); capacity 686 requests/s (`make load`).
- The teacher studio syncs before acting, so proposals that arrived after it was opened can be
  accepted; refusals are shown instead of ignored.
- Mastery judges on a decaying average (65% on each newest try) instead of the mean of the last
  three tries, so recovery after remediation shows (ADR 0007).
- Mutation testing builds with a slim `mutants` profile (no debug info, no incremental state).
- Rubric marking matches Latin-script terms at word starts ("same" no longer matches
  "sesame"); Arabic roots and symbols still match inside words.
- The `Framework` and `FrameworkRegistry` traits require `Send + Sync`, so a school's
  frameworks can be shared across server threads.

### Fixed

- A proposal whose mark a teacher undid now reads `undone`, not `applied` or `accepted`: evidence
  recorded from a proposal names it (`EvidenceRecorded.proposal`, omitted otherwise, so older
  logs read unchanged).
