# MIT Solve: 2027 Global Learning Challenge

Page: https://solve.mit.edu/challenges/2027-global-learning-challenge (read 13 October 2026).

## The challenge

> How can we close global education gaps while improving outcomes for every learner?

- **Deadline:** 2 November 2026, 12:00 noon ET (applications opened 22 September 2026).
- **Stage:** Prototype, Pilot, Growth or Scale. **Concept-stage ideas are not eligible**
  (except for the Indigenous Communities Fellowship), so a working prototype is required.
- **One solution per challenge** per team lead.
- **Prizes:** every Solver team gets a $10,000 grant (about $40k on average in total), plus a
  pool of over $1M. Relevant: AI for Humanity Prize (up to $150,000 for AI, data science or
  ML), E Ink Innovation Prize ($100,000, interest in education), and the Crescent Enterprises
  AI for Social Innovation Prize (up to $100,000, up to four teams).

## Focus areas, and how Zidara answers them

| Solve wants | Zidara |
|---|---|
| **Strengthen teachers' practice**, supporting their judgement and relationships | The teacher studio shows who is stuck on which outcome; agents pre-mark and triage, while teachers keep every summative judgement. |
| **Emergency and low-resource settings:** low bandwidth, offline, low cost | Native apps that install once and work offline, read-only lesson packs, a data-only API, video through a CDN, and a compiled core that runs on modest servers. |
| **Assistive technology and multilingual** | WCAG 2.2 AA tested automatically; right-to-left and many languages in the kernel; experiences per age, including no-reading-required for young children. |
| **Better assessment:** formative, diagnostics, early flags | The evidence log ties every piece of work to outcomes; frameworks define judgement; an operations agent flags learners going off track. |

Cross-cutting priorities:
- **AI that deepens learning:** agents are governed actors inside the pedagogy, not a chat box beside it.
- **Peer, teacher and community connection:** role surfaces for teachers and parents.
- **Designed with the communities served:** `TODO` (see the gaps below).
- **Curricula and interoperability standards:** CASE, QTI, xAPI, LTI 1.3, SCORM import, Open Badges; any curriculum becomes a framework.
- **Strengthen public systems:** one kernel serves many schools, and every framework and mini-app built is reused.

## Judging criteria

| Criterion | Our evidence | Gap |
|---|---|---|
| Alignment | Above | |
| Potential for impact | | Who is served first, how many, in which settings |
| Feasibility | Kanonix and Sibr show a small team shipping fast; the prototype; a pilot draft (`pilot-plan.md`) | The owner's dates and school |
| Innovative approach | Pedagogy as a first-class layer; N + M instead of N × M; governed agents; one Rust core everywhere | |
| Human-centred design | The team's lived knowledge of the communities | Named partners, co-design sessions, learner and teacher quotes |
| Scalability (finalists) | Stateless runners, one replicated database, CDN for media, every framework reused; cost per learner (`cost-model.md`) | Revenue side of the model |
| Partnership potential (finalists) | | What Solve's network would unlock |
| Technical feasibility (finalists) | The prototype and its gauntlet (`solve-evidence.md`) | A live link |

## Our advantage

The builder knows the world the entry serves, firsthand. `TODO`: write it in your own words.
Name the communities and settings (countries, languages, connection conditions, school
models), what you have seen there, and the relationships you can bring.

## Before 2 November

1. **Prototype (required):** the kernel with two frameworks, one lesson (video, text, quiz),
   evidence and progress, an offline-capable client, and the test gauntlet.
2. **One partner or pilot setting**, even small, with a letter or quote.
3. **The application text**, using the tables above.
4. **A short video:** cut from `pitch/film`, with every mention of a client, Moodle internals and
   another client removed.
5. **Name checks** (`naming.md`).
