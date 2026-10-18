# Solve application: the evidence pack

Notes for the Team Lead to write the application **in their own words**, in English. Solve
advises against AI-written applications ("impersonal or generic … can strongly disadvantage"),
and the applicant must confirm nothing is "fabricated, exaggerated, or otherwise misrepresented".
So this file holds facts, numbers and where each is proven. It is not text to paste.

Source of the questions: Solve's "2027 Global Challenges: Application Questions" PDF.
**Deadline:** the PDF says both 6 November (instructions) and 2 November (page 3); the challenge
page says 2 November 2026, 12:00 noon ET. Treat **2 November** as binding. Confirm it on the
live form when submitting.

## Facts that hold today (each one checkable in this repository)

| Claim | Evidence |
|---|---|
| Two pedagogies on one kernel: mastery (decaying average, 80%) and Cambridge-style (calendar, grade boundaries) | `crates/zidara-frameworks`; 18 executable user stories in `features/` |
| Switching a class's pedagogy redraws the same learner's progress, with no data lost | story `framework_switch.feature`; browser test "switching the framework redraws…" |
| Works fully offline after one visit; offline work syncs later, and phone and server compute the same result | browser tests "offline work is kept…" and "after one visit, the app opens with no network"; golden session replayed natively, in Node (wasm) and on the server |
| Arabic first (right to left), French and English | `web/src/lib/i18n/strings.ts` (every key in three languages, tested); RTL browser test |
| Accessible | axe: zero WCAG 2.x A/AA violations on every screen; Lighthouse accessibility 100 |
| Fast on an economy phone | Lighthouse mobile performance 100; map median 60 fps with no long task at 4× CPU throttle |
| Light on low-end phones | with data saver on or a phone reporting ≤ 2 GB of memory, the lesson opens without the game engine: JS heap 3.1–3.3 MB vs 5.5–5.6 MB, main-thread work 710–723 ms vs 1 542–1 628 ms on the same throttled phone (browser test; ADR 0008) |
| Small | kernel in the browser 150 KB gzipped; first page 28 KB gzipped |
| Runs on a small server | 300 learners in 10 classes on 2 CPU cores: p95 16–32 ms reads, 33–71 ms writes across runs; capacity about 690 requests/s (`make load`, `tools/load`) |
| AI that augments teachers, not replaces them | the assessor agent proposes; only confident formative marks apply; summative marks are impossible for an agent (permission table); teachers accept, reject or undo (ADR 0004; browser test "an unsure answer waits for the teacher") |
| Privacy by design for minors | PostgreSQL row-level security: learners see only their own work and name; classes never see each other (9 isolation tests; ADR 0006) |
| Fair practice | quiz versions rotate per learner, enforced by the kernel; recovery after remediation counts (ADR 0007) |
| Proven by tests, not trust | 123 kernel/framework tests (also run as wasm), 18 user stories, 57 server tests on real PostgreSQL, 23 web unit tests, 12 browser tests; 100% of lines and 180/180 branches; 358 mutants, every viable one caught |

## Question by question

**Name (≤30 chars).** "Zidara" (name checks still to do, `docs/naming.md`).

**One-line summary (≤150 chars).** Say what it is for, not how it is built: one learning app
whose teaching method (mastery, Cambridge…) is a setting, that works offline in Arabic, with an
AI that drafts and teachers decide.

**Dimension.** The best fit is "Work in emergency or low-resource settings … offline". It also
covers "multilingual" and "Improve assessment … flag when a learner is off-track so a teacher
can intervene" (remediation and teacher alerts are built in). Choose one; mention the others.

**What specific problem? (200 words).** *Owner:* local statistics for the communities served:
learners, connectivity, languages, and the teacher-to-learner ratio. Facts from this project:
general platforms (Moodle: 400M+ users) carry every pedagogy and cannot be cut down to one
school's; Arabic is often an afterthought; the teaching method is not something the software
understands.

**What is your solution? (200 words).** The kernel/framework/experience separation; the demo
lesson; offline-first; the governed agent. Link the demo video (`pitch/demo/zidara-demo.mp4`, 26 s) or the pitch (`pitch/solve/zidara-solve-pitch.mp4`, 100 s).

**Who does it serve? (200 words).** *Owner:* the learners and teachers you know: country, ages,
schools, connection conditions, languages. The prototype targets 9–11-year-olds learning
fractions in Arabic, French and English on mid-range Android phones.

**Stage.** **Prototype**: "an initial working version … (often 0 users)". Do not claim Pilot:
there are no real users yet.

**Why that stage (200 words): what you built and tested.** Use the facts table. Name the
numbers; link the repository or the demo.

**Beneficiaries served now.** 0 (be honest; Solve says it doesn't change your chances).

**Website/app.** Either a public demo link that stays up (decision for the owner: the existing
Hostinger server could host it), or upload the pitch video (100 s) as the proof point.

**Well-positioned team (300 words).** *Owner only.* Solve wants engagement with the target
population: who you have talked to, how many, and what changed because of them. Kanonix and
Sibr show delivery speed; this prototype shows the method. Name a partner school or teacher
if you have one (G7 "needs the owner").

**Team size; time worked.** *Owner.* The prototype was built 14–16 October 2026 on a design
from 10–11 October.

**Why Solve? (200 words).** *Owner:* barriers. Likely candidates: a pilot school partner,
curriculum alignment, child-safety and privacy review, hosting cost.

**What makes it innovative? (300 words).**
- Pedagogy is a first-class, swappable layer: N frameworks + M experiences cost N + M, not N × M.
- The game *is* the framework, so rewards come only from evidence of learning.
- AI agents are governed actors under the same permissions as people.
- One audited core runs on phone and server and gives the same answers (the golden session).

**Tech stack and how you know it works (300 words).** Rust kernel (pure functions, no I/O),
compiled to wasm for the browser; axum + PostgreSQL with row-level security; Svelte 5 + PixiJS
offline PWA; Claude as an optional marker behind a rubric fallback. *How we know*: the facts
table. Technologies: Software and Mobile Applications; Generative AI.

**Business model (300 words).** *Owner.* The design note says it is not built to be sold to
a client; the horizon is another client reusing it across e-learning projects.

## Still needed from the owner (no agent can supply these honestly)

1. The words: Solve reads for the applicant's own voice and passion.
2. Community engagement and a partner (letter or quote), with real numbers.
3. "Our advantage": countries, schools and relationships.
4. Name, trademark and domain checks.
5. Public demo hosting, or the uploaded demo video.
6. A voice for narration (own recording or a licensed TTS), if the film should speak.
