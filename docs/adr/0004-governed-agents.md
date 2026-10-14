# 0004. AI agents are governed actors

Date: 2026-10-14. Status: accepted.

## Context
Solve's AI focus asks for AI that deepens learning. Ours must act inside rules people approve,
never beside them.

## Decision
- An agent is an `Actor::Agent` with a `Role`, checked against the same permission table as
  people. `AssessmentAgent` may propose assessments and record formative evidence, nothing else.
- The kernel never calls a model. The agent sends `ProposeAssessment { answer, assessment }`
  like any other command; the model (Claude, or the deterministic rubric) runs in a shell.
- `Policy::decide` applies a proposal only when it is formative and its confidence is at least
  8 500 basis points; everything else is queued for a teacher.
- There is no second runtime permission check when a proposal is applied: the policy never
  applies summative work and the permission table forbids it. A second check would be
  unreachable code, which the coverage and mutation gates rightly reject. The permission-table
  test pins the rule instead.

## Consequences
- Swapping the model changes no kernel code.
- Teachers see the queue and every agent action in the studio view, and can undo any of them.
