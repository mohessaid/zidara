# 0005. Offline sync replays commands

Date: 2026-10-14. Status: accepted.

## Context
Learners work offline. Merging edits is hard; trusting a client's events is unsafe.

## Decision
Offline, the shell runs commands through the wasm kernel against its local log and keeps each
in an outbox as an `Envelope { id, at, command }`. Back online, the server replays each envelope
through the same `decide`, with the same checks. It stores applied envelope ids and skips
repeats. `at` is when the learner acted, capped at the server's clock.

## Consequences
- A client can only ask, so it cannot forge evidence.
- Phone and server compute the same progress; an acceptance story checks it.
- Server-side events get server ids; the phone replaces its provisional log after sync (M3).
