//! The fold: what is true now, computed from the log.

use std::collections::{BTreeMap, BTreeSet};

use serde::{Deserialize, Serialize};

use crate::event::{Event, EventBody};
use crate::evidence::Evidence;
use crate::governance::{Decision, Proposal};
use crate::ids::{ActivityId, EventId, FrameworkId, OutcomeId, UserId};
use crate::roles::Actor;
use crate::rules::NotifyReason;

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ProposalStatus {
    Applied,
    Queued,
    Accepted,
    Rejected,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Answer {
    pub learner: UserId,
    pub activity: ActivityId,
    pub text: String,
}

#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct CohortState {
    pub framework: Option<FrameworkId>,
    /// Oldest first, with who recorded each piece.
    pub evidence: Vec<(EventId, Actor, Evidence)>,
    pub answers: BTreeMap<EventId, Answer>,
    pub proposals: BTreeMap<EventId, (Proposal, ProposalStatus)>,
    pub assignments: BTreeMap<EventId, (UserId, OutcomeId, ActivityId)>,
    pub notifications: BTreeSet<(UserId, OutcomeId, NotifyReason)>,
}

impl CohortState {
    pub fn evidence_for(&self, learner: UserId, outcome: &OutcomeId) -> Vec<&Evidence> {
        self.evidence
            .iter()
            .map(|(_, _, e)| e)
            .filter(|e| e.learner == learner && &e.outcome == outcome)
            .collect()
    }

    pub fn is_assigned(&self, learner: UserId, outcome: &OutcomeId, activity: &ActivityId) -> bool {
        self.assignments
            .values()
            .any(|(l, o, a)| *l == learner && o == outcome && a == activity)
    }

    pub fn has_proposal_for(&self, answer: EventId) -> bool {
        self.proposals.values().any(|(p, _)| p.answer == answer)
    }
}

/// Folds a log into state. Events are ordered by `at` (a stable sort, so events from one
/// command keep their order), and undone events are skipped.
pub fn fold(events: &[Event]) -> CohortState {
    let mut ordered: Vec<&Event> = events.iter().collect();
    ordered.sort_by_key(|e| e.at);
    let undone: BTreeSet<EventId> = ordered
        .iter()
        .filter_map(|e| match e.body {
            EventBody::Undone { target } => Some(target),
            _ => None,
        })
        .collect();
    let mut state = CohortState::default();
    for event in ordered.into_iter().filter(|e| !undone.contains(&e.id)) {
        match &event.body {
            EventBody::EvidenceRecorded { evidence } => {
                state
                    .evidence
                    .push((event.id, event.actor.clone(), evidence.clone()));
            }
            EventBody::AnswerSubmitted {
                learner,
                activity,
                answer,
            } => {
                state.answers.insert(
                    event.id,
                    Answer {
                        learner: *learner,
                        activity: activity.clone(),
                        text: answer.clone(),
                    },
                );
            }
            EventBody::ProposalMade { proposal } => {
                let status = match proposal.decision {
                    Decision::Apply => ProposalStatus::Applied,
                    Decision::Queue => ProposalStatus::Queued,
                };
                state.proposals.insert(event.id, (proposal.clone(), status));
            }
            EventBody::ProposalDecided { proposal, accept } => {
                if let Some(entry) = state.proposals.get_mut(proposal) {
                    entry.1 = if *accept {
                        ProposalStatus::Accepted
                    } else {
                        ProposalStatus::Rejected
                    };
                }
            }
            EventBody::ActivityAssigned {
                learner,
                outcome,
                activity,
            } => {
                state
                    .assignments
                    .insert(event.id, (*learner, outcome.clone(), activity.clone()));
            }
            EventBody::TeacherNotified {
                learner,
                outcome,
                reason,
            } => {
                state
                    .notifications
                    .insert((*learner, outcome.clone(), *reason));
            }
            EventBody::FrameworkChanged { framework } => state.framework = Some(framework.clone()),
            EventBody::QuizSubmitted { .. } | EventBody::Undone { .. } => {}
        }
    }
    state
}
