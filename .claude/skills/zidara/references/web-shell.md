# The web shell

Distilled from the Svelte 5 best-practices skill and the official PixiJS v8 skills
(core and performance). For any PixiJS API detail, fetch the official index:
`https://pixijs.download/release/docs/llms.txt`.

## Shape

- Vite + TypeScript + Svelte 5, as an installable PWA. Not SvelteKit: there is no server
  rendering; the server sends data, not pages.
- The shell calls the kernel only through the wasm bridge (`zidara-wasm`), and the server only
  through the JSON API.
- Bridge types are generated from Rust (`ts-rs`), so the contract cannot drift.
- Experiences render **view intents** (`LearnerView`, `StudioView`). They never compute
  judgement themselves.
- Offline flow:
  - A service worker caches the shell and the lesson pack.
  - IndexedDB holds the local log and the outbox of `Envelope`s.
  - Back online, the shell posts the outbox and replaces its provisional log with the server's.

## Svelte 5

- State is `$state`, computed values are `$derived`. Use `$effect` only to sync with the
  outside world (PixiJS, IndexedDB), never to compute a value.
- Props come from `let { ... } = $props()`. Use `$bindable()` only where `bind:` is needed.
- Events are `onclick={...}`, not `on:click`. Use callback props, not `createEventDispatcher`.
  Use snippets with `{@render ...}`, not `<slot>`.
- Keep logic in plain TypeScript modules that Vitest can test without a DOM. Components stay
  thin.

## PixiJS v8 (the Quest map)

- Create the app with `const app = new Application(); await app.init({...})`. Draw with
  `Graphics` using `.rect(...).fill(...)`, not v7's `beginFill`/`endFill`.
- One `Application`. On teardown call `app.destroy({ releaseGlobalResources: true })`, or
  pooled textures leak into the next app and flicker.
- Pack art into spritesheets loaded with `Assets`. Avoid individual textures; batches break
  on texture, blend-mode and object-type changes.
- Group the scene graph by object type (sprites together, graphics together) so it batches.
- Text that changes (counts, stars) uses `BitmapText`. Never update `Text` every frame.
  Labels, and all Arabic text, live in the DOM over the canvas (proper shaping, screen
  readers).
- Use `cacheAsTexture` for static decoration that rarely changes. Call
  `updateCacheTexture()` after changes. Don't toggle it.
- Recycle objects (`Pool`) rather than destroying and recreating them every level.
- Set `interactiveChildren = false` on decorative containers and give hit regions a `hitArea`.
- Prefer rectangle masks (scissor) to graphics masks to sprite masks. Use filters sparingly,
  and set `filterArea` when the bounds are known.
- Leave `resolution` and `antialias` at their defaults until profiling on the economy profile
  says otherwise.

## Language and direction

- `<html lang="ar" dir="rtl">` by default. The locale switch updates both attributes and
  `Localized::get`.
- Use logical CSS (`margin-inline-start`, `inset-inline-end`). The map's path flows in the
  reading direction, so mirror it for RTL.
- Fonts: a subset of an Arabic face with Latin glyphs (for example Noto Sans Arabic), plus
  digits. Measure text in Arabic as well as English; Arabic runs taller.

## Quality gates (economy profile)

- In Playwright, use Chromium with a 4× CPU throttle and "fast 4G" on a mid-range Android
  viewport.
- The Quest map must hold 60 fps, with no long task over 50 ms during a level transition.
- Lighthouse performance ≥ 90 and accessibility = 100.
- Every screen must pass axe with 0 WCAG 2.2 AA violations (`references/accessibility.md`).

## The bridge (G3, proven)

- Build: `scripts/build-wasm.sh` → `web/src/lib/kernel/pkg` (`--target web`) and
  `target/wasm-node` (`--target nodejs`, for tests). Load with the web target's `init()`.
- `new Shell(JSON.stringify({ lesson, framework, tenant, cohort }))`; `run(actorJson,
  envelopeJson)` → `{status: "applied", events} | {status: "duplicate"} | {status: "refused",
  error}`; `learnerView(user, locale, nowMs)`; `studioView(rosterJson, locale, nowMs)`;
  `events()`; `load(eventsJson)` after a sync. Errors throw.
- Envelope ids come from `crypto.randomUUID()`; the shell keeps the outbox in IndexedDB and,
  after posting it, replaces its log with the server's (`/events`) via `load`.
- Pass the device clock as `nowMs` (Cambridge calendar locks depend on it).
- Types: import from `web/src/lib/kernel/types/` (generated; never edit; `make types`).
- Queued agent proposals carry the learner's id; the Quest must not show them (only the
  studio does).

## Proven in G4

- Keep the PixiJS map mounted and running (re-creating it cost 350–405 ms; pausing it moved a
  GPU upload into the first frame back). Its canvas is `aria-hidden`; DOM twins are the controls.
- Never bind a template to state created in `$effect`: effects run after the first render.
  Initialise it in the `$state(...)` call.
- `aria-label` must contain the visible text (WCAG 2.5.3); prefer visually hidden text inside
  the element.
- Syncs queue (`syncNow` chains on a promise); never skip one because another is running.
- The sign-in page must stay light: screens load on demand, and the kernel's wasm is fetched
  after sign-in or 2 s after idle. That took Lighthouse performance from 51 to 100.
- Lighthouse: `make lighthouse` (gate: performance ≥ 90, accessibility = 100).
