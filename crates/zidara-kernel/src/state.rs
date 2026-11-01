//! The fold: what is true now, computed from the log.

use std::collections::{BTreeMap, BTreeSet};

use serde::{Deserialize, Serialize};

use crate::activity::Response;
use crate::event::{Event, EventBody};
use crate::evidence::Evidence;
use crate::governance::{Decision, Proposal};
use crate::ids::{ActivityId, EventId, FrameworkId, OutcomeId, UserId};
use crate::roles::Actor;
use crate::rules::NotifyReason;

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[cfg_attr(feature = "ts", derive(ts_rs::TS), ts(export))]
#[serde(rename_all = "snake_case")]
pub enum ProposalStatus {
    Applied,
    Queued,
    Accepted,
    Rejected,
    /// The mark it led to was undone by a teacher.
    Undone,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Answer {
    pub learner: UserId,
    pub activity: ActivityId,
    pub text: String,
}

/// Each learner's answer to each question of a live round, and whether it was right.
pub type LiveAnswers = BTreeMap<(UserId, u32), (Response, bool)>;

/// The live round running in a class.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Challenge {
    /// The event that started it.
    pub id: EventId,
    pub activity: ActivityId,
    pub question: u32,
    pub answers: LiveAnswers,
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
    /// Assignments a teacher undid: the rules do not assign them again.
    pub dismissed: BTreeSet<(UserId, OutcomeId, ActivityId)>,
    pub challenge: Option<Challenge>,
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
    fold_refs(events)
}

/// `fold` over borrowed events from anywhere (e.g. stored history chained with new events), so
/// nothing is copied: cloning a class's whole log for every command dominated server load.
pub fn fold_refs<'a>(events: impl IntoIterator<Item = &'a Event>) -> CohortState {
    let mut ordered: Vec<&Event> = events.into_iter().collect();
    ordered.sort_by_key(|e| e.at);
    let undone: BTreeSet<EventId> = ordered
        .iter()
        .filter_map(|e| match e.body {
            EventBody::Undone { target } => Some(target),
            _ => None,
        })
        .collect();
    let mut state = CohortState::default();
    let mut unmarked = BTreeSet::new();
    // Live answers by round: an answer may sort before its round's start.
    let mut live: BTreeMap<EventId, LiveAnswers> = BTreeMap::new();
    for event in ordered.iter().filter(|e| undone.contains(&e.id)) {
        match &event.body {
            EventBody::ActivityAssigned {
                learner,
                outcome,
                activity,
            } => {
                state
                    .dismissed
                    .insert((*learner, outcome.clone(), activity.clone()));
            }
            EventBody::EvidenceRecorded {
                proposal: Some(proposal),
                ..
            } => {
                unmarked.insert(*proposal);
            }
            _ => {}
        }
    }
    for event in ordered.into_iter().filter(|e| !undone.contains(&e.id)) {
        match &event.body {
            EventBody::EvidenceRecorded { evidence, .. } => {
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
            EventBody::ChallengeStarted { activity } => {
                state.challenge = Some(Challenge {
                    id: event.id,
                    activity: activity.clone(),
                    question: 0,
                    answers: BTreeMap::new(),
                });
            }
            EventBody::ChallengeAdvanced {
                challenge,
                question,
            } => {
                if let Some(running) = state.challenge.as_mut().filter(|c| c.id == *challenge) {
                    running.question = *question;
                }
            }
            EventBody::ChallengeAnswered {
                challenge,
                learner,
                question,
                response,
                correct,
            } => {
                live.entry(*challenge)
                    .or_default()
                    .insert((*learner, *question), (response.clone(), *correct));
            }
            EventBody::ChallengeEnded { challenge } => {
                if state.challenge.as_ref().is_some_and(|c| c.id == *challenge) {
                    state.challenge = None;
                }
            }
            EventBody::QuizSubmitted { .. } | EventBody::Undone { .. } => {}
        }
    }
    if let Some(running) = state.challenge.as_mut() {
        running.answers = live.remove(&running.id).unwrap_or_default();
    }
    for proposal in unmarked {
        if let Some(entry) = state.proposals.get_mut(&proposal) {
            entry.1 = ProposalStatus::Undone;
        }
    }
    state
}
