# 0001. A pure kernel

Date: 2026-10-14. Status: accepted.

## Context
The design promises one Rust core on the server, in the offline web client and inside native
apps. Any I/O inside the core (a database driver, an async runtime, the system clock, a random
number generator) breaks at least one of those targets.

## Decision
The kernel is two pure functions plus readers:
`decide(actor, command, history, env, ids) -> Result<Vec<Event>, KernelError>` and
`fold(events) -> CohortState`, with `learner_view` and `studio_view` on top. Time comes in
through `Env::now`, ids through the `IdGen` trait. Stores, networks and models live in shells.
`zidara-kernel` and `zidara-frameworks` must build for wasm32, Android and iOS in CI.

## Consequences
- Async shells (axum + sqlx, IndexedDB) wrap the kernel without a storage trait.
- Every rule is testable with plain values, so tests are fast and run on wasm32 too.
- `decide` refolds the history for each command. That is O(history) per command; fine for a
  cohort's log, and the first thing to cache if measurements say so.
