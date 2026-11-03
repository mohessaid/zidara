# What it costs to serve a learner

What it costs to run Zidara at scale, and whether that is sustainable. Measured numbers are
marked **measured** and say where they come from; everything else is an **assumption**, stated
so the owner can change it. Model prices are Anthropic's list prices as cached on 6 October 2026; check them before citing.

## Measured

| What | Number | Where |
|---|---|---|
| Server capacity on 2 CPU cores | about 690 requests/s, 0 failures | `make load` (`tools/load/capacity.js`) |
| Latency at 200 requests/s from 300 learners | p95 16–32 ms reads, 33–71 ms writes across runs | `make load` (`tools/load/k6.js`) |
| Download to make the app work offline | about 0.8 MB once (41 files, compressed), plus 0.24 MB for a lesson film in one language | the service worker's precache list in `web/dist/sw.js`; `web/public/media` |
| Page shown on first visit | 28 KB compressed | `make lighthouse` |

After that one download, a lesson costs no data: the work is stored on the phone and sends
small commands when a connection comes back.

## Hosting

The load test is harsher than real use: each of its 300 learners makes a request every 1.5 s,
around the clock.

- **Lower bound, measured only:** one 2-core server carried 300 learners at that rate with
  headroom to spare (200 of 690 requests/s). At capacity, that is about 1 000 learners each
  making a request every 1.5 s.
- **With assumptions:** a learner in a session makes one request every 10 s (answers and a
  few views), and at the busiest hour 30% of enrolled learners are in a session. Running at
  half capacity (345 requests/s) leaves room for spikes: 3 450 learners at once, so about
  **11 000 enrolled learners per 2-core server**.

| Assumption | Value |
|---|---|
| A 2-core VPS | $10–20 a month (check the provider) |
| Database | on the same server for a pilot; a managed PostgreSQL later |

Hosting per learner per year: $120–240 / 11 000 ≈ **$0.01–0.02** (assumptions), or
$120–240 / 1 000 ≈ **$0.12–0.24** (measured lower bound).

## Marking open answers with AI

Optional: without an API key, the rubric marks and the cost is zero.

The request (`crates/zidara-server/src/assessor.rs`) is a short system prompt, the question,
the rubric and the child's answer, with low effort and a small JSON reply.

| Assumption | Value |
|---|---|
| Input per answer | about 600 tokens (prompt, rubric, answer and the output schema) |
| Output per answer | up to about 400 tokens (the JSON and the model's brief thinking) |
| Open answers per learner | 2 per lesson, 3 lessons a week, 36 weeks: 216 a year |

| Model (price per million tokens, in / out) | Per answer | Per learner per year |
|---|---|---|
| Claude Opus 5.5 ($4 / $20) | $0.0104 | $2.25 |
| Claude Sonnet 5.5 ($2 / $10) | $0.0052 | $1.12 |
| Claude Haiku 5.5 ($0.10 / $0.50) | $0.00026 | $0.06 |

Levers, cheapest first:
- **Choose the model per deployment** (`ZIDARA_ASSESSOR_MODEL`). The kernel's policy gates every
  mark the same way, so a cheaper model changes cost and accuracy, not safety: only confident
  practice marks apply, and tests always go to the teacher.
- **Batch marking:** the Messages Batches API halves the price. Practice marks can wait for a
  teacher's next visit; the code would need a batch path (not built).
- **The rubric first:** answers the rubric marks clearly could skip the model (not built).

Prompt caching does not help here: the shared prefix is shorter than the minimum that can be
cached.

## Totals per learner per year

| Setup | Hosting | Marking | Total |
|---|---|---|---|
| Rubric only | $0.01–0.24 | $0 | **under $0.25** |
| Haiku 5.5 | $0.01–0.24 | $0.06 | **under $0.30** |
| Opus 5.5 (the default) | $0.01–0.24 | $2.25 | **about $2.50** |

The ranges come from the assumptions above. To replace them with real numbers, a pilot should
log requests per learner per day and the `usage` the API returns for each marked answer.
