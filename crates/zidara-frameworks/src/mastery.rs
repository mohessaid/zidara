//! Mastery learning (Bloom): move on when you show the outcome, not when the calendar says.

use zidara_kernel::evidence::Evidence;
use zidara_kernel::framework::{
    Availability, Band, Framework, Judgement, LockReason, OutcomeState, ProgressContext,
};
use zidara_kernel::ids::FrameworkId;
use zidara_kernel::rules::Rule;
use zidara_kernel::score::Score;

pub const ID: &str = "mastery";

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Mastery {
    /// The mean of recent tries that counts as mastered.
    pub threshold: Score,
    /// Below this after `support_after` tries, a learner needs support.
    pub support_below: Score,
    pub support_after: u32,
    /// How many recent tries count.
    pub window: usize,
    rules: Vec<Rule>,
}

impl Mastery {
    /// 80% over the last three tries; support and remediation below 60% after two.
    pub fn standard() -> Self {
        Self {
            threshold: Score::of(8_000),
            support_below: Score::of(6_000),
            support_after: 2,
            window: 3,
            rules: vec![Rule::Remediate {
                below: Score::of(6_000),
                after_attempts: 2,
            }],
        }
    }
}

impl Framework for Mastery {
    fn id(&self) -> FrameworkId {
        FrameworkId::new(ID)
    }

    fn judge(&self, evidence: &[&Evidence]) -> Judgement {
        let attempts = evidence.len() as u32;
        let recent: Vec<Score> = evidence
            .iter()
            .rev()
            .take(self.window)
            .map(|e| e.score)
            .collect();
        let Some(score) = Score::mean(&recent) else {
            return Judgement::NOT_STARTED;
        };
        let (state, rank, label) = if score >= self.threshold {
            (OutcomeState::Achieved, 2, "mastery.mastered")
        } else if score >= self.support_below {
            (OutcomeState::InProgress, 1, "mastery.developing")
        } else if attempts >= self.support_after {
            (OutcomeState::NeedsSupport, 0, "mastery.not_yet")
        } else {
            (OutcomeState::InProgress, 0, "mastery.not_yet")
        };
        Judgement {
            state,
            score: Some(score),
            band: Some(Band {
                label: label.into(),
                rank,
                of: 3,
            }),
            attempts,
        }
    }

    fn availability(&self, ctx: &ProgressContext<'_>) -> Vec<Availability> {
        ctx.graph
            .iter()
            .map(|o| Availability {
                outcome: o.id.clone(),
                lock: o
                    .prerequisites
                    .iter()
                    .find(|p| ctx.state_of(p) != OutcomeState::Achieved)
                    .map(|p| LockReason::Prerequisite { outcome: p.clone() }),
            })
            .collect()
    }

    fn rules(&self) -> &[Rule] {
        &self.rules
    }
}
