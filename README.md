# Zidara

A learning platform that knows how teaching works: one kernel, many pedagogies, an experience
for every audience, and AI agents that act only inside rules people approve. The core is
written in Rust and runs on the server, in the offline web client and (by design) inside native
apps.

This repository is the working prototype for the [MIT Solve 2027 Global Learning
Challenge](https://solve.mit.edu/challenges/2027-global-learning-challenge) (deadline
6 November 2026, 12:00 pm EST, per the live challenge page on 31 October).

## What works today

- **One lesson, two pedagogies.** "Equivalent fractions" for ages 9–11, under mastery learning
  (decaying average, 80%) or Cambridge-style teaching (scheme-of-work calendar, grade
  boundaries). A teacher switches the class's framework and every learner's path redraws from
  the same evidence.
- **Arabic first**, right to left, with French and English. A lesson film in all three.
- **Light on low-end phones.** With data saver on, or a phone reporting 2 GB of memory or less, the lesson opens
  without the game engine: 40% less memory and half the main-thread work (ADR 0008).
- **Offline-first.** After one visit the app opens and the lesson works with no network; work
  syncs later, and the phone and the server compute the same result.
- **AI that drafts, teachers who decide.** The assessor marks open answers (Claude, or a
  rubric). Only confident formative marks apply; summative marks are impossible for an agent.
  Every agent action can be undone.
- **Isolation in the database.** Row-level security: learners see only their own work and
  name; classes never see each other.
- **Fair practice.** Quiz versions rotate per learner, enforced by the kernel; recovery after
  remediation counts.

## The numbers (from `make gauntlet`, 18 October 2026)

| Gate | Result |
|---|---|
| Kernel and frameworks | 123 tests, also run on wasm32 |
| Coverage | 100% of lines and functions, 180/180 branches (kernel, frameworks, server, wasm core) |
| Mutation testing | 360 mutants (243 core, 117 server): every viable one caught |
| User stories | 18 scenarios in plain language (`features/`) |
| Server, on real PostgreSQL | 57 tests, including 9 isolation tests |
| Phone = server | one golden session, identical natively, in Node (wasm) and on the server |
| Web | 28 unit tests; 12 browser tests on the real server, 1 on the browser-only demo |
| Accessibility | zero WCAG violations (axe, every screen); Lighthouse accessibility 100 |
| Performance | Lighthouse mobile 100; map median 60 fps at 4× CPU throttle; kernel 164 KB gzipped |
| Load (2 CPU cores, 300 learners in 10 classes) | p95 16–32 ms reads, 33–71 ms writes across runs, at 150 views/s + 50 answers/s; capacity ~690 requests/s |

## Run it

```bash
make serve                  # PostgreSQL in Docker + the server on :8080 (demo sign-in)
cd web && pnpm install && npx vite   # the app on :5173, proxied to the server
make gauntlet               # every proof (Docker, Node, k6 and the Rust nightly needed)
```

Deploying: `docs/deploy.md`. One binary serves the API and the app. A demo build (`pnpm build:demo`) runs the whole school in the browser, as static files.

## What is here

| Path | What it is |
|---|---|
| `crates/zidara-kernel` | The six building blocks of a pedagogy, as pure functions (no I/O) |
| `crates/zidara-frameworks` | Mastery and Cambridge-style, behind one trait |
| `crates/zidara-server` | axum + PostgreSQL with row-level security; the assessor agent |
| `crates/zidara-wasm` | The kernel for the browser |
| `crates/zidara-acceptance`, `features/` | Executable user stories |
| `web/` | The offline-first app (Svelte 5, PixiJS) |
| `content/` | The lesson pack and credits |
| `docs/` | Design (`proposal.md`), spec and plans, decisions (`adr/`), `JOURNAL.md` (the story, mistakes included), `GOALS.md`, `solve-evidence.md` |
| `pitch/` | The Solve pitch video and the recorded demo |
| `tools/` | Video and load tooling |

## Status

Prototype (Solve's definition: a working version, not yet with users). Before submitting:
`docs/GOALS.md` lists what only the team can supply: a partner school, the application in
the team's own words, name checks, hosting, and a voice.
