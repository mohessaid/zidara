//! Cambridge-style teaching: a scheme of work opens outcomes on a calendar, and summative
//! checkpoints are judged against grade boundaries. Practice shows a band but never judges.

use std::collections::BTreeMap;

use serde::{Deserialize, Serialize};
use zidara_kernel::evidence::{Evidence, EvidenceKind};
use zidara_kernel::framework::{
    Availability, Band, Framework, Judgement, LockReason, OutcomeState, ProgressContext,
};
use zidara_kernel::ids::{FrameworkId, OutcomeId, Timestamp};
use zidara_kernel::rules::Rule;
use zidara_kernel::score::Score;

pub const ID: &str = "cambridge-style";

/// A grade boundary: scores at or above `min` earn `label` (a translation key).
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct Boundary {
    pub label: String,
    pub min: Score,
}

#[derive(Clone, Debug, PartialEq, Eq, thiserror::Error)]
pub enum BoundaryError {
    #[error("at least one boundary is needed")]
    Empty,
    #[error("the first boundary must start at zero")]
    FirstNotZero,
    #[error("boundaries must rise strictly")]
    NotRising,
    #[error("expected rank {0} is past the last boundary")]
    ExpectedOutOfRange(u8),
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct CambridgeStyle {
    boundaries: Vec<Boundary>,
    expected_rank: u8,
    opens_at: BTreeMap<OutcomeId, Timestamp>,
    rules: Vec<Rule>,
}

impl CambridgeStyle {
    pub fn new(
        boundaries: Vec<Boundary>,
        expected_rank: u8,
        opens_at: BTreeMap<OutcomeId, Timestamp>,
    ) -> Result<Self, BoundaryError> {
        let Some(first) = boundaries.first() else {
            return Err(BoundaryError::Empty);
        };
        if first.min != Score::ZERO {
            return Err(BoundaryError::FirstNotZero);
        }
        if boundaries.windows(2).any(|pair| pair[1].min <= pair[0].min) {
            return Err(BoundaryError::NotRising);
        }
        if usize::from(expected_rank) >= boundaries.len() {
            return Err(BoundaryError::ExpectedOutOfRange(expected_rank));
        }
        let rules = vec![Rule::NotifyBelowRank {
            rank: expected_rank,
        }];
        Ok(Self {
            boundaries,
            expected_rank,
            opens_at,
            rules,
        })
    }

    /// Four bands for a primary checkpoint; "secure" is expected.
    pub fn primary_checkpoint(opens_at: BTreeMap<OutcomeId, Timestamp>) -> Self {
        let b = |label: &str, min| Boundary {
            label: label.into(),
            min: Score::of(min),
        };
        let bands = vec![
            b("cambridge.beginning", 0),
            b("cambridge.developing", 4_000),
            b("cambridge.secure", 6_500),
            b("cambridge.excelling", 8_500),
        ];
        Self::new(bands, 2, opens_at).expect("the demo boundaries are valid")
    }

    fn band(&self, score: Score) -> Band {
        // The first boundary starts at zero, so some boundary always matches.
        let rank = self
            .boundaries
            .iter()
            .rposition(|b| score >= b.min)
            .unwrap_or(0);
        Band {
            label: self.boundaries[rank].label.clone(),
            rank: rank as u8,
            of: self.boundaries.len() as u8,
        }
    }
}

impl Framework for CambridgeStyle {
    fn id(&self) -> FrameworkId {
        FrameworkId::new(ID)
    }

    fn judge(&self, evidence: &[&Evidence]) -> Judgement {
        let summative: Vec<Score> = evidence
            .iter()
            .filter(|e| e.kind == EvidenceKind::Summative)
            .map(|e| e.score)
            .collect();
        let all: Vec<Score> = evidence.iter().map(|e| e.score).collect();
        let basis = if summative.is_empty() {
            &all
        } else {
            &summative
        };
        let Some(score) = Score::mean(basis) else {
            return Judgement::NOT_STARTED;
        };
        let band = self.band(score);
        let state = if summative.is_empty() {
            OutcomeState::InProgress
        } else if band.rank >= self.expected_rank {
            OutcomeState::Achieved
        } else {
            OutcomeState::NeedsSupport
        };
        Judgement {
            state,
            score: Some(score),
            band: Some(band),
            attempts: evidence.len() as u32,
        }
    }

    fn availability(&self, ctx: &ProgressContext<'_>) -> Vec<Availability> {
        ctx.graph
            .iter()
            .map(|o| Availability {
                outcome: o.id.clone(),
                lock: self
                    .opens_at
                    .get(&o.id)
                    .filter(|at| ctx.now < **at)
                    .map(|at| LockReason::OpensAt { at: *at }),
            })
            .collect()
    }

    fn rules(&self) -> &[Rule] {
        &self.rules
    }
}
