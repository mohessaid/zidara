//! Governance: how an agent's marking is weighed before it touches a learner's record.

use serde::{Deserialize, Serialize};

/// One thing a good answer mentions; any of the terms counts.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct Criterion {
    pub any_of: Vec<String>,
    pub weight: u16,
}

/// What a teacher looks for in an open answer.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct Rubric {
    pub criteria: Vec<Criterion>,
    pub min_words: u16,
}
