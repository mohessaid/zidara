# Changelog

All notable changes to Zidara are recorded here. The format follows
[Keep a Changelog 1.1.0](https://keepachangelog.com/en/1.1.0/).

## [Unreleased]

### Added

- The front page is light (ADR 0017), in the demo's own tokens: ink headings, white and grey
  sections, one cobalt accent (#2445e0) on both pages, the owner's pick of four measured
  palettes; saffron is gone. No logo, no gradients, no badges; a test fails on
  any gradient or logo. Its screenshots and share image show the new look.
- The contrast walk now measures disabled buttons and every progress bar's track too.

### Fixed

- After Check, the answers already given stayed solid tiles; for a moment they had taken the
  dashed edge meant for buttons not ready yet.
- Zidara reads as what it is: an open-source learning engine for any language. The README,
  the front page, its share text and the project skill no longer single out languages or regions ("right to
  left from the start" says what matters), and the five whys use global figures. The
  competition, its deadline and the application material left the repository.
- In the demo, a disabled Check, the studio's Apply, and the empty part of every progress bar
  were the colour of the ground behind them. Disabled buttons now keep a dashed edge and
  every bar track a visible edge. The saffron Enter and Start buttons are ink, and the badges
  ("Everything is saved", the point chips) are plain words.

- A new face (ADR 0016). The front page is a dark, high-contrast stage: Alexandria headings,
  one saffron accent, a console in the hero where a request to Zidara is answered layer by
  layer, corner brackets travelling along the closing phrase, no numbered section labels. The
  demo is a light lesson in the manner of Brilliant: course cards, one column of levels, Check
  pinned at the bottom, a green or orange panel that says why. Bizra, the oasis scene and the
  palms are removed; new screenshots, share image and app icon.
- Contrast is measured on every screen: every visible text against what is really behind it,
  and every control's edge (3:1), inside each accessibility check and in a static tour of all
  screens. Fixes from it: darker green and orange, a visible edge on every control, unlit stars
  drawn as outlines, edges on the language picker and the front page's cards.
- The game and the studio now speak the front page's language (ADR 0015, art bible): every
  picture is a grid of dots (the oasis, Bizra, palms, course badges, the burst after a right
  answer), white ground under a faint dot grid, ink and blue buttons, thin lines, mono labels,
  levels as dotted tiles, Readex Pro everywhere (Arabic and Latin cached for offline). Baloo
  Bhaijaan 2 is gone.
- `ZIDARA_DB_PORT` sets the local database's port (default 55432) for Docker, the dev server,
  the browser tests and the load tools, for when another project holds 55432.
- The front page now pitches Zidara as what it is (ADR 0015): the adaptable learning engine for
  the AI era, one kernel under any framework and any experience, with governed AI agents. The
  live console switches both the framework and the experience (children's path, secondary grid,
  teacher's studio, text message) over one view of the real kernel. A context.dev-style face:
  hand-written dot-matrix canvases for the hero, the architecture, each card and the wordmark;
  ruled sections; a new share image. The game appears as one experience among many.
- A front page at `/` (ADR 0014) that pitches Zidara: a live proof in the hero, where the real
  kernel, in a Web Worker, judges one learner's same answers under Mastery and Cambridge-style;
  five whys with every figure sourced; how it works; real screens under "Everything here is made
  with Zidara"; the gauntlet's numbers; questions. Arabic, French and English, light and dark,
  with share metadata and a share image taken from the real hero.
- `girth-landing`, a project skill for the front page, distilled from design-taste-frontend and
  seo-aeo-best-practices.

- The pilot server is live at https://zidara.ta9in.com (DNS record added, certificate issued).
- The pedagogy lens (ADR 0013): under the path, "School rules: Mastery | Cambridge-style"
  replays the learner's own evidence in a throwaway kernel under the other framework and
  redraws the path from it. Nothing is run, saved or sent; the class keeps its teacher's choice.
- The oasis (ADR 0013): learner screens stand on sand under a morning sky, with dunes, water and
  date palms drawn in SVG. The courses are horseshoe-arch doorways, the levels eight-pointed
  stars, and a palm grows for every outcome the framework says is achieved.
- A right answer: Bizra cheers, small stars burst and "+10" rises; the finish screen counts the
  points up, lands the stars one by one and grows a palm on mastery. Off under reduced motion.
- Guests (ADR 0012): the front page offers the two courses as cards, and one tap starts a
  guest learner in it, with no account and no names. `POST /api/guest` makes a learner in every
  open class through `private.join_as_guest`, the only way the app can write users, capped at
  2 000 guests until the nightly reset; the browser demo does the same. A guest switches
  courses from the menu. The demo classes (named people, teachers) moved to `#classes`.
- A path in the style of Duolingo: a unit banner, round levels winding down the screen, the next
  one glowing with a START bubble, gold when mastered, locked levels that say why when tapped,
  and a trophy at the end. A game bar shows the course, the streak and the points.
- Bizra, the guide: a seed with a sprout, drawn in SVG, who reads to the learner in a speech
  bubble, cheers mastery and encourages after a miss.
- A second track, "How Zidara works", for anyone: how work becomes evidence and a verdict, what
  switching the teaching approach does, and what the AI may and may not do, in Arabic, French
  and English. Its open answer is marked by the governed assistant it describes. It has its own
  class ("Discover Zidara"), with Lina learning and Omar teaching, on the server and in the
  browser demo. The learner's header now shows the class's own lesson title.
- A lesson player in the style of Duolingo and Brilliant: tapping a stone opens a session, one
  step per screen, with a progress bar. Quiz questions come one at a time and are checked at
  once by the kernel's own marking, with a gentle "let's look again", the answer and why; the
  first try at each question is what is submitted as evidence. Fractions are answered by
  shading or cutting bars, questions show bars and round shapes, and two toys (folding a strip,
  a fraction wall) come before practice. The finish screen shows what the class's framework now
  says, with stars, XP and the streak, both read from the log.
- Live rounds in the app: the teacher's studio has a board to project (pick a quiz, start, show
  the answer with each option's count, next, end with the class's result); every phone in the
  class follows within seconds. Phones sync every 6 s while on screen and every 1.5 s during a
  round.
- Sounds synthesised in the browser (no files), with a mute switch; no losing sounds.
- The lesson pack: every quiz question, in every version, has a picture, an answer input and an
  explanation in Arabic, French and English; two explore steps were added.
- Live rounds in the kernel: a teacher starts a round on a quiz, every phone in the class shows
  its questions at the teacher's pace, the kernel marks each answer as it arrives, and ending
  the round turns each learner's answers into practice evidence the class's framework judges.
  The board counts answers and picks per option; nobody is ranked. Answers name their round,
  so a phone whose clock is behind cannot put an answer in another round.
- Questions can carry a look: a picture (a bar, a circle, or two bars to compare), a way to
  answer (type, shade a cut bar, or cut and shade), and an explanation shown after answering.
  The lesson refuses a look that cannot be drawn or cannot give the answer.
- An explore step with a toy (fold a strip, a fraction wall): something to play with before
  practising, recording no evidence.
- The phone's kernel can check one answer at once (`check`), with the kernel's own marking;
  evidence still comes only from submitting the quiz.
- The real server runs at https://zidara.ta9in.com (ADR 0010): its own user,
  database and roles on a shared machine, one Caddy file, and a clean school every night.
- The demo school is published at https://zidara.pages.dev and the code at
  https://github.com/mohessaid/zidara.
- The demo school in the browser (ADR 0009): `pnpm build:demo` builds the app with the school
  inside the page, as static files any host can serve; `make demo` (and `ZIDARA_URL=… pnpm
  demo` for a published link) runs the whole story with no server. The sign-in page says what
  the demo does not show, in three languages.
- `visibility::visible_to` and `governance::rubric_marks` in the kernel, exposed to the browser
  (`visibleEvents`, `rubricMarks`); the server's `learner_of` now comes from the kernel.
- The light experience: with data saver on, or a phone reporting 2 GB of memory or less, a learner
  starts in the table and the map's game engine is not loaded until asked for (ADR 0008). On a throttled
  phone: 40% less JS memory and half the main-thread work. If the map's code cannot arrive
  (the connection dropped after sign-in), the table takes over with a message and the map
  button tries again.
- `pnpm smoke` (`web/playwright.deploy.config.ts`): checks a running deployment at `ZIDARA_URL`
  without changing its data: the API refuses visitors, a learner signs in, and the app reloads
  with no network.
- `docs/cost-model.md` (cost per learner: measured capacity plus stated assumptions),
  `docs/pilot-plan.md` (a draft for the owner),
  `docs/impact-facts.md` (public figures with sources, for the owner to check and choose from).
- One process for the whole prototype: `ZIDARA_WEB_DIR` makes the server also serve the built
  web app (unknown `/api/` paths stay JSON 404s). `docs/deploy.md`: build, database roles,
  systemd, Caddy.
- `tools/load/`: k6 load and capacity tests against a release server pinned to 2 cores.
- The 26-second demo of the real app (`web/tests/demo`, `scripts/compose-demo.py` →
  `pitch/demo/zidara-demo.mp4`).
- The lesson film "What is a fraction?" in Arabic, French and English, drawn from code
  (`tools/video`), with a transcript as its text alternative; `content/CREDITS.md`.
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

- The game moved to `#play`; `/` is the front page, and an installed app opens the game.
- The interface opens in English unless the visitor chose a language before (ADR 0014).
- New look (ADR 0012): white ground, chunky buttons with a solid edge, one rounded typeface for
  Arabic and Latin (Baloo Bhaijaan 2), brighter colours kept to WCAG AA, a feedback sheet that
  rises in green or blue, and finish cards for the session's points, all points and the streak.
- The nightly reset of the hosted sandbox is `private.reset_demo()`, which also removes guests.
- The real server is at https://zidara.ta9in.com; the interim sslip.io address is gone.
- The browser's kernel is built with its own size profile (`opt-level = "z"`, `wasm-opt -Oz`).
  It is 184 KB gzipped, up from 164 KB: serde's code for the new tagged enums.
- `docs/deploy.md`, after a rehearsal on a fresh database: Rust from rustup (a distribution's
  `cargo` is too old), a real `ZIDARA_SECRET` for `migrate` and `seed` (a placeholder is
  refused), the owner role over TCP, and the browser smoke check.
- The docs quote the measured numbers of the final runs (they still quoted earlier ones).
- The pitch video, re-rendered: the kernel is 150 KB, 95% of pages within 32 ms (it said 30, which one
  load run exceeded), and the light grid for low-end phones.
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

### Removed

- The PixiJS map, GSAP, the table view and the light experience: the HTML path replaces them.

### Fixed

- The app's typeface (Baloo Bhaijaan 2) had never rendered: an inline splash style on `<body>`
  set `system-ui`, which beat the face set on `:root`. Every screen fell back to the system font.
- A burst of stars could widen the page by a few pixels, and phones zoomed out to fit it.

- The points in the learner's bar stayed at 0 after a session until a reload: the value
  short-circuited on the kernel, which is not reactive, before reading the log.
- A poll that brings nothing new reloads and redraws nothing, and the log is read once per change
  for XP and the streak: with polls every few seconds, idle syncs were redrawing every screen.
- A live answer that reaches the server after the teacher moved on is dropped quietly instead of
  showing a child "refused by the server"; the phone follows the board's question.
- Commands built from a screen's state are copied before they are stored: IndexedDB cannot
  store Svelte's state proxies ("could not be cloned").
- Material from another project's pitch is removed from the repository and its history, and the
  design note (`docs/proposal.md`) no longer carries that project's framing.
- Browser tests no longer assume a date: under Cambridge-style, F3 opened on 19 October, and
  the test that expected it locked would have failed from then on.
- The Lighthouse runner starts Chromium without its sandbox, as Playwright does: AppArmor on
  the updated kernel stopped the sandbox from starting.
- A proposal whose mark a teacher undid now reads `undone`, not `applied` or `accepted`: evidence
  recorded from a proposal names it (`EvidenceRecorded.proposal`, omitted otherwise, so older
  logs read unchanged).
