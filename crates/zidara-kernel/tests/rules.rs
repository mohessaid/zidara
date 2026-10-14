#[cfg(target_arch = "wasm32")]
use wasm_bindgen_test::wasm_bindgen_test as test;

use zidara_kernel::framework::{Band, Judgement, OutcomeState};
use zidara_kernel::rules::{Action, NotifyReason, Rule, evaluate};
use zidara_kernel::score::Score;

fn judged(score: u16, attempts: u32, rank: Option<u8>) -> Judgement {
    Judgement {
        state: OutcomeState::InProgress,
        score: Some(Score::of(score)),
        band: rank.map(|rank| Band {
            label: "b".into(),
            rank,
            of: 4,
        }),
        attempts,
    }
}

const REMEDIATE: Rule = Rule::Remediate {
    below: Score::of(6_000),
    after_attempts: 2,
};

#[test]
fn remediation_waits_for_enough_attempts() {
    assert_eq!(evaluate(&[REMEDIATE], &judged(3_000, 1, None)), vec![]);
    assert_eq!(
        evaluate(&[REMEDIATE], &judged(3_000, 2, None)),
        vec![
            Action::Remediate,
            Action::NotifyTeacher(NotifyReason::Struggling)
        ]
    );
}

#[test]
fn remediation_starts_strictly_below_the_line() {
    assert_eq!(evaluate(&[REMEDIATE], &judged(6_000, 3, None)), vec![]);
    assert_eq!(evaluate(&[REMEDIATE], &judged(5_999, 3, None)).len(), 2);
}

#[test]
fn no_score_means_no_remediation() {
    assert_eq!(evaluate(&[REMEDIATE], &Judgement::NOT_STARTED), vec![]);
}

#[test]
fn bands_below_the_expected_rank_notify() {
    let rule = [Rule::NotifyBelowRank { rank: 2 }];
    assert_eq!(
        evaluate(&rule, &judged(5_000, 1, Some(1))),
        vec![Action::NotifyTeacher(NotifyReason::BelowExpected)]
    );
    assert_eq!(evaluate(&rule, &judged(7_000, 1, Some(2))), vec![]);
    assert_eq!(evaluate(&rule, &judged(5_000, 1, None)), vec![]);
}

#[test]
fn rules_apply_in_order() {
    let rules = [Rule::NotifyBelowRank { rank: 2 }, REMEDIATE];
    assert_eq!(
        evaluate(&rules, &judged(1_000, 2, Some(0))),
        vec![
            Action::NotifyTeacher(NotifyReason::BelowExpected),
            Action::Remediate,
            Action::NotifyTeacher(NotifyReason::Struggling)
        ]
    );
}
