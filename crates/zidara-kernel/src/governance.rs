//! Governance: how an agent's marking is weighed before it touches a learner's record.

use serde::{Deserialize, Serialize};

use crate::evidence::EvidenceKind;
use crate::ids::{ActivityId, EventId, OutcomeId, UserId};
use crate::score::Score;

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

/// A mark with how sure the marker is.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct Assessment {
    pub score: Score,
    pub confidence: Score,
    pub rationale: String,
}

/// All criteria met, or none: easy to call.
pub const CLEAR: Score = Score::of(9_000);
/// Some criteria met: a teacher should look.
pub const MIXED: Score = Score::of(6_000);
/// Too few words to judge.
pub const TOO_SHORT: Score = Score::of(4_000);

/// Deterministic first-pass marking against a rubric. Used offline, in tests, and as the
/// fallback when no model is configured.
pub fn assess(rubric: &Rubric, answer: &str) -> Assessment {
    let text = normalize(answer);
    let total: u32 = rubric.criteria.iter().map(|c| u32::from(c.weight)).sum();
    let met: Vec<&Criterion> = rubric
        .criteria
        .iter()
        .filter(|c| c.any_of.iter().any(|term| text.contains(&normalize(term))))
        .collect();
    let matched: u32 = met.iter().map(|c| u32::from(c.weight)).sum();
    let score = Score::from_fraction(matched, total).unwrap_or(Score::ZERO);
    let words = answer.split_whitespace().count();
    let confidence = if words < usize::from(rubric.min_words) {
        TOO_SHORT
    } else if matched == 0 || matched == total {
        CLEAR
    } else {
        MIXED
    };
    let rationale = format!(
        "met {} of {} criteria in {} words",
        met.len(),
        rubric.criteria.len(),
        words
    );
    Assessment {
        score,
        confidence,
        rationale,
    }
}

/// Lower case, without Arabic short vowels (U+064B–U+0652) or tatweel (U+0640).
fn normalize(text: &str) -> String {
    text.to_lowercase()
        .chars()
        .filter(|c| !matches!(c, '\u{064B}'..='\u{0652}' | '\u{0640}'))
        .collect()
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum Decision {
    Apply,
    Queue,
}

/// When an agent's mark may be applied without a person.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct Policy {
    pub auto_apply_at: Score,
}

impl Policy {
    pub const STANDARD: Policy = Policy {
        auto_apply_at: Score::of(8_500),
    };

    /// Formative and confident: apply. Anything else, summative above all: a teacher decides.
    pub fn decide(&self, kind: EvidenceKind, assessment: &Assessment) -> Decision {
        if kind == EvidenceKind::Formative && assessment.confidence >= self.auto_apply_at {
            Decision::Apply
        } else {
            Decision::Queue
        }
    }
}

/// An agent's mark for one answer, with what the policy decided.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct Proposal {
    pub answer: EventId,
    pub learner: UserId,
    pub outcome: OutcomeId,
    pub activity: ActivityId,
    pub kind: EvidenceKind,
    pub assessment: Assessment,
    pub decision: Decision,
}
