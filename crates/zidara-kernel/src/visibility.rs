//! Who may read which event. A learner reads their own events and cohort-wide ones; teachers
//! and agents read the whole cohort. The server enforces this in the database (row-level
//! security on each event's learner); shells without a database use `visible_to`.

use std::collections::BTreeMap;

use crate::event::{Event, EventBody};
use crate::ids::{EventId, UserId};
use crate::roles::Role;

/// Which learner an event is about. `known` maps earlier events to their learner, for bodies
/// that point at another event.
pub fn learner_of(body: &EventBody, known: &BTreeMap<EventId, Option<UserId>>) -> Option<UserId> {
    match body {
        EventBody::QuizSubmitted { learner, .. }
        | EventBody::AnswerSubmitted { learner, .. }
        | EventBody::ActivityAssigned { learner, .. }
        | EventBody::TeacherNotified { learner, .. } => Some(*learner),
        EventBody::EvidenceRecorded { evidence, .. } => Some(evidence.learner),
        EventBody::ProposalMade { proposal } => Some(proposal.learner),
        EventBody::ProposalDecided {
            proposal: target, ..
        }
        | EventBody::Undone { target } => known.get(target).copied().flatten(),
        EventBody::FrameworkChanged { .. } => None,
    }
}

/// The events of a log, in order, that `viewer` may read in `role`.
pub fn visible_to(events: &[Event], viewer: UserId, role: Role) -> Vec<&Event> {
    if role != Role::Learner {
        return events.iter().collect();
    }
    let mut known = BTreeMap::new();
    events
        .iter()
        .filter(|event| {
            let learner = learner_of(&event.body, &known);
            known.insert(event.id, learner);
            learner.is_none_or(|l| l == viewer)
        })
        .collect()
}
