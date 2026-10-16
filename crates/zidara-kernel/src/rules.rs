//! Rules: automation a framework asks for, evaluated as a pure function.

use serde::{Deserialize, Serialize};

use crate::framework::Judgement;
use crate::score::Score;

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[cfg_attr(feature = "ts", derive(ts_rs::TS), ts(export))]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum Rule {
    /// Below `below` after `after_attempts` tries: offer remediation and tell the teacher.
    Remediate { below: Score, after_attempts: u32 },
    /// A band below `rank`: tell the teacher early.
    NotifyBelowRank { rank: u8 },
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[cfg_attr(feature = "ts", derive(ts_rs::TS), ts(export))]
#[serde(rename_all = "snake_case")]
pub enum NotifyReason {
    Struggling,
    BelowExpected,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Action {
    Remediate,
    NotifyTeacher(NotifyReason),
}

pub fn evaluate(rules: &[Rule], judgement: &Judgement) -> Vec<Action> {
    let mut actions = Vec::new();
    for rule in rules {
        match *rule {
            Rule::Remediate {
                below,
                after_attempts,
            } => {
                if judgement.attempts >= after_attempts
                    && judgement.score.is_some_and(|s| s < below)
                {
                    actions.push(Action::Remediate);
                    actions.push(Action::NotifyTeacher(NotifyReason::Struggling));
                }
            }
            Rule::NotifyBelowRank { rank } => {
                if judgement.band.as_ref().is_some_and(|b| b.rank < rank) {
                    actions.push(Action::NotifyTeacher(NotifyReason::BelowExpected));
                }
            }
        }
    }
    actions
}
