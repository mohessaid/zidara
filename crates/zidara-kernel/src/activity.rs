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
    /// How it looks and how it is answered. Experiences draw it; the lesson only checks that
    /// it can be drawn and answered.
    #[serde(default)]
    pub look: Look,
}

/// The largest number of parts a picture or an input is cut into: more no longer fits a phone.
pub const MAX_PARTS: u32 = 24;

#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
#[cfg_attr(feature = "ts", derive(ts_rs::TS), ts(export))]
pub struct Look {
    /// A picture beside the prompt.
    #[serde(default)]
    pub visual: Option<Visual>,
    /// How a fraction is given.
    #[serde(default)]
    pub input: Input,
    /// Shown once the learner has answered: why the answer is what it is.
    #[serde(default)]
    pub explain: Option<Localized>,
}

/// A whole cut into equal parts, some of them coloured.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[cfg_attr(feature = "ts", derive(ts_rs::TS), ts(export))]
pub struct Part {
    pub parts: u32,
    pub shaded: u32,
}

impl Part {
    fn drawable(self) -> bool {
        (1..=MAX_PARTS).contains(&self.parts) && self.shaded <= self.parts
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[cfg_attr(feature = "ts", derive(ts_rs::TS), ts(export))]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum Visual {
    Bar {
        parts: u32,
        shaded: u32,
    },
    /// A round whole: a pizza, a cake.
    Circle {
        parts: u32,
        shaded: u32,
    },
    /// Two bars of the same length, one above the other, to compare.
    Pair {
        top: Part,
        bottom: Part,
    },
}

impl Visual {
    pub fn drawable(self) -> bool {
        match self {
            Visual::Bar { parts, shaded } | Visual::Circle { parts, shaded } => {
                Part { parts, shaded }.drawable()
            }
            Visual::Pair { top, bottom } => top.drawable() && bottom.drawable(),
        }
    }
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
#[cfg_attr(feature = "ts", derive(ts_rs::TS), ts(export))]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum Input {
    /// Type the top and the bottom number.
    #[default]
    Keypad,
    /// Colour parts of a bar already cut into `parts`.
    Shade { parts: u32 },
    /// Choose how many parts to cut (up to `max`), then colour them.
    Cut { max: u32 },
}

impl Input {
    /// Whether a learner can give the expected answer this way.
    pub fn can_answer(self, expected: &Expected) -> bool {
        let limit = match self {
            Input::Keypad => return true,
            Input::Shade { parts } => parts,
            Input::Cut { max } => max,
        };
        let (num, den, exact) = match expected {
            Expected::Choice { .. } => return false,
            Expected::Exactly { num, den } => (*num, *den, true),
            Expected::EquivalentTo { num, den } => (*num, *den, false),
        };
        // A bar shows at most one whole, cut into at most MAX_PARTS.
        if den == 0 || num > den || limit > MAX_PARTS {
            return false;
        }
        // The fewest parts that can show the answer.
        let fewest = if exact { den } else { den / gcd(num, den) };
        match self {
            // Every part count that shows it is a multiple of the fewest (exactly: only that).
            Input::Shade { .. } => {
                limit >= 1
                    && if exact {
                        limit == den
                    } else {
                        limit % fewest == 0
                    }
            }
            _ => fewest <= limit,
        }
    }
}

fn gcd(a: u32, b: u32) -> u32 {
    if b == 0 { a } else { gcd(b, a % b) }
}

/// Something to play with before practising. It records no evidence.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[cfg_attr(feature = "ts", derive(ts_rs::TS), ts(export))]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum Toy {
    /// A paper strip folded in half up to `folds` times: 1/2 = 2/4 = 4/8.
    Fold { folds: u32 },
    /// The same whole cut differently on each row, to compare and match.
    Wall { rows: Vec<u32> },
}

impl Toy {
    pub fn drawable(&self) -> bool {
        match self {
            Toy::Fold { folds } => (1..=4).contains(folds),
            Toy::Wall { rows } => !rows.is_empty() && rows.iter().all(|r| (1..=12).contains(r)),
        }
    }
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
        /// May contain `{locale}`: one rendering per language.
        src: String,
        seconds: u32,
        /// A text alternative for learners who cannot see the video (WCAG 1.2.1).
        #[serde(default)]
        transcript: Option<Localized>,
    },
    Reading {
        body: Localized,
        #[serde(default)]
        audio: Option<String>,
    },
    Quiz {
        evidence: EvidenceKind,
        questions: Vec<Question>,
        /// More versions of the same questions; attempts rotate through them (version 0 is
        /// `questions`), so a retake is not a memory test.
        #[serde(default)]
        variants: Vec<Vec<Question>>,
    },
    OpenResponse {
        evidence: EvidenceKind,
        outcome: OutcomeId,
        prompt: Localized,
        rubric: Rubric,
    },
    Explore {
        body: Localized,
        toy: Toy,
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

/// The questions of version `variant` (0 is the original), if the quiz has it.
pub fn version<'a>(
    questions: &'a [Question],
    variants: &'a [Vec<Question>],
    variant: u32,
) -> Option<&'a [Question]> {
    match variant {
        0 => Some(questions),
        n => variants.get(n as usize - 1).map(Vec::as_slice),
    }
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
