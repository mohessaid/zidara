# Owner's checklist to submission

The deadline is **2 November 2026, 12:00 noon ET** (17:00 UTC; US clocks change on 1 November). Aim to submit on
**30 October** and keep 31 October to 2 November for problems. Only the owner can do these;
the software side is ready.

| By | Do | Why it matters | Where |
|---|---|---|---|
| 19 Oct | **Merge the work into `main`**, then **push to a repository the judges can open** (there is no remote yet) | `main` holds only the first commit; the evidence pack links to the code | `git switch main && git merge --ff-only heads/g3-bridge` (the branch: a tag has the same name and points at an older commit) |
| 19 Oct | **Write to two or three schools or teachers** for a pilot letter or a quote | letters take a week or more; a named partner is the biggest gap in the application | `docs/pilot-plan.md` stage 0 |
| 22 Oct | **Name checks**: trademarks (WIPO Global Brand Database, INPI, USPTO), the domain, the app stores; meanings in Tamazight, Turkish, Persian, Urdu, Malay, Swahili | a forced rename after submission costs the video and the text | `docs/naming.md` |
| 22 Oct | **Hosting: yes or no.** If yes, a VPS (a Rust binary and PostgreSQL; shared web hosting cannot run it) and a domain | a live link lets judges try it; without it, the video is the proof | `docs/deploy.md` |
| 25 Oct | **Voice**: record your own narration, license a voice, or keep the films captioned | the pitch and lesson films are silent today | `tools/video` |
| 25 Oct | **AI marking model**: Opus 5.5 (default, about $2.25 per learner a year) or Haiku 5.5 (about $0.06) | sets cost and accuracy; the policy keeps both safe | `docs/cost-model.md`, `ZIDARA_ASSESSOR_MODEL` |
| 28 Oct | **Write the application in your own words**: problem, who it serves, your advantage, team, why Solve, business model | Solve reads for the applicant's own voice; you sign that nothing is misrepresented | `docs/solve-evidence.md`, `docs/impact-facts.md`, `docs/cost-model.md` |
| 29 Oct | **Upload the pitch video** where Solve can play it (YouTube unlisted or Vimeo) | the form takes a link | `pitch/solve/zidara-solve-pitch.mp4` |
| 30 Oct | **Submit.** Check the deadline on the live form first | the Solve PDF gives two dates | solve.mit.edu |

## What to say carefully

- Stage: **Prototype** (0 users). Do not claim Pilot.
- Every number in the application should be one from `docs/solve-evidence.md`, which says
  where it is proven. The costs in `docs/cost-model.md` mix measurements and assumptions; say
  which is which.
- The demo sign-in lets anyone act as anyone. If the link is public, say it is a demo with
  invented learners.
