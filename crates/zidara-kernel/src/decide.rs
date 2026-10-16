//! decide: the kernel's one entry point for change.
//!
//! Given who acts, what they ask and what has happened, it returns the new events or says
//! why not. It is pure: the same inputs give the same events on the server and on a phone.

use serde::{Deserialize, Serialize};

use crate::activity::{Activity, ActivityKind, Response, mark};
use crate::event::{Event, EventBody};
use crate::evidence::{Evidence, EvidenceKind};
use crate::framework::{Framework, FrameworkRegistry};
use crate::governance::{Assessment, Decision, Policy, Proposal};
use crate::ids::{
    ActivityId, CohortId, EventId, FrameworkId, IdGen, OutcomeId, TenantId, Timestamp, UserId, Uuid,
};
use crate::lesson::Lesson;
use crate::roles::{Actor, Permission, Role};
use crate::rules::{Action, evaluate};
use crate::score::Score;
use crate::state::{CohortState, ProposalStatus, fold};

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[cfg_attr(feature = "ts", derive(ts_rs::TS), ts(export))]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum Command {
    SubmitQuiz {
        activity: ActivityId,
        responses: Vec<Response>,
    },
    SubmitAnswer {
        activity: ActivityId,
        answer: String,
    },
    RecordEvidence {
        learner: UserId,
        outcome: OutcomeId,
        activity: ActivityId,
        score: Score,
        kind: EvidenceKind,
    },
    ProposeAssessment {
        answer: EventId,
        assessment: Assessment,
    },
    DecideProposal {
        proposal: EventId,
        accept: bool,
    },
    Undo {
        target: EventId,
    },
    ChangeFramework {
        framework: FrameworkId,
    },
}

/// A command as a shell sends it. `id` makes a retry harmless; `at` is when the person acted.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[cfg_attr(feature = "ts", derive(ts_rs::TS), ts(export))]
pub struct Envelope {
    pub id: Uuid,
    pub at: Timestamp,
    pub command: Command,
}

/// Everything a decision depends on besides the log. Shells build one per request.
#[derive(Clone, Copy, Debug)]
pub struct Env<'a> {
    pub now: Timestamp,
    pub tenant: TenantId,
    pub cohort: CohortId,
    pub lesson: &'a Lesson,
    pub frameworks: &'a dyn FrameworkRegistry,
    pub default_framework: &'a FrameworkId,
    pub policy: Policy,
}

#[derive(Clone, Debug, PartialEq, Eq, thiserror::Error, Serialize)]
#[cfg_attr(feature = "ts", derive(ts_rs::TS), ts(export))]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum KernelError {
    #[error("{role:?} may not {permission:?}")]
    Forbidden { role: Role, permission: Permission },
    #[error("only a person can submit work")]
    NotAPerson,
    #[error("unknown activity {activity:?}")]
    UnknownActivity { activity: ActivityId },
    #[error("activity {activity:?} does not take this kind of work")]
    WrongActivityKind { activity: ActivityId },
    #[error("unknown outcome {outcome:?}")]
    UnknownOutcome { outcome: OutcomeId },
    #[error("expected {expected} responses, got {got}")]
    Responses { expected: usize, got: usize },
    #[error("no answer {answer:?}")]
    UnknownAnswer { answer: EventId },
    #[error("answer {answer:?} already has a proposal")]
    AlreadyProposed { answer: EventId },
    #[error("no open proposal {proposal:?}")]
    NotOpen { proposal: EventId },
    #[error("event {target:?} cannot be undone")]
    NotUndoable { target: EventId },
    #[error("unknown framework {framework:?}")]
    UnknownFramework { framework: FrameworkId },
}

pub fn decide(
    actor: &Actor,
    command: Command,
    history: &[Event],
    env: &Env<'_>,
    ids: &mut dyn IdGen,
) -> Result<Vec<Event>, KernelError> {
    let mut batch = Batch {
        history,
        env,
        ids,
        events: Vec::new(),
    };
    match command {
        Command::SubmitQuiz {
            activity,
            responses,
        } => submit_quiz(&mut batch, actor, activity, responses)?,
        Command::SubmitAnswer { activity, answer } => {
            submit_answer(&mut batch, actor, activity, answer)?
        }
        Command::RecordEvidence {
            learner,
            outcome,
            activity,
            score,
            kind,
        } => record_evidence(
            &mut batch,
            actor,
            Evidence {
                learner,
                outcome,
                activity,
                score,
                kind,
            },
        )?,
        Command::ProposeAssessment { answer, assessment } => {
            propose(&mut batch, actor, answer, assessment)?
        }
        Command::DecideProposal { proposal, accept } => {
            decide_proposal(&mut batch, actor, proposal, accept)?
        }
        Command::Undo { target } => undo(&mut batch, actor, target)?,
        Command::ChangeFramework { framework } => change_framework(&mut batch, actor, framework)?,
    }
    Ok(batch.events)
}

