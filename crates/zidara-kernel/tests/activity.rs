#[cfg(target_arch = "wasm32")]
use wasm_bindgen_test::wasm_bindgen_test as test;

use zidara_kernel::activity::*;
use zidara_kernel::ids::OutcomeId;
use zidara_kernel::locale::Localized;
use zidara_kernel::score::Score;

fn fraction(num: u32, den: u32) -> Response {
    Response::Fraction { num, den }
}

fn question(outcome: &str, expected: Expected) -> Question {
    Question {
        id: format!("q-{outcome}"),
        outcome: OutcomeId::new(outcome),
        prompt: Localized::default(),
        expected,
    }
}

#[test]
fn choices_match_the_correct_index_only() {
    let expected = Expected::Choice {
        options: vec![Localized::default(); 3],
        correct: 1,
    };
    assert!(expected.accepts(&Response::Choice { index: 1 }));
    assert!(!expected.accepts(&Response::Choice { index: 2 }));
    assert!(!expected.accepts(&fraction(1, 1)));
}

#[test]
fn exact_fractions_need_the_same_form() {
    let expected = Expected::Exactly { num: 2, den: 4 };
    assert!(expected.accepts(&fraction(2, 4)));
    assert!(!expected.accepts(&fraction(1, 2)));
    assert!(!expected.accepts(&fraction(2, 5)));
    assert!(!expected.accepts(&Response::Choice { index: 0 }));
}

#[test]
fn equivalent_fractions_accept_any_equal_form() {
    let expected = Expected::EquivalentTo { num: 1, den: 2 };
    assert!(expected.accepts(&fraction(1, 2)));
    assert!(expected.accepts(&fraction(2, 4)));
    assert!(expected.accepts(&fraction(50, 100)));
    assert!(!expected.accepts(&fraction(2, 3)));
    assert!(!expected.accepts(&Response::Choice { index: 0 }));
}

#[test]
fn a_zero_denominator_is_never_equivalent() {
    // 0/0 would pass the cross-multiplication test without the guard.
    assert!(!Expected::EquivalentTo { num: 0, den: 1 }.accepts(&fraction(0, 0)));
    assert!(!Expected::EquivalentTo { num: 1, den: 2 }.accepts(&fraction(0, 0)));
}

#[test]
fn huge_numbers_do_not_overflow() {
    let expected = Expected::EquivalentTo {
        num: u32::MAX,
        den: u32::MAX,
    };
    assert!(expected.accepts(&fraction(1, 1)));
    assert!(!expected.accepts(&fraction(u32::MAX - 1, u32::MAX)));
}

#[test]
fn marking_scores_each_outcome_in_order_of_appearance() {
    let questions = [
        question("F2", Expected::Exactly { num: 2, den: 4 }),
        question("F1", Expected::Exactly { num: 1, den: 4 }),
        question("F2", Expected::EquivalentTo { num: 2, den: 3 }),
    ];
    let responses = [fraction(2, 4), fraction(3, 4), fraction(5, 6)];
    let marks = mark(&questions, &responses).unwrap();
    assert_eq!(
        marks,
        vec![
            (OutcomeId::new("F2"), Score::of(5_000)),
            (OutcomeId::new("F1"), Score::ZERO)
        ]
    );
}

#[test]
fn marking_needs_one_response_per_question() {
    let questions = [question("F1", Expected::Exactly { num: 1, den: 4 })];
    assert_eq!(
        mark(&questions, &[]),
        Err(MarkError {
            expected: 1,
            got: 0
        })
    );
    assert_eq!(
        MarkError {
            expected: 1,
            got: 0
        }
        .to_string(),
        "expected 1 responses, got 0"
    );
}

#[test]
fn an_empty_quiz_marks_nothing() {
    assert_eq!(mark(&[], &[]), Ok(vec![]));
}

#[test]
fn activities_read_from_json_with_defaults() {
    let json = r#"{"id":"r","title":{"en":"Read"},"outcomes":["F1"],
                   "kind":{"type":"reading","body":{"en":"Halves"}}}"#;
    let activity: Activity = serde_json::from_str(json).unwrap();
    assert_eq!(activity.role, ActivityRole::Core);
    assert_eq!(
        activity.kind,
        ActivityKind::Reading {
            body: Localized::from_pairs(&[("en", "Halves")]),
            audio: None
        }
    );
}
