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
    /// The weight of each newest try in a decaying average (the rest goes to everything before).
    pub recent_weight: Score,
    rules: Vec<Rule>,
}

impl Mastery {
    /// 80% on a decaying average (65% on each newest try); support and remediation below 60%
    /// after two tries. ADR 0007.
    pub fn standard() -> Self {
        Self {
            threshold: Score::of(8_000),
            support_below: Score::of(6_000),
            support_after: 2,
            recent_weight: Score::of(6_500),
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
        let mut tries = evidence.iter().map(|e| u64::from(e.score.bp()));
        let Some(first) = tries.next() else {
            return Judgement::NOT_STARTED;
        };
        let (recent, earlier) = (
            u64::from(self.recent_weight.bp()),
            u64::from(Score::MAX_BP - self.recent_weight.bp()),
        );
        let max = u64::from(Score::MAX_BP);
        // Integer decaying average, rounded half up at each step: identical on every target.
        let decayed = tries.fold(first, |sofar, next| {
            (sofar * earlier + next * recent + max / 2) / max
        });
        // A weighted average never exceeds the largest score averaged, so this is in range.
        let score = Score::of(decayed as u16);
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
