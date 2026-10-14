---
name: zidara
description: The Zidara project's own skill: how to build, test, design and record work in this repository. Use for ANY work in the Zidara repo - kernel and frameworks (Rust), the server (axum, sqlx, PostgreSQL row-level security), the web shell (Svelte 5, PixiJS, PWA, Arabic RTL), accessibility, gamification and game UI, playtests, the Solve pitch video and lesson videos, and the project record (changelog, journal, ADRs).
---

# Zidara

A learning platform with one kernel, many pedagogies (frameworks), an experience for every
audience, and AI agents that act only inside rules people approve. The prototype is for the
MIT Solve 2027 Global Learning Challenge (deadline 2 November 2026, 12:00 pm EST).

Read first: `docs/superpowers/specs/2026-10-14-zidara-prototype-design.md` (what and why) and
`AGENTS.md` (ground rules). This skill is how.

## Rules that always apply

1. **The kernel rule.** `zidara-kernel` and `zidara-frameworks` do no I/O, use no clock and no
   randomness. `decide(actor, command, history, env, ids) -> events` changes things;
   `fold(events) -> state` and the views read them. Shells (server, web, native) do all I/O.
   CI proves it by building for wasm32, Android and iOS.
2. **Proof, not review.** Test first and watch it fail. Done means `make gauntlet` is green.
   See `references/testing.md`.
3. **Integers for judgement.** `Score` is basis points, 0–10 000. No floats.
4. **Agents are actors.** They go through the permission table like people; the kernel never
   calls a model; the policy queues anything unsure or summative for a teacher.
5. **Clients send commands, never events.** Offline work replays on the server through the
   same `decide`.
6. **Quality on economy-class devices.** Target a 2022 mid-range Android (4 GB RAM, WebGL2).
   Low-end gets its own experience later; never cap the main experience for it.
7. **Experiences are swappable, the view-intent contract is not.** The kernel says what is
   true ("F2 in progress at 64%, F3 locked"); experiences decide how to draw it.
8. **Arabic first.** Right-to-left, real text shaping, French and English beside it.
9. **The record is part of the work.** See "The record" below.
10. **Use the best tool for each job**, chosen on evidence; HyperFrames is one option, not the
    default. When a third-party skill helps, distil what applies into this skill and remove
    the original.

## Where to look

| Working on | Read |
|---|---|
| Any Rust code, tests, the gauntlet, coverage, mutants, stories | `references/testing.md` |
| The server: schema, RLS, roles, sync, the model call | `references/postgres.md` |
| The web shell: Svelte 5, PixiJS map, PWA, offline, RTL | `references/web-shell.md` |
| Any screen: WCAG 2.2 AA, contrast, focus, targets, motion | `references/accessibility.md` |
| Gamification, game UI, visual identity, balance, playtests | `references/game-design.md` |
| The Solve pitch video or a lesson video | `references/video.md` |

## The record

- `CHANGELOG.md`: Keep a Changelog; every behaviour change, under Unreleased.
- `docs/JOURNAL.md`: newest first, the day it happens. Each entry: what happened, the problem,
  what we did, numbers, the lesson. Mistakes go in; they are the useful part of the story.
- `docs/adr/NNNN-*.md`: Context / Decision / Consequences, under 30 lines, for real decisions.
- Commits: short plain imperative subject (≤ 50 chars), optional body saying why. No prefixes,
  no emoji, no AI attribution lines.
- Plans and specs: `docs/superpowers/specs/`, `docs/superpowers/plans/`.

## Commands

```bash
make gauntlet      # everything below
make lint          # fmt --check + clippy -D warnings
make test          # nextest, all crates but acceptance
make acceptance    # cucumber stories in features/
make wasm          # kernel + frameworks tests on wasm32 (Node)
make targets       # Android and iOS type-checks
make coverage      # nightly llvm-cov: 100% lines/functions + scripts/branch-gate.sh
make mutants       # cargo-mutants, cargo test tool, 4 jobs: 0 missed
```

Tooling traps already met (details in `docs/JOURNAL.md`):
- The `rtk` hook rewrites and truncates `make`/`cargo` output. Use `rtk proxy <cmd>` when you
  count results.
- `pkill -f <pattern>` can match your own shell command line. Use `pgrep -fa` first and kill by PID.
- `cargo fmt` reflows lines. A `sed` written against the old text silently matches nothing,
  so check that the edit landed before trusting what follows.
