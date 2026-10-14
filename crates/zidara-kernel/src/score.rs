//! Scores in basis points. Integers keep judgement identical on every target.

use serde::{Deserialize, Serialize};

/// A score from 0 (nothing) to 10 000 (everything), in basis points.
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(try_from = "u16", into = "u16")]
pub struct Score(u16);

/// Why a number is not a score.
#[derive(Clone, Copy, Debug, PartialEq, Eq, thiserror::Error)]
pub enum ScoreError {
    #[error("{0} basis points is above 10000")]
    OutOfRange(u16),
    #[error("{num}/{den} is above one")]
    FractionAboveOne { num: u32, den: u32 },
    #[error("a fraction cannot have a zero denominator")]
    ZeroDenominator,
}

impl Score {
    /// The highest score, in basis points.
    pub const MAX_BP: u16 = 10_000;
    pub const ZERO: Score = Score(0);
    pub const FULL: Score = Score(Self::MAX_BP);

    pub fn new(bp: u16) -> Result<Self, ScoreError> {
        if bp > Self::MAX_BP {
            return Err(ScoreError::OutOfRange(bp));
        }
        Ok(Self(bp))
    }

    /// For constants: a bad value fails at compile time, or panics in tests.
    pub const fn of(bp: u16) -> Self {
        assert!(bp <= Self::MAX_BP, "a score is at most 10000 basis points");
        Self(bp)
    }

    /// `num / den`, rounded half up.
    pub fn from_fraction(num: u32, den: u32) -> Result<Self, ScoreError> {
        if den == 0 {
            return Err(ScoreError::ZeroDenominator);
        }
        if num > den {
            return Err(ScoreError::FractionAboveOne { num, den });
        }
        let (num, den) = (u64::from(num), u64::from(den));
        Ok(Self(
            ((num * u64::from(Self::MAX_BP) + den / 2) / den) as u16,
        ))
    }

    /// The mean, rounded half up; `None` when there is nothing to average.
    pub fn mean(scores: &[Score]) -> Option<Score> {
        if scores.is_empty() {
            return None;
        }
        let n = scores.len() as u64;
        let sum: u64 = scores.iter().map(|s| u64::from(s.0)).sum();
        Some(Self(((sum + n / 2) / n) as u16))
    }

    pub fn bp(self) -> u16 {
        self.0
    }
}

impl TryFrom<u16> for Score {
    type Error = ScoreError;
    fn try_from(bp: u16) -> Result<Self, Self::Error> {
        Self::new(bp)
    }
}

impl From<Score> for u16 {
    fn from(score: Score) -> u16 {
        score.0
    }
}