/// The class's framework, or the default when nobody has chosen one.
pub fn current_framework<'a>(
    state: &CohortState,
    env: &Env<'a>,
) -> Result<&'a dyn Framework, KernelError> {
    let id = state.framework.as_ref().unwrap_or(env.default_framework);
    env.frameworks
        .get(id)
        .ok_or_else(|| KernelError::UnknownFramework {
            framework: id.clone(),
        })
}

/// The events one command produces, built on top of the history.
struct Batch<'a, 'i> {
    history: &'a [Event],
    env: &'a Env<'a>,
    ids: &'i mut dyn IdGen,
    events: Vec<Event>,
}

impl Batch<'_, '_> {
    fn push(&mut self, actor: Actor, body: EventBody) -> EventId {
        let id = self.ids.next_id();
        let env = self.env;
        self.events.push(Event {
            id,
            at: env.now,
            tenant: env.tenant,
            cohort: env.cohort,
            actor,
            body,
        });
        id
    }

    fn state(&self) -> CohortState {
        let all: Vec<Event> = self.history.iter().chain(&self.events).cloned().collect();
        fold(&all)
    }

    /// Records evidence, then runs the framework's rules for that learner and outcome.
    fn record(&mut self, actor: Actor, evidence: Evidence) -> Result<(), KernelError> {
        let (learner, outcome) = (evidence.learner, evidence.outcome.clone());
        self.push(actor, EventBody::EvidenceRecorded { evidence });
        let state = self.state();
        let framework = current_framework(&state, self.env)?;
        let judgement = framework.judge(&state.evidence_for(learner, &outcome));
        for action in evaluate(framework.rules(), &judgement) {
            match action {
                Action::Remediate => {
                    if let Some(activity) = self.env.lesson.remediation_for(&outcome)
                        && !state.is_assigned(learner, &outcome, &activity.id)
                    {
                        let activity = activity.id.clone();
                        self.push(
                            Actor::System,
                            EventBody::ActivityAssigned {
                                learner,
                                outcome: outcome.clone(),
                                activity,
                            },
                        );
                    }
                }
                Action::NotifyTeacher(reason) => {
                    if !state
                        .notifications
                        .contains(&(learner, outcome.clone(), reason))
                    {
                        self.push(
                            Actor::System,
                            EventBody::TeacherNotified {
                                learner,
                                outcome: outcome.clone(),
                                reason,
                            },
                        );
                    }
                }
            }
        }
        Ok(())
    }
}

fn require(actor: &Actor, permission: Permission) -> Result<(), KernelError> {
    if actor.can(permission) {
        Ok(())
    } else {
        Err(KernelError::Forbidden {
            role: actor.role(),
            permission,
        })
    }
}

fn person(actor: &Actor) -> Result<UserId, KernelError> {
    match actor {
        Actor::Human { user, .. } => Ok(*user),
        _ => Err(KernelError::NotAPerson),
    }
}

fn permission_to_record(kind: EvidenceKind) -> Permission {
    match kind {
        EvidenceKind::Formative => Permission::RecordFormative,
        EvidenceKind::Summative => Permission::RecordSummative,
    }
}

fn activity<'a>(env: &Env<'a>, id: &ActivityId) -> Result<&'a Activity, KernelError> {
    env.lesson
        .activity(id)
        .ok_or_else(|| KernelError::UnknownActivity {
            activity: id.clone(),
        })
}

fn submit_quiz(
    b: &mut Batch<'_, '_>,
    actor: &Actor,
    id: ActivityId,
    responses: Vec<Response>,
) -> Result<(), KernelError> {
    require(actor, Permission::SubmitWork)?;
    let learner = person(actor)?;
    let ActivityKind::Quiz {
        evidence: kind,
        questions,
    } = &activity(b.env, &id)?.kind
    else {
        return Err(KernelError::WrongActivityKind { activity: id });
    };
    let marks = mark(questions, &responses).map_err(|e| KernelError::Responses {
        expected: e.expected,
        got: e.got,
    })?;
    b.push(
        actor.clone(),
        EventBody::QuizSubmitted {
            learner,
            activity: id.clone(),
            responses,
        },
    );
    for (outcome, score) in marks {
        b.record(
            Actor::System,
            Evidence {
                learner,
                outcome,
                activity: id.clone(),
                score,
                kind: *kind,
            },
        )?;
    }
    Ok(())
}

