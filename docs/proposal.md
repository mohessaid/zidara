# Zidara: a learning platform that knows how teaching works

A from-scratch design, first worked out on 10–11 October 2026. It is not a plugin platform like
Moodle; teaching itself is built in. One kernel carries many pedagogies, each audience gets an
experience made for it, and automation (AI agents included) runs inside rules that people
approve. The core is written in Rust and shared by the server, the offline web client and
native apps.

## 1. The problem with today's platforms

- **General-purpose LMSs carry everything.** Moodle ships every activity, tool and teaching
  approach in one large application (more than 400 million users, 160,000+ sites). Features
  can be switched off but never left out, so it cannot become a small, portable app shaped by
  how a school teaches.
- **Its frameworks cannot be detached.** Moodle has competency frameworks (`core_competency`)
  and a SCORM player (`mod_scorm`: SCORM 1.2 in core, SCORM 2004 only in part), but both live
  inside Moodle, tied to its own courses and activities. Neither can be lifted out and reused.
- **Automation happens between systems.** Student information system, LMS and tools are
  separate; every arrow between them needs mapping, retries and reconciliation. Scaling by
  running more LMS instances multiplies that work: N instances, N databases, N upgrades.
- **Pedagogy is not automated.** Today's automation automates administration; the
  teaching itself stays outside the system's understanding.

## 2. The model: three axes plus automation

```
                ┌──────────────────────────────────────────────┐
  EXPERIENCE    │ Early-years path · Secondary dashboard ·     │  how it looks and feels,
  (how)         │ Teacher studio · Parent view · Chat tutor    │  per audience and device
                ├──────────────────────────────────────────────┤
  FRAMEWORK     │ Cambridge · Mastery · IB · Competency-based ·│  what learning means:
  (what)        │ Spaced repetition · a client's own pedagogy  │  outcomes, evidence, rules
                ├──────────────────────────────────────────────┤
  KERNEL        │ people · groups · content · activities ·     │  built once,
  (foundation)  │ evidence log · scheduling · API · security   │  tested hard
                └──────────────────────────────────────────────┘
  AUTOMATION ═══ runs through all three: events, rules, AI agents as actors
```

One kernel; frameworks and experiences mix freely. Cambridge can run with a playful experience
for 6-year-olds and a dense one for 16-year-olds; a training centre's competency framework can
reuse the secondary dashboard. Each combination is a configuration, not a new build.

### 2.1 What a learning framework is

A framework is a pedagogy made explicit. Compared side by side (Cambridge, Bloom's mastery
learning, competency-based education, IB assessment criteria, spaced repetition, project-based
learning, SCORM packages as a content standard), every one reduces to the same six building
blocks:

| Building block | What it is | How frameworks differ |
|---|---|---|
| **Outcomes** | What a learner should be able to do, as a graph or tree | Syllabus objectives, IB criteria, vocational competencies |
| **Activities** | What the learner does: read, watch, practise, project, discuss | Mix and order |
| **Evidence** | What a learner's work proves about which outcome | Quiz score, rubric, observation, portfolio |
| **Judgement** | How evidence turns into a level or grade | Grade boundaries, mastery thresholds, rubric bands, pass/fail |
| **Progression** | What unlocks next | Fixed calendar, mastery gates, free choice, spaced review |
| **Roles** | Who teaches, assesses, moderates, mentors | Teacher, examiner, moderator, peer, AI tutor |

The blocks stay; only the rules change. Cambridge judges against grade boundaries through the
year; mastery learning unlocks the next step at 80%; competency-based education moves each
learner on when they show the skill; spaced repetition schedules each review just before
memory fades. **A new framework costs weeks, not a new product.**

How a framework is added:
- **Data:** outcome trees, grade boundaries and thresholds are imported as data (CASE for
  outcome frameworks).
- **Code:** judgement and progression logic is compiled code behind a fixed interface (a Rust
  trait), through the full test gauntlet.
