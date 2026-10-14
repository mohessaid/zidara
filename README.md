# Zidara

A learning platform that knows how teaching works: one kernel, many pedagogies, an experience
for every audience, and AI agents that act only inside rules people approve. The core is
written in Rust and runs on the server, in the offline web client and inside the native apps.

This folder prepares an entry to the [MIT Solve 2027 Global Learning
Challenge](https://solve.mit.edu/challenges/2027-global-learning-challenge) (deadline
2 November 2026).

## What is here

| Path | What it is |
|---|---|
| `crates/` | The Rust kernel, the two frameworks and the acceptance stories (`make gauntlet`) |
| `features/` | The user stories, readable by anyone |
| `content/` | The demo lesson pack (Arabic, French, English) |
| `CHANGELOG.md`, `docs/JOURNAL.md`, `docs/adr/` | What changed, the story behind it, and why |
| `docs/proposal.md` | The design: the three axes plus automation, architecture, why Rust, standards, proof |
| `docs/solve-2027.md` | The challenge, how the design answers each focus area and criterion, gaps, next steps |
| `docs/naming.md` | Why "Zidara": David Placek's Diamond questions, sound, linguistic checks, alternatives |
| `docs/discussion-2026-10.md` | The original discussion, word for word |
| `pitch/film/` | The HyperFrames pitch film project (`npx hyperframes preview` to edit) |
| `pitch/deliverables/` | The rendered pitch: voice cut, silent cut with subtitles, 25-slide PDF |
| `pitch/archive/` | The first Manim draft and the first storyboard |

The pitch film was made for one manager, in the context of an earlier project. Re-cut it for
Solve before using it.

## Status

- Design: written.
- Prototype: the kernel (M1, 14 October) and the server (M2, 15 October) are done. Two
  frameworks, a governed agent and offline sync run on PostgreSQL with row-level security;
  `make serve` runs it locally. Next: the wasm bridge and the web shell (G3, G4). Goals and
  dates: `docs/GOALS.md`.
- Name: chosen; trademark and domain checks to do.