fn submit_answer(
    b: &mut Batch<'_, '_>,
    actor: &Actor,
    id: ActivityId,
    answer: String,
) -> Result<(), KernelError> {
    require(actor, Permission::SubmitWork)?;
    let learner = person(actor)?;
    if !matches!(
        activity(b.env, &id)?.kind,
        ActivityKind::OpenResponse { .. }
    ) {
        return Err(KernelError::WrongActivityKind { activity: id });
    }
    b.push(
        actor.clone(),
        EventBody::AnswerSubmitted {
            learner,
            activity: id,
            answer,
        },
    );
    Ok(())
}

fn record_evidence(
    b: &mut Batch<'_, '_>,
    actor: &Actor,
    evidence: Evidence,
) -> Result<(), KernelError> {
    require(actor, permission_to_record(evidence.kind))?;
    if b.env.lesson.graph.get(&evidence.outcome).is_none() {
        return Err(KernelError::UnknownOutcome {
            outcome: evidence.outcome,
        });
    }
    activity(b.env, &evidence.activity)?;
    b.record(actor.clone(), evidence)
}

fn propose(
    b: &mut Batch<'_, '_>,
    actor: &Actor,
    answer: EventId,
    assessment: Assessment,
) -> Result<(), KernelError> {
    require(actor, Permission::ProposeAssessment)?;
    let state = b.state();
    let Some(submitted) = state.answers.get(&answer) else {
        return Err(KernelError::UnknownAnswer { answer });
    };
    if state.has_proposal_for(answer) {
        return Err(KernelError::AlreadyProposed { answer });
    }
    let ActivityKind::OpenResponse {
        evidence: kind,
        outcome,
        ..
    } = &activity(b.env, &submitted.activity)?.kind
    else {
        return Err(KernelError::WrongActivityKind {
            activity: submitted.activity.clone(),
        });
    };
    // The permission table already forbids agents summative marks; the policy never applies them.
    let decision = b.env.policy.decide(*kind, &assessment);
    let proposal = Proposal {
        answer,
        learner: submitted.learner,
        outcome: outcome.clone(),
        activity: submitted.activity.clone(),
        kind: *kind,
        assessment: assessment.clone(),
        decision,
    };
    b.push(actor.clone(), EventBody::ProposalMade { proposal });
    if decision == Decision::Apply {
        let evidence = Evidence {
            learner: submitted.learner,
            outcome: outcome.clone(),
            activity: submitted.activity.clone(),
            score: assessment.score,
            kind: *kind,
        };
        b.record(actor.clone(), evidence)?;
    }
    Ok(())
}

fn decide_proposal(
    b: &mut Batch<'_, '_>,
    actor: &Actor,
    proposal: EventId,
    accept: bool,
) -> Result<(), KernelError> {
    require(actor, Permission::DecideProposal)?;
    let state = b.state();
    let Some((open, ProposalStatus::Queued)) = state.proposals.get(&proposal) else {
        return Err(KernelError::NotOpen { proposal });
    };
    b.push(
        actor.clone(),
        EventBody::ProposalDecided { proposal, accept },
    );
    if accept {
        let evidence = Evidence {
            learner: open.learner,
            outcome: open.outcome.clone(),
            activity: open.activity.clone(),
            score: open.assessment.score,
            kind: open.kind,
        };
        b.record(actor.clone(), evidence)?;
    }
    Ok(())
}

fn undo(b: &mut Batch<'_, '_>, actor: &Actor, target: EventId) -> Result<(), KernelError> {
    require(actor, Permission::Undo)?;
    let state = b.state();
    let undoable = state.evidence.iter().any(|(id, _, _)| *id == target)
        || state.assignments.contains_key(&target);
    if !undoable {
        return Err(KernelError::NotUndoable { target });
    }
    b.push(actor.clone(), EventBody::Undone { target });
    Ok(())
}

fn change_framework(
    b: &mut Batch<'_, '_>,
    actor: &Actor,
    framework: FrameworkId,
) -> Result<(), KernelError> {
    require(actor, Permission::ChangeFramework)?;
    if b.env.frameworks.get(&framework).is_none() {
        return Err(KernelError::UnknownFramework { framework });
    }
    b.push(actor.clone(), EventBody::FrameworkChanged { framework });
    Ok(())
}
