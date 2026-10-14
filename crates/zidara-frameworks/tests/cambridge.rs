#[cfg(target_arch = "wasm32")]
use wasm_bindgen_test::wasm_bindgen_test as test;

use std::collections::BTreeMap;

use zidara_frameworks::cambridge::{Boundary, BoundaryError, CambridgeStyle, ID};
use zidara_kernel::evidence::{Evidence, EvidenceKind};
use zidara_kernel::framework::{Framework, Judgement, LockReason, OutcomeState, ProgressContext};
use zidara_kernel::ids::*;
use zidara_kernel::locale::Localized;
use zidara_kernel::outcome::{Outcome, OutcomeGraph};
use zidara_kernel::rules::Rule;
use zidara_kernel::score::Score;

fn b(label: &str, min: u16) -> Boundary {
    Boundary {
        label: label.into(),
        min: Score::of(min),
    }
}

fn work(items: &[(u16, EvidenceKind)]) -> Vec<Evidence> {
    items
        .iter()
        .map(|(bp, kind)| Evidence {
            learner: UserId(Uuid::from_u128(1)),
            outcome: OutcomeId::new("F1"),
            activity: ActivityId::new("checkpoint"),
            score: Score::of(*bp),
            kind: *kind,
        })
        .collect()
}

fn judge(items: &[(u16, EvidenceKind)]) -> Judgement {
    let evidence = work(items);
    CambridgeStyle::primary_checkpoint(BTreeMap::new()).judge(&evidence.iter().collect::<Vec<_>>())
}

use EvidenceKind::{Formative as F, Summative as S};

#[test]
fn boundaries_are_checked() {
    let none = BTreeMap::new;
    assert_eq!(
        CambridgeStyle::new(vec![], 0, none()),
        Err(BoundaryError::Empty)
    );
    assert_eq!(
        CambridgeStyle::new(vec![b("a", 1)], 0, none()),
        Err(BoundaryError::FirstNotZero)
    );
    assert_eq!(
        CambridgeStyle::new(vec![b("a", 0), b("b", 0)], 0, none()),
        Err(BoundaryError::NotRising)
    );
    assert_eq!(
        CambridgeStyle::new(vec![b("a", 0), b("b", 10)], 2, none()),
        Err(BoundaryError::ExpectedOutOfRange(2))
    );
    let ok = CambridgeStyle::new(vec![b("a", 0), b("b", 10)], 1, none()).unwrap();
    assert_eq!(ok.rules(), &[Rule::NotifyBelowRank { rank: 1 }]);
    assert_eq!(ok.id(), FrameworkId::new(ID));
    assert_eq!(ID, "cambridge-style");
    assert_eq!(
        BoundaryError::NotRising.to_string(),
        "boundaries must rise strictly"
    );
}

#[test]
fn no_evidence_is_not_started() {
    assert_eq!(judge(&[]), Judgement::NOT_STARTED);
}

#[test]
fn practice_shows_a_band_but_does_not_judge() {
    let j = judge(&[(9_000, F)]);
    assert_eq!(j.state, OutcomeState::InProgress);
    let band = j.band.unwrap();
    assert_eq!(
        (band.label.as_str(), band.rank, band.of),
        ("cambridge.excelling", 3, 4)
    );
}

#[test]
fn summative_work_judges_against_the_boundaries() {
    assert_eq!(judge(&[(6_500, S)]).state, OutcomeState::Achieved);
    assert_eq!(judge(&[(6_499, S)]).state, OutcomeState::NeedsSupport);
    assert_eq!(
        judge(&[(6_499, S)]).band.unwrap().label,
        "cambridge.developing"
    );
    assert_eq!(
        judge(&[(3_999, S)]).band.unwrap().label,
        "cambridge.beginning"
    );
    assert_eq!(
        judge(&[(4_000, S)]).band.unwrap().label,
        "cambridge.developing"
    );
    assert_eq!(
        judge(&[(8_500, S)]).band.unwrap().label,
        "cambridge.excelling"
    );
}

#[test]
fn once_summative_exists_practice_no_longer_counts() {
    let j = judge(&[(0, F), (10_000, S)]);
    assert_eq!((j.score, j.attempts), (Some(Score::FULL), 2));
    assert_eq!(j.state, OutcomeState::Achieved);
}

#[test]
fn the_calendar_opens_outcomes() {
    let o = |id: &str| Outcome {
        id: OutcomeId::new(id),
        title: Localized::default(),
        prerequisites: vec![],
    };
    let graph = OutcomeGraph::new(vec![o("F1"), o("F2"), o("F3")]).unwrap();
    let mut opens = BTreeMap::new();
    opens.insert(OutcomeId::new("F2"), Timestamp(100));
    opens.insert(OutcomeId::new("F3"), Timestamp(200));
    let framework = CambridgeStyle::primary_checkpoint(opens);
    let judgements = BTreeMap::new();
    let at = |now| {
        let ctx = ProgressContext {
            graph: &graph,
            judgements: &judgements,
            now: Timestamp(now),
        };
        framework
            .availability(&ctx)
            .into_iter()
            .map(|a| a.lock)
            .collect::<Vec<_>>()
    };
    assert_eq!(
        at(99),
        [
            None,
            Some(LockReason::OpensAt { at: Timestamp(100) }),
            Some(LockReason::OpensAt { at: Timestamp(200) })
        ]
    );
    assert_eq!(
        at(100),
        [None, None, Some(LockReason::OpensAt { at: Timestamp(200) })]
    );
}
