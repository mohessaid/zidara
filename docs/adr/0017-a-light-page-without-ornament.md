# 0017. A light front page without ornament; nothing in the demo that fades into its ground

Date: 2026-11-03. Status: accepted. Supersedes the front page's look in ADR 0016 (its type,
motion and measured contrast stand).

## Context
The owner did not like the dark front page's colours and asked for light mode, no logo, and no
gradients or badges: "obvious that's ai, everyone is using them". In the demo, "the colors are
still sometimes background like foreground". Our contrast walk passed, so it had blind spots.
Screens showed them: a disabled Check was grey on grey (WCAG exempts disabled controls, so we
skipped them), and every progress track was #f4f4f5 on white (1.1:1), so empty bars vanished.

## Decision
- **One design system.** The front page uses the demo's light tokens: white and one grey
  surface for alternate sections, ink Alexandria headings, grey reading text.
- **One accent, cobalt (#2445e0), the owner's pick from four measured palettes.** It is 7.0:1
  on white and white on it is 7.0:1, so it serves as text, as a button face and as the
  drawings' lit dots. Saffron failed as a control (1.8:1 on white) and is gone. Cobalt on the
  ink block is 2.7:1, so it never sits there. States keep their own colours, the same on both
  pages: mastered green, in progress cobalt, help orange, locked grey.
- **No logo, gradients or badges.** The name is a word. Frames are one plain edge. States
  are words with a small square of colour. Pills, chips, glows and dotted gradient fills are
  removed. A test fails on any gradient or logo on the front page.
- **Nothing fades into its ground.** Disabled buttons keep a dashed 3.4:1 edge and muted words
  (7.4). Every bar that fills has a white track with a 3.4:1 edge. `lowContrast` now measures
  disabled controls and every progress bar or `[data-meter]`; a planted-mistake test proves it.

## Consequences
- The front page's screenshots and share image were recaptured in the new look.
- The app icon is unchanged (an installed app needs one); no page shows a logo.
