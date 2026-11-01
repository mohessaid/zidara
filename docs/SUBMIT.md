# Owner's checklist to submission

The deadline is **6 November 2026, 12:00 pm EST** (17:00 UTC), as the live challenge page read
on 31 October. Until then we planned for 2 November, from an earlier reading. Aim to submit on
**4 November** and keep 5–6 November for problems. Only the owner can do these; the software
side is ready.

| By | Do | Why it matters | Where |
|---|---|---|---|
| 31 Oct | **Done 1 Nov:** the code is public at https://github.com/mohessaid/zidara | the evidence pack links to the code | `gh repo create` then `git push -u origin main` |
| 31 Oct | **Write to two or three schools or teachers** for a pilot letter or a quote, even one line by email | a named partner is the biggest gap in the application | `docs/pilot-plan.md` stage 0 |
| 1 Nov | **Name checks**: trademarks, the domain, the app stores, meanings in other languages | a forced rename after submission costs the video and the text | `docs/naming.md` (research findings added there) |
| 1 Nov | **Done 1 Nov:** the demo is at https://zidara.pages.dev (Cloudflare Pages) | a link lets judges try it; without it, the video is the proof | `docs/deploy.md`, "The demo link" |
| 2 Nov | **Voice** (optional now): the pitch is captioned and the app has its own synthesised sounds (no files, clear rights). Record your own narration only if you want a voice | the pitch and lesson films have no spoken words | `tools/video` |
| 2 Nov | **AI marking model**: Opus 5.5 (default, about $2.25 per learner a year) or Haiku 5.5 (about $0.06) | sets cost and accuracy; the policy keeps both safe | `docs/cost-model.md`, `ZIDARA_ASSESSOR_MODEL` |
| 3 Nov | **Write the application in your own words**: problem, who it serves, your advantage, team, why Solve, business model | Solve reads for the applicant's own voice; you sign that nothing is misrepresented | `docs/solve-evidence.md`, `docs/impact-facts.md`, `docs/cost-model.md` |
| 4 Nov, 10:00 ET | **Optional: Solve's English application clinic**, to ask questions directly | the last clinic before the deadline | the challenge page |
| 4 Nov | **Upload the pitch video** where Solve can play it (YouTube unlisted or Vimeo), then **submit** | the form takes a link; keep two days spare | `pitch/solve/zidara-solve-pitch.mp4`, solve.mit.edu |

## What to say carefully

- Stage: **Prototype** (0 users). Do not claim Pilot.
- Every number in the application should be one from `docs/solve-evidence.md`, which says
  where it is proven. The costs in `docs/cost-model.md` mix measurements and assumptions; say
  which is which.
- The demo link runs in the visitor's browser with invented people. It shows the pedagogy and the
  teacher's decisions. The real server (https://zidara.ta9in.com) shows sync between
  devices and live rounds; it marks with the rubric, not a paid model. Cite the tests in
  `docs/solve-evidence.md` for isolation and AI marking.
- To show a live round: open the real server as Karim on a laptop and as Amina on a phone.
