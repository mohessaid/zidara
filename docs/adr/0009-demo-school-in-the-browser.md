# 0009. A demo school in the browser

Date: 2026-10-31. Status: accepted.

## Context
Anyone should be able to try the prototype, not only watch it. The real server needs a VPS
with PostgreSQL, which the owner has not decided on, and a public server with a demo sign-in
lets anyone write to a shared database. The kernel already runs in the browser.

## Decision
A separate build (`pnpm build:demo`, `VITE_ZIDARA_DEMO=browser`) answers the app's API inside
the page. `DemoSchool` takes the server's place behind the same `fetch`: the same paths, JSON
and status codes, the server's demo people, and the same rules from the same wasm kernel:
commands run as the signed-in person, each person reads what the server's database would show
them (`visibility::visible_to`), and the assessment agent marks open answers with the rubric
(`governance::rubric_marks`). Both rules moved from the server into the kernel, so there is
one rule, tested and mutation-tested once. The real build carries none of it: Vite replaces the
flag when building, and the build was checked for the demo's code.

## Consequences
- The demo is static files: any host serves it, at no cost, and each visitor's school lives in
  their own browser. Nobody can change what another visitor sees.
- It shows the pedagogy, the learner's path, the light experience, offline use, the teacher's
  decisions and the framework switch. It does not show the server: PostgreSQL row-level
  security, sync between devices, load, or a model marking. The sign-in page says so in three
  languages, and nothing public may cite the demo as proof of those.
- The browser kernel grew from 150 to 164 KB compressed (the rubric marker). Lighthouse stays
  100/100 for both builds.
