# Pilot plan (a draft for the owner to rewrite)

Solve's feasibility criterion asks for a credible plan from prototype to pilot. This is a
skeleton with the parts the software already supports filled in. The school, the people, the
dates and the ethics route are the owner's; nothing here has been agreed with anyone.

## The question

Does a class that learns equivalent fractions with Zidara, offline on its own phones, learn
as much or more than it would otherwise, while the teacher spends less time marking?

## Stages

| Stage | Who | Length | Done when |
|---|---|---|---|
| 0. Agreement | one school, one teacher, the parents | 2–4 weeks | a signed letter; consent forms in Arabic and French; data rules of the country checked |
| 1. Teacher first | the teacher alone, then with the team | 1 week | the teacher has run the lesson, switched the method, accepted and undone an AI mark |
| 2. One class | one class of about 30 learners aged 9–11 | 3 weeks (the lesson's unit) | every learner finished the unit; the measures below collected |
| 3. Second class or school | a second teacher, ideally another setting (connection, language) | 3 weeks | the same measures, compared |

## What to measure

| Measure | How | Already in the software |
|---|---|---|
| Learning | the same short fractions test before and after, on paper | the unit's own test marks, by outcome (`/api/me`, the studio) |
| Use offline | share of work done with no connection | every command carries its device time; the server records when it arrived |
| Teacher time | minutes spent marking per lesson, by the teacher's own log | the studio's queue: proposals accepted, rejected, undone |
| AI quality | the teacher's agreement with AI marks | accepted vs rejected vs undone proposals, per class |
| Fairness | gaps between learners by language and device | the evidence log, by learner |
| Cost | requests per learner per day; tokens per marked answer | server logs; the API's `usage` (see `docs/cost-model.md`) |
| Experience | short interviews with learners, the teacher and a few parents | (not in the software) |

## What must exist before stage 2

- Real sign-in instead of the demo sign-in (OIDC or class codes), and the demo sign-in turned off.
- Hosting with HTTPS on a domain (`docs/deploy.md`).
- A privacy notice and a data-retention rule; a way to export and delete a learner's data.
- The narration voice decided (or the films stay captioned).
- A test bank with more question versions, so retakes stay fair over three weeks.

## Risks

| Risk | What reduces it |
|---|---|
| Phones shared or below the economy class | the low-end experience (the experience layer is replaceable by design) |
| No connection for days | the app works offline; work syncs later, and phone and server compute the same result |
| AI marks a child unfairly | only confident practice marks apply; tests always go to the teacher; every AI action can be undone |
| The teacher does not trust it | stage 1 with the teacher alone; the teacher decides every queued mark |
