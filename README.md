# Zidara

The adaptable learning engine for the AI era: an open-source LMS core that knows how teaching
works. One tested kernel, any teaching framework, any experience for any audience, in any
language, and AI agents that act only inside rules people approve. The core is written in Rust
and runs on the server, in the offline web client and (by design) inside native apps.

- **Try it:** https://zidara.ta9in.com. Pick a course and start: no account, no names. "How
  Zidara works" teaches the idea by playing it; "Equivalent fractions" is a complete maths
  track. The real server (PostgreSQL, classes kept apart by the database, emptied every night).
- **Teachers and live rounds:** the "demo classes" link at the bottom of the front page. Open
  it as Karim on a laptop and as Amina and Yacine on two phones for a live round.
- **The same app with no server:** https://zidara.pages.dev runs the whole school inside your
  browser and works offline after one visit.
- **Code:** https://github.com/mohessaid/zidara

## What works today

- **One kernel, any framework.** Outcomes, activities, evidence, judgement, progression and
  roles are the six blocks every pedagogy we compared reduces to; a framework is rules over
  them. Mastery learning and Cambridge-style teaching ship today, behind one trait.
- **Lessons that play like a game, judged like a school.** Each outcome is a session in the
  style of Duolingo and Brilliant: one step per screen, a film, a reading, a toy to play with
  (fold a paper strip, a fraction wall), then questions answered by shading or cutting bars,
  checked at once by the kernel, with the answer and why. XP, streaks and stars are read from the
  evidence log, and the finish screen shows what the class's teaching approach now says: the
  game never claims more than the learner can do.
- **Live rounds, like Kahoot, without the pressure.** The teacher projects a board, starts a
  round on a quiz, and every phone in the class shows the question within seconds. The kernel
  marks each answer, the board counts answers and picks, and ending the round turns every
  learner's answers into practice evidence. No timer, no names, no ranking: the class plays
  together.
- **Two complete tracks.** "Equivalent fractions" (three outcomes, every question in three
  versions, two toys, a checkpoint, help assigned by the rules) and "How Zidara works", which
  teaches the concept itself to anyone, on the same kernel.
- **One kernel, two pedagogies.** "Equivalent fractions" for ages 9–11, under mastery learning
  (decaying average, 80%) or Cambridge-style teaching (scheme-of-work calendar, grade
  boundaries). A teacher switches the class's framework and every learner's path redraws from
  the same evidence.
- **Any language, right to left from the start.** Real text shaping and mirrored layouts on
  every screen; the demo ships three languages, one of them right to left.
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

## The numbers (from `make gauntlet`, 1 November 2026)

| Gate | Result |
|---|---|
| Kernel and frameworks | 139 tests, also run on wasm32 |
| Coverage | 100% of lines and functions, 228/228 branches (kernel, frameworks, server, wasm core) |
| Mutation testing | 423 mutants (308 core, 115 server): every viable one caught |
| User stories | 18 scenarios in plain language (`features/`) |
| Server, on real PostgreSQL | 58 tests, including 9 isolation tests |
| Phone = server | one golden session, identical natively, in Node (wasm) and on the server |
| Web | 36 unit tests; 13 browser tests on the real server (one a two-phone live round), 2 on the browser-only demo (one of them offline) |
| Accessibility | zero WCAG violations (axe, every screen); Lighthouse accessibility 100 |
| Performance | Lighthouse mobile 99–100; map median 60 fps at 4× CPU throttle (on a shared machine it can dip under load: `docs/JOURNAL.md`); kernel 184 KB gzipped |
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
| `docs/` | Design (`proposal.md`), spec and plans, decisions (`adr/`), `JOURNAL.md` (the story, mistakes included) |
| `pitch/` | The recorded demo |
| `tools/` | Video and load tooling |

## Status

A working prototype, open source, not yet with users. Contributions and pilots welcome.
