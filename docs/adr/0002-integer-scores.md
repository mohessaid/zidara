# 0002. Integer scores in basis points

Date: 2026-10-14. Status: accepted.

## Context
Judgement must give the same answer on the server, in the browser and on a phone. Floating
point rounding differs across platforms and compilers often enough to matter at a boundary
("is 0.8 mastered?").

## Decision
`Score` is a `u16` from 0 to 10 000 basis points. Fractions and means round half up with
integer arithmetic in `u64`. Constants are built with `Score::of`, a `const fn` that fails at
compile time when out of range.

## Consequences
- Boundaries are exact: 8 000 is mastered, 7 999 is not, everywhere.
- 0.01% resolution is finer than any grading scheme needs.
