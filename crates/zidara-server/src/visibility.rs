//! Which learner an event is about. The database shows learners only their own events and
//! cohort-wide ones, so every stored event carries this answer in `learner_id`.

use std::collections::BTreeMap;

use zidara_kernel::event::EventBody;
use zidara_kernel::ids::{EventId, UserId};

/// `known` maps earlier events to their learner, for bodies that point at another event.
pub fn learner_of(body: &EventBody, known: &BTreeMap<EventId, Option<UserId>>) -> Option<UserId> {
    match body {
        EventBody::QuizSubmitted { learner, .. }
        | EventBody::AnswerSubmitted { learner, .. }
        | EventBody::ActivityAssigned { learner, .. }
        | EventBody::TeacherNotified { learner, .. } => Some(*learner),
        EventBody::EvidenceRecorded { evidence } => Some(evidence.learner),
        EventBody::ProposalMade { proposal } => Some(proposal.learner),
        EventBody::ProposalDecided {
            proposal: target, ..
        }
        | EventBody::Undone { target } => known.get(target).copied().flatten(),
        EventBody::FrameworkChanged { .. } => None,
    }
}
