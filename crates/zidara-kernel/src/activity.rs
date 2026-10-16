//! Activities: what a learner does, and how a quiz is marked.

use serde::{Deserialize, Serialize};

use crate::evidence::EvidenceKind;
use crate::governance::Rubric;
use crate::ids::{ActivityId, OutcomeId};
use crate::locale::Localized;
use crate::score::Score;

/// The right answer to a question.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[cfg_attr(feature = "ts", derive(ts_rs::TS), ts(export))]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum Expected {
    Choice {
        options: Vec<Localized>,
        correct: u32,
    },
    /// This fraction in exactly this form (e.g. 2/4, not 1/2).
    Exactly { num: u32, den: u32 },
    /// Any fraction equal to this one.
    EquivalentTo { num: u32, den: u32 },
}

/// A learner's answer to a question.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[cfg_attr(feature = "ts", derive(ts_rs::TS), ts(export))]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum Response {
    Choice { index: u32 },
    Fraction { num: u32, den: u32 },
}

impl Expected {
    pub fn accepts(&self, response: &Response) -> bool {
        match (self, response) {
            (Expected::Choice { correct, .. }, Response::Choice { index }) => index == correct,
            (Expected::Exactly { num, den }, Response::Fraction { num: n, den: d }) => {
                n == num && d == den
            }
            (Expected::EquivalentTo { num, den }, Response::Fraction { num: n, den: d }) => {
                *d != 0 && u64::from(*n) * u64::from(*den) == u64::from(*num) * u64::from(*d)
            }
            _ => false,
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[cfg_attr(feature = "ts", derive(ts_rs::TS), ts(export))]
pub struct Question {
    pub id: String,
    pub outcome: OutcomeId,
    pub prompt: Localized,
    pub expected: Expected,
}

/// Core activities are the lesson; remediation is offered when a learner struggles.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
#[cfg_attr(feature = "ts", derive(ts_rs::TS), ts(export))]
#[serde(rename_all = "snake_case")]
pub enum ActivityRole {
    #[default]
    Core,
    Remediation,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[cfg_attr(feature = "ts", derive(ts_rs::TS), ts(export))]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum ActivityKind {
    Video {
        src: String,
        seconds: u32,
    },
    Reading {
        body: Localized,
        #[serde(default)]
        audio: Option<String>,
    },
    Quiz {
        evidence: EvidenceKind,
        questions: Vec<Question>,
    },
    OpenResponse {
        evidence: EvidenceKind,
        outcome: OutcomeId,
        prompt: Localized,
        rubric: Rubric,
    },
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[cfg_attr(feature = "ts", derive(ts_rs::TS), ts(export))]
pub struct Activity {
    pub id: ActivityId,
    pub title: Localized,
    pub outcomes: Vec<OutcomeId>,
    #[serde(default)]
    pub role: ActivityRole,
    pub kind: ActivityKind,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, thiserror::Error)]
#[error("expected {expected} responses, got {got}")]
pub struct MarkError {
    pub expected: usize,
    pub got: usize,
}

/// Marks a quiz: one score per outcome, in the order outcomes first appear.
pub fn mark(
    questions: &[Question],
    responses: &[Response],
) -> Result<Vec<(OutcomeId, Score)>, MarkError> {
    if questions.len() != responses.len() {
        return Err(MarkError {
            expected: questions.len(),
            got: responses.len(),
        });
    }
    let mut tally: Vec<(OutcomeId, u32, u32)> = Vec::new();
    for (question, response) in questions.iter().zip(responses) {
        let right = u32::from(question.expected.accepts(response));
        match tally
            .iter_mut()
            .find(|(outcome, _, _)| *outcome == question.outcome)
        {
            Some(entry) => {
                entry.1 += right;
                entry.2 += 1;
            }
            None => tally.push((question.outcome.clone(), right, 1)),
        }
    }
    // right <= total and total >= 1 by construction, so the fraction is always valid.
    Ok(tally
        .into_iter()
        .map(|(outcome, right, total)| {
            (
                outcome,
                Score::from_fraction(right, total).unwrap_or(Score::ZERO),
            )
        })
        .collect())
}
