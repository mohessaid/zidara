//! Evidence: what a piece of work proves about an outcome.

use serde::{Deserialize, Serialize};

use crate::ids::{ActivityId, OutcomeId, UserId};
use crate::score::Score;

/// Formative evidence guides learning; summative evidence judges it.
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[cfg_attr(feature = "ts", derive(ts_rs::TS), ts(export))]
#[serde(rename_all = "snake_case")]
pub enum EvidenceKind {
    Formative,
    Summative,
}

/// One piece of evidence. Who recorded it lives on the event that carries it.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[cfg_attr(feature = "ts", derive(ts_rs::TS), ts(export))]
pub struct Evidence {
    pub learner: UserId,
    pub outcome: OutcomeId,
    pub activity: ActivityId,
    pub score: Score,
    pub kind: EvidenceKind,
}
