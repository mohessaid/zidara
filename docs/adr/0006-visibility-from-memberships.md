# 0006. The database derives what each person sees from memberships

Date: 2026-10-15. Status: accepted. Supersedes the `app.cohort_ids` setting in the spec.

## Context
The spec had the server set `app.tenant_id` and `app.cohort_ids` on each request. That makes
the server the judge of which cohorts a request may touch: a bug that sets the wrong list
leaks a cohort, and the database cannot tell.

## Decision
- The server sets one thing, `app.user_id`, with `SET LOCAL` in the request's transaction.
- Policies derive the rest from `memberships`. Two `security definer` functions,
  `private.my_cohorts()` and `private.overseen_cohorts()`, read memberships without RLS, so the
  policy on `memberships` itself does not recurse.
- Learners see their own events (`learner_id = me`) and cohort-wide ones (`learner_id is
  null`). Teachers and the agent see their cohorts. Learners see their own membership and user
  rows only: no classmates' names.
- Settings are read through `nullif(current_setting(..., true), '')`, because a pooled
  connection keeps an empty setting after `SET LOCAL` ends; the policy then yields no rows.
- The app role owns nothing, has no `BYPASSRLS`, and has `SELECT, INSERT` only on the log.

## Consequences
- A server bug can at worst act as the wrong *user*, not widen what a user may see.
- Proven by 9 tests on PostgreSQL 16; making the app role the table owner fails 5 of them.
- Every query runs inside a transaction that starts with `begin_as(user)`.
