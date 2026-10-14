# Changelog

All notable changes to Zidara are recorded here. The format follows
[Keep a Changelog 1.1.0](https://keepachangelog.com/en/1.1.0/).

## [Unreleased]

### Added

- Rust workspace and the gauntlet: `make gauntlet` runs lint, tests, acceptance stories,
  wasm32 tests, Android and iOS checks, 100% line, function and branch coverage, and mutation
  testing. A CI workflow mirrors them (unverified until the repo has a remote). Mutation
  testing uses `cargo test`, because nextest
  cannot list the cucumber binary.
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
