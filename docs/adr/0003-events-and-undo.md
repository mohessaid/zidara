# 0003. Append-only events; undo appends

Date: 2026-10-14. Status: accepted.

## Context
The proposal promises that every action, an agent's included, is logged and reversible, and
that the evidence log is the analytics (xAPI-shaped).

## Decision
Everything that happens is an `Event` with an actor, a tenant, a cohort and a time. Nothing is
updated or deleted. Undo appends `Undone { target }`; the fold skips undone events. Only
evidence and assignments can be undone. Folds order events by `at` with a stable sort, so
events from one command keep their order and late-synced work lands where it happened.

## Consequences
- The audit trail is complete by construction.
- The database grants the app role INSERT and SELECT on events, nothing else (M2).
- An undone undo is not supported; `decide` refuses to undo anything but evidence and
  assignments.
