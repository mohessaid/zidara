# 0008. Low-end phones start in the light experience

Date: 2026-10-18. Status: accepted.

## Context
The prototype targets economy phones (3–4 GB) with the map, a PixiJS game scene that keeps
rendering. Phones with 2 GB or less, and anyone with data saver on, pay for a scene they may
not need. The design says low-end devices get their own experience; the kernel and the views
are the same for every experience, so only the screen changes.

## Decision
A learner starts in the table experience, and the map's code (PixiJS, GSAP, 100 KB
compressed) is not loaded or run, when the browser reports data saver (`Save-Data`) or
`navigator.deviceMemory` of 2 GB or less. Otherwise the map starts as before. The learner can
switch either way; once loaded, the map stays mounted (the earlier measurement still holds).
The offline bundle still contains the map's code: excluding it from the service worker's
precache would save the bytes but could leave a learner without the map offline if their
first visit ended before the worker took control.

## Consequences
- Measured on the same throttled phone (4× CPU), signed in to the same screen: JS heap 3.1–3.3
  MB against 5.5–5.6 MB, main-thread work 710–723 ms against 1 542–1 628 ms, script 143–144 ms
  against 517–538 ms. The map also keeps a render loop running while shown; the light
  experience has none.
- Only Chromium reports these hints. Other browsers get the map, as before.
- Saving the bytes too (a data-saver bundle) is a later step for the low-end experience.
