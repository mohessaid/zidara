//! Events: everything that happens, with who did it and when. The log is append-only.

use serde::{Deserialize, Serialize};

use crate::activity::Response;
use crate::evidence::Evidence;
use crate::governance::Proposal;
use crate::ids::{
    ActivityId, CohortId, EventId, FrameworkId, OutcomeId, TenantId, Timestamp, UserId,
};
use crate::roles::Actor;
use crate::rules::NotifyReason;

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[cfg_attr(feature = "ts", derive(ts_rs::TS), ts(export))]
pub struct Event {
    pub id: EventId,
    pub at: Timestamp,
    pub tenant: TenantId,
    pub cohort: CohortId,
    pub actor: Actor,
    pub body: EventBody,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[cfg_attr(feature = "ts", derive(ts_rs::TS), ts(export))]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum EventBody {
    QuizSubmitted {
        learner: UserId,
        activity: ActivityId,
        #[serde(default)]
        variant: u32,
        responses: Vec<Response>,
    },
    AnswerSubmitted {
        learner: UserId,
        activity: ActivityId,
        answer: String,
    },
    EvidenceRecorded {
        evidence: Evidence,
    },
    ActivityAssigned {
        learner: UserId,
        outcome: OutcomeId,
        activity: ActivityId,
    },
    TeacherNotified {
        learner: UserId,
        outcome: OutcomeId,
        reason: NotifyReason,
    },
    ProposalMade {
        proposal: Proposal,
    },
    ProposalDecided {
        proposal: EventId,
        accept: bool,
    },
    FrameworkChanged {
        framework: FrameworkId,
    },
    /// Undo never deletes: it appends this, and folds skip the target.
    Undone {
        target: EventId,
    },
}
