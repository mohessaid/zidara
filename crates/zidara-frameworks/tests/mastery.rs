#[cfg(target_arch = "wasm32")]
use wasm_bindgen_test::wasm_bindgen_test as test;

use std::collections::BTreeMap;

use zidara_frameworks::mastery::{ID, Mastery};
use zidara_kernel::evidence::{Evidence, EvidenceKind};
use zidara_kernel::framework::{Framework, Judgement, LockReason, OutcomeState, ProgressContext};
use zidara_kernel::ids::*;
use zidara_kernel::locale::Localized;
use zidara_kernel::outcome::{Outcome, OutcomeGraph};
use zidara_kernel::rules::Rule;
use zidara_kernel::score::Score;

fn tries(scores: &[u16]) -> Vec<Evidence> {
    scores
        .iter()
        .map(|bp| Evidence {
            learner: UserId(Uuid::from_u128(1)),
            outcome: OutcomeId::new("F1"),
            activity: ActivityId::new("quiz"),
            score: Score::of(*bp),
            kind: EvidenceKind::Formative,
        })
        .collect()
}

fn judge(scores: &[u16]) -> Judgement {
    let evidence = tries(scores);
    Mastery::standard().judge(&evidence.iter().collect::<Vec<_>>())
}

#[test]
fn the_standard_settings_are_blooms() {
    let m = Mastery::standard();
    assert_eq!(m.id(), FrameworkId::new(ID));
    assert_eq!(ID, "mastery");
    assert_eq!(
        (
            m.threshold,
            m.support_below,
            m.support_after,
            m.recent_weight
        ),
        (Score::of(8_000), Score::of(6_000), 2, Score::of(6_500))
    );
    assert_eq!(
        m.rules(),
        &[Rule::Remediate {
            below: Score::of(6_000),
            after_attempts: 2
        }]
    );
}

#[test]
fn no_evidence_is_not_started() {
    assert_eq!(judge(&[]), Judgement::NOT_STARTED);
}

#[test]
fn eighty_percent_is_mastery() {
    let j = judge(&[8_000]);
    assert_eq!(
        (j.state, j.score, j.attempts),
        (OutcomeState::Achieved, Some(Score::of(8_000)), 1)
    );
    let band = j.band.unwrap();
    assert_eq!(
        (band.label.as_str(), band.rank, band.of),
        ("mastery.mastered", 2, 3)
    );
}

#[test]
fn just_below_eighty_is_developing() {
    let j = judge(&[7_999]);
    assert_eq!(j.state, OutcomeState::InProgress);
    assert_eq!(j.band.unwrap().label, "mastery.developing");
}

#[test]
fn a_weak_first_try_is_progress_and_a_weak_second_needs_support() {
    let one = judge(&[3_000]);
    assert_eq!(one.state, OutcomeState::InProgress);
    assert_eq!(one.band.unwrap().label, "mastery.not_yet");
    assert_eq!(judge(&[3_000, 3_000]).state, OutcomeState::NeedsSupport);
    // 6000 is the line: not below it.
    assert_eq!(judge(&[6_000, 6_000]).state, OutcomeState::InProgress);
}

/// A decaying average: each try counts 65%, everything before it 35% (the Otus/Skyward
/// default), so growth shows quickly without one lucky try deciding mastery.
#[test]
fn recent_tries_count_most() {
    // 0.35 × 3 333 + 0.65 × 10 000 = 7 667, rounded half up.
    let recovering = judge(&[3_333, 3_333, 10_000]);
    assert_eq!(
        (recovering.state, recovering.score),
        (OutcomeState::InProgress, Some(Score::of(7_667)))
    );
    // 0.35 × 7 667 + 0.65 × 10 000 = 9 183: a second good try masters it.
    assert_eq!(
        judge(&[3_333, 3_333, 10_000, 10_000]).score,
        Some(Score::of(9_183))
    );
    assert_eq!(
        judge(&[3_333, 3_333, 10_000, 10_000]).state,
        OutcomeState::Achieved
    );
    // A slip after mastery shows at once: 0.35 × 10 000 = 3 500.
    assert_eq!(judge(&[10_000, 0]).score, Some(Score::of(3_500)));
    // Order matters: the same tries the other way round tell a different story.
    assert_eq!(judge(&[10_000, 3_333, 3_333]).score, Some(Score::of(4_150)));
    // Rounding half up: 0.35 × 1 = 0.35 → 0; 0.35 × 2 = 0.7 → 1.
    assert_eq!(judge(&[1, 0]).score, Some(Score::ZERO));
    assert_eq!(judge(&[2, 0]).score, Some(Score::of(1)));
}

#[test]
fn an_outcome_opens_when_its_prerequisites_are_mastered() {
    let o = |id: &str, needs: &[&str]| Outcome {
        id: OutcomeId::new(id),
        title: Localized::default(),
        prerequisites: needs.iter().map(|n| OutcomeId::new(*n)).collect(),
    };
    let graph =
        OutcomeGraph::new(vec![o("F1", &[]), o("F2", &["F1"]), o("F3", &["F1", "F2"])]).unwrap();
    let mut judgements = BTreeMap::new();
    judgements.insert(OutcomeId::new("F1"), judge(&[9_000]));
    judgements.insert(OutcomeId::new("F2"), judge(&[5_000]));
    let ctx = ProgressContext {
        graph: &graph,
        judgements: &judgements,
        now: Timestamp(0),
    };
    let locks: Vec<Option<LockReason>> = Mastery::standard()
        .availability(&ctx)
        .into_iter()
        .map(|a| a.lock)
        .collect();
    assert_eq!(
        locks,
        [
            None,
            None,
            Some(LockReason::Prerequisite {
                outcome: OutcomeId::new("F2")
            })
        ]
    );
    let nothing = BTreeMap::new();
    let ctx = ProgressContext {
        graph: &graph,
        judgements: &nothing,
        now: Timestamp(0),
    };
    assert_eq!(
        Mastery::standard().availability(&ctx)[1].lock,
        Some(LockReason::Prerequisite {
            outcome: OutcomeId::new("F1")
        })
    );
}
