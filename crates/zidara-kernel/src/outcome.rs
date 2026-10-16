//! Outcomes: what a learner should be able to do, as a graph of prerequisites.

use std::collections::BTreeSet;

use serde::{Deserialize, Serialize};

use crate::ids::OutcomeId;
use crate::locale::Localized;

/// One outcome, in the shape CASE frameworks use (id, title, what it builds on).
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[cfg_attr(feature = "ts", derive(ts_rs::TS), ts(export))]
pub struct Outcome {
    pub id: OutcomeId,
    pub title: Localized,
    #[serde(default)]
    pub prerequisites: Vec<OutcomeId>,
}

#[derive(Clone, Debug, PartialEq, Eq, thiserror::Error)]
pub enum GraphError {
    #[error("outcome {0:?} appears twice")]
    Duplicate(OutcomeId),
    #[error("outcome {outcome:?} needs unknown outcome {missing:?}")]
    UnknownPrerequisite {
        outcome: OutcomeId,
        missing: OutcomeId,
    },
    #[error("outcome {:?} is part of a prerequisite cycle", .0.as_str())]
    Cycle(OutcomeId),
}

/// Outcomes ordered so every prerequisite comes before the outcomes that need it.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct OutcomeGraph {
    outcomes: Vec<Outcome>,
}

impl OutcomeGraph {
    pub fn new(outcomes: Vec<Outcome>) -> Result<Self, GraphError> {
        let mut ids = BTreeSet::new();
        for outcome in &outcomes {
            if !ids.insert(outcome.id.clone()) {
                return Err(GraphError::Duplicate(outcome.id.clone()));
            }
        }
        for outcome in &outcomes {
            if let Some(missing) = outcome.prerequisites.iter().find(|p| !ids.contains(*p)) {
                return Err(GraphError::UnknownPrerequisite {
                    outcome: outcome.id.clone(),
                    missing: missing.clone(),
                });
            }
        }
        // Repeatedly take the first outcome whose prerequisites are all placed.
        let mut placed = BTreeSet::new();
        let mut remaining = outcomes;
        let mut ordered = Vec::with_capacity(remaining.len());
        while !remaining.is_empty() {
            let Some(next) = remaining
                .iter()
                .position(|o| o.prerequisites.iter().all(|p| placed.contains(p)))
            else {
                return Err(GraphError::Cycle(remaining[0].id.clone()));
            };
            let outcome = remaining.remove(next);
            placed.insert(outcome.id.clone());
            ordered.push(outcome);
        }
        Ok(Self { outcomes: ordered })
    }

    pub fn get(&self, id: &OutcomeId) -> Option<&Outcome> {
        self.outcomes.iter().find(|o| &o.id == id)
    }

    pub fn iter(&self) -> impl Iterator<Item = &Outcome> {
        self.outcomes.iter()
    }

    pub fn len(&self) -> usize {
        self.outcomes.len()
    }

    pub fn is_empty(&self) -> bool {
        self.outcomes.is_empty()
    }
}
