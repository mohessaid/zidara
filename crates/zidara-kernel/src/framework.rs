//! The framework contract: judgement and progression. Each pedagogy implements it once.

use std::collections::BTreeMap;
use std::fmt::Debug;

use serde::{Deserialize, Serialize};

use crate::evidence::Evidence;
use crate::ids::{FrameworkId, OutcomeId, Timestamp};
use crate::outcome::OutcomeGraph;
use crate::rules::Rule;
use crate::score::Score;

/// Where a learner stands on one outcome. Experiences decide how to draw it.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum OutcomeState {
    NotStarted,
    InProgress,
    Achieved,
    NeedsSupport,
}

/// A named level. `rank` counts from 0 (lowest) to `of - 1`; a label is a translation key.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct Band {
    pub label: String,
    pub rank: u8,
    pub of: u8,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct Judgement {
    pub state: OutcomeState,
    pub score: Option<Score>,
    pub band: Option<Band>,
    pub attempts: u32,
}

impl Judgement {
    pub const NOT_STARTED: Judgement = Judgement {
        state: OutcomeState::NotStarted,
        score: None,
        band: None,
        attempts: 0,
    };
}

/// Why an outcome is not open yet.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum LockReason {
    Prerequisite { outcome: OutcomeId },
    OpensAt { at: Timestamp },
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct Availability {
    pub outcome: OutcomeId,
    pub lock: Option<LockReason>,
}

/// What progression may look at.
#[derive(Clone, Copy, Debug)]
pub struct ProgressContext<'a> {
    pub graph: &'a OutcomeGraph,
    pub judgements: &'a BTreeMap<OutcomeId, Judgement>,
    pub now: Timestamp,
}

impl ProgressContext<'_> {
    pub fn state_of(&self, outcome: &OutcomeId) -> OutcomeState {
        self.judgements
            .get(outcome)
            .map_or(OutcomeState::NotStarted, |j| j.state)
    }
}

/// A pedagogy made explicit. Data (thresholds, bands, calendars) lives in the implementation;
/// the kernel only calls these four functions.
pub trait Framework: Debug + Send + Sync {
    fn id(&self) -> FrameworkId;
    /// Judges one learner on one outcome from that outcome's evidence, oldest first.
    fn judge(&self, evidence: &[&Evidence]) -> Judgement;
    /// For each outcome in graph order: open, or why not.
    fn availability(&self, ctx: &ProgressContext<'_>) -> Vec<Availability>;
    /// Automation the framework asks for.
    fn rules(&self) -> &[Rule];
}

/// Finds frameworks by id. Shells hold the registry; the kernel only reads it.
pub trait FrameworkRegistry: Debug + Send + Sync {
    fn get(&self, id: &FrameworkId) -> Option<&dyn Framework>;
}
