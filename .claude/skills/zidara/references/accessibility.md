# Accessibility: WCAG 2.2 AA is the floor

Distilled from Addy Osmani's accessibility skill and the game-UI skill's accessibility rules,
narrowed to Zidara's screens: Quest, Grid, Teacher studio and the lesson player.

## Workflow

1. Run the automated audits on every screen: `@axe-core/playwright` in E2E (0 violations) and
   Lighthouse accessibility (100) on the economy profile.
2. Use the failing nodes to locate the component. Don't search the codebase for generic
   patterns.
3. Check the accessibility tree: names, roles, states, landmarks, headings. Then exercise the
   flow by keyboard and with a screen reader (TalkBack for Android).
4. Fix the source, then rerun the same audit.

## Numbers

| Check | Requirement |
|---|---|
| Text contrast | 4.5:1 (normal), 3:1 (large: ≥ 18.66 px, or ≥ 14 px bold) |
| UI components, icons, stars, map nodes | 3:1 against what is next to them |
| Touch targets | WCAG 2.2 minimum 24 × 24 CSS px; **we use 48 × 48** for children on phones |
| Text size | 16 px minimum for children's screens, 18 px body, 24 px for key game info |
| Zoom | usable at 200%, with no horizontal scroll at 320 CSS px wide |
| Motion | respect `prefers-reduced-motion`: no parallax, shake or large zooms; keep fades |
| Flashing | never more than 3 flashes per second |

## Rules for our screens

- **Never colour alone.** A state is colour + shape + text: a locked level has a padlock icon
  and the label "locked"; stars are drawn shapes and also announced ("2 of 3 stars").
- The canvas is decorative. Every map node has a DOM twin (a button with a name, its state
  and its lock reason) positioned over it, so keyboard and screen-reader users play the same
  game.
- Focus is always visible (a 2 px outline at 3:1 or better), never hidden behind sticky
  headers (2.4.11), in reading order (mirrored in RTL).
- Interactions never need dragging (2.5.7). Every drag has a tap alternative.
- Live regions announce new stars, unlocked levels and the agent's feedback. Use `polite`
  except for errors.
- Video has captions in each language. Readings have audio, and the text is the transcript.
- Set `lang` on the page and on any passage in another language (`<span lang="fr">`).
- Sign-in for children needs no memory or puzzle test (3.3.8): picture passwords or a
  teacher-issued code.
- Help sits in the same place on every screen (3.2.6). Never ask for the same information
  twice (3.3.7).
