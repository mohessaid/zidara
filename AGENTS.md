# Ground rules

For anyone working in this repository, human or agent.

## The kernel rule

`crates/zidara-kernel` and `crates/zidara-frameworks` do no I/O: no database, network, async
runtime, clock or randomness. Shells pass time and ids in and store what comes out. The build
enforces it: both crates must compile for `wasm32-unknown-unknown`, `aarch64-linux-android` and
`aarch64-apple-ios` (`make wasm targets`).

## Proof, not review

- Write the test first and watch it fail.
- Nothing is done until `make gauntlet` is green: lint, unit and property tests, acceptance
  stories, wasm32 tests, the Android and iOS checks, 100% lines, functions and branches, and
  every mutant caught.
- A mutant may be excluded only when it is proven equivalent, with the reason in
  `.cargo/mutants.toml` and an entry in `docs/JOURNAL.md`.
- Scores are integers in basis points (0–10 000). No floats in judgement.

## The record

- Every behaviour change updates `CHANGELOG.md` under Unreleased.
- Every notable event goes in `docs/JOURNAL.md`, newest first, the day it happens: what
  happened, the problem, what we did, the numbers, the lesson. Mistakes go in.
- Real decisions get an ADR in `docs/adr/`.

## Commits

Short, plain, imperative subject lines (50 characters or fewer), an optional body saying why.
No prefixes, no emoji, no AI attribution lines.

## The machine

Docker runs only while someone works: no `restart:` policies, no background jobs dev does not
need. Stop the stack after a session.

## Skills

Project skills live in `.claude/skills/`: Postgres and RLS, property-based and mutation
testing, Rust testing, Svelte 5, PixiJS, WCAG 2.2, game UI, art bible, playtest and balance
checks, technical video scripts.