- **Never runtime third-party plugins:** that is Moodle's problem again (quality, security,
  upgrades).

### 2.2 The experience axis

A framework says what is true ("three outcomes in progress, one needs review, the next unlocks
on mastery"); it never says "show a progress bar". Each experience renders that intent its own
way: a glowing path for a 6-year-old, a mastery grid for a 16-year-old, "3 of 28 learners are
stuck on outcome 4.2" in the teacher's studio, a nudge from the AI tutor. **N frameworks plus M
experiences cost N + M to build, not N × M.**

Experiences vary by audience age, role, mode (self-paced, live, project, exam, conversational),
device and connection, language and script (right-to-left included), and accessibility (WCAG
2.2 AA as the floor). They are built in four layers:

1. **Design system:** tokens and components; an institution's brand is a token set, not a fork.
2. **Activity mini-apps:** a quiz, a reading with audio, a lab simulation, a coding sandbox.
   Each runs in a host shell and talks to the kernel only through a typed bridge ("record this
   evidence", "fetch this content"). It cannot touch the database, so it cannot break the
   system, and every mini-app is reusable by every institution.
3. **Role surfaces:** the learner's path, the teacher's studio, the parent's summary.
4. **Experience templates:** named combinations for an audience ("Early years", "Secondary",
   "Adult professional").

### 2.3 Automation: the next level of automation

| Today | This model |
|---|---|
| Integrations between systems | One kernel, one data model; nothing to synchronise internally |
| Automation reacts to copied records | Automation reacts to **events** as they happen (evidence recorded, outcome mastered, learner inactive 5 days) |
| Rules hard-coded per integration | Rules live in the framework ("mastery < 60% after 2 attempts: assign remediation, tell the teacher") |
| Bots fill gaps | **AI agents are first-class actors** with roles and permissions, every action logged and reversible |
| Analytics assembled afterwards | The evidence log (xAPI-shaped, append-only) is the analytics |
| Automating administration | Automating **pedagogy itself**, inside rules people approve |

Agents as actors, under the same permission system as humans:
- **Tutor agent:** reads a learner's evidence, suggests activities; never grades summative work.
- **Assessment agent:** pre-marks with a confidence score; anything uncertain goes to a teacher.
- **Operations agent:** spots learners at risk and opens a case for a human.
- **Authoring agent:** turns a curriculum document into a draft framework for a designer to approve.

## 3. Architecture

```
            native apps (iOS, Android, desktop) · light web client
                                  │
          edge: load balancer  ·  CDN for video and files
                                  │
     ┌──────────┬────────────────┴─┬──────────┐
  runner 1   runner 2           runner 3    (more on demand)   ← one app, identical, stateless
     └──────────┴────────┬─────────┴──────────┘
          PgBouncer → PostgreSQL primary ──streaming──► replicas (reads, failover)
                         │
          object storage (files) · queue (jobs) · identity (OIDC)
```

- **One application, many entry points.** Runners hold no state, so traffic growth means adding
  runners; a failed runner is routed around.
- **One database cluster:** a primary takes writes, replicas take reads and stand by for
  failover. It is the one part scaled with care, and measured before it grows.
- **Video never touches the app servers:** 1,000 concurrent viewers at 1.5 Mb/s need 1.5 Gb/s,
  more than a typical server uplink. It goes through a CDN or streaming service.
- **Segregation and isolation are data rules:** every row carries its cohort and tenant, and
  PostgreSQL row-level security enforces them, so no screen and no agent can bypass them.

### 3.1 The platform is an API; the apps are native

- The server sends data, not pages; the web client stays light and the server stops serving
  front-end files on every visit.
- Native apps install once, work offline and feel immersive: Swift on Apple devices, Kotlin on
  Android, native desktop, around one shared core (the approach 37signals took with HEY).
- Offline lesson packs (read-only) are cheap; offline edits synced later are hard (conflict
  resolution) and are scoped carefully.

### 3.2 Why Rust

For:
- **One core, compiled several ways:** the server (axum on PostgreSQL), WebAssembly in the web
  client for offline use, and the shared core inside native apps.
- **Small footprint and predictable performance:** a compiled core and a small API serve
  thousands of learners on modest servers.
- **Strict types:** suited to code largely written by AI agents and proven by tests.
- **Evidence that agents build Rust at scale:** HEY's Rust core with six native apps (Rails
  World, 23 September 2026); ArtCraft's seven open-source creative apps written in Rust.

Honest limits:
- At school scale the app server is not the bottleneck; the database and video delivery are.
- Someone on the team must be able to review and fix Rust when an agent gets it wrong.
- Branch coverage (`cargo-llvm-cov --branch`) still needs a pinned nightly toolchain in its own
  CI job; mutation testing (`cargo-mutants`) is mature.
- Identity: any standard OIDC provider works through a Rust OIDC client.

## 4. Built in from day one

- **Safeguarding minors:** age-appropriate sign-in, no public profiles, controlled messaging,
  audit trails.
- **Privacy:** data minimisation, consent, export and deletion, per-tenant isolation.
- **Languages:** internationalisation and right-to-left in the kernel, not in each screen.
- **Accessibility:** WCAG 2.2 AA, tested automatically.
- **Open standards:** CASE (outcome frameworks), QTI (questions), xAPI (evidence), LTI 1.3
  (outside tools), SCORM import (legacy content), Open Badges (credentials).
- **Testing discipline:** unit, acceptance, end to end, mutation (every mutant killed), 100%
  line and branch coverage, integration against real services.

## 5. Proof that a small team can build it

- **Kanonix** (legal research for Algerian law): 11,000+ official-journal documents from 1962
  to today, 110,000 pages in Arabic and French, 38 extraction passes for about $300 of
  Document AI (estimated at $100,000+ and years of human work in 2016). Built in Elixir and
  Phoenix, learned for the project, under the full test gauntlet; RAG with pgvector, search on
  OpenSearch; shares 2 vCPUs and 8 GB with other apps. Launched 1 October 2026 with no
  marketing: 2,700 users in its first full week, up to 480 a day.
- **Sibr:** v1 by a team of three on known technologies took 15 days plus 2 months of polish
  and shipped polls. With Claude Opus 5.5 (Fable 5.1 as advisor), one developer moved it to the
  Internet Computer and Motoko, both new to the team, in one week, and added surveys, studies
  and earnings.

## 6. How to start safely

A three-week spike decides, with evidence rather than opinion:
- the kernel with two very different frameworks (Cambridge-style and mastery) to prove the six
  building blocks hold;
- one lesson with video, text and a quiz; progress tracking and a basic gradebook;
- cohort isolation enforced by the database;
- the full test gauntlet wired in from day one;
- at the end of week 3, compare real speed with the estimates and decide.

## Sources

- [Rails World 2026 keynote: DHH, HEY, Rust and AI agents](https://rustify.rs/articles/rails-world-2026-keynote-dhh-hey-rust-ai-agents)
- [ArtCraft: open-source Adobe alternatives (PetaPixel)](https://petapixel.com/2026/10/07/someone-rebuilt-free-open-source-versions-of-photoshop-premiere-and-lightroom-using-ai/)
- [Khan Academy: Go + Services = One Goliath Project](https://blog.khanacademy.org/go-services-one-goliath-project/)
- [Moodle Competencies](https://docs.moodle.org/403/en/Competencies), [SCORM activity](https://docs.moodle.org/38/en/mod/scorm/view), [SCORM FAQ](https://docs.moodle.org/23/en/SCORM_FAQ)
- [Moodle 5.3 AI usage report](https://docs.moodle.org/503/en/AI_usage_report)
- [cargo-llvm-cov branch coverage](https://github.com/taiki-e/cargo-llvm-cov/issues/8), [cargo-mutants](https://github.com/sourcefrog/cargo-mutants/releases)
