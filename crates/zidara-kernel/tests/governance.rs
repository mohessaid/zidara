#[cfg(target_arch = "wasm32")]
use wasm_bindgen_test::wasm_bindgen_test as test;

use zidara_kernel::evidence::EvidenceKind;
use zidara_kernel::governance::*;
use zidara_kernel::score::Score;

fn rubric() -> Rubric {
    Rubric {
        criteria: vec![
            Criterion {
                any_of: vec!["multiply".into(), "ضرب".into(), "multipli".into()],
                weight: 3,
            },
            Criterion {
                any_of: vec!["same".into(), "نفس".into(), "même".into()],
                weight: 1,
            },
        ],
        min_words: 3,
    }
}

#[test]
fn a_full_answer_scores_full_with_clear_confidence() {
    let a = assess(
        &rubric(),
        "You multiply the top and bottom by the same number",
    );
    assert_eq!(a.score, Score::FULL);
    assert_eq!(a.confidence, CLEAR);
    assert_eq!(a.rationale, "met 2 of 2 criteria in 10 words");
}

#[test]
fn weights_shape_the_score() {
    let a = assess(&rubric(), "you multiply them both");
    assert_eq!(a.score, Score::of(7_500));
    assert_eq!(a.confidence, MIXED);
}

#[test]
fn a_wrong_answer_scores_zero_with_clear_confidence() {
    let a = assess(&rubric(), "I add one to each part");
    assert_eq!(a.score, Score::ZERO);
    assert_eq!(a.confidence, CLEAR);
}

#[test]
fn matching_ignores_case() {
    assert_eq!(assess(&rubric(), "MULTIPLY by the SAME").score, Score::FULL);
}

#[test]
fn arabic_diacritics_and_tatweel_are_ignored() {
    let a = assess(&rubric(), "نَضْرِبُ البسط والمقام في نـفـس العدد");
    assert_eq!(a.score, Score::FULL);
}

#[test]
fn short_answers_get_low_confidence() {
    assert_eq!(assess(&rubric(), "multiply same").confidence, TOO_SHORT);
    // Exactly the minimum is long enough.
    assert_eq!(assess(&rubric(), "multiply the same").confidence, CLEAR);
}

#[test]
fn an_empty_rubric_scores_zero() {
    let a = assess(
        &Rubric {
            criteria: vec![],
            min_words: 0,
        },
        "anything at all",
    );
    assert_eq!(a.score, Score::ZERO);
}

#[test]
fn the_policy_applies_only_confident_formative_marks() {
    let policy = Policy::STANDARD;
    let at = |confidence| Assessment {
        score: Score::FULL,
        confidence: Score::of(confidence),
        rationale: String::new(),
    };
    assert_eq!(policy.auto_apply_at, Score::of(8_500));
    assert_eq!(
        policy.decide(EvidenceKind::Formative, &at(8_500)),
        Decision::Apply
    );
    assert_eq!(
        policy.decide(EvidenceKind::Formative, &at(8_499)),
        Decision::Queue
    );
    assert_eq!(
        policy.decide(EvidenceKind::Summative, &at(10_000)),
        Decision::Queue
    );
}

#[test]
fn latin_terms_match_at_word_starts_only() {
    let same = Rubric {
        criteria: vec![Criterion {
            any_of: vec!["same".into()],
            weight: 1,
        }],
        min_words: 0,
    };
    assert_eq!(assess(&same, "I like sesame bread").score, Score::ZERO);
    assert_eq!(assess(&same, "it is the same").score, Score::FULL);
    assert_eq!(assess(&same, "(same) answer").score, Score::FULL);
    let times = Rubric {
        criteria: vec![Criterion {
            any_of: vec!["times".into()],
            weight: 1,
        }],
        min_words: 0,
    };
    assert_eq!(assess(&times, "sometimes it works").score, Score::ZERO);
    // Accented Latin letters start words too.
    let equal = Rubric {
        criteria: vec![Criterion {
            any_of: vec!["égal".into()],
            weight: 1,
        }],
        min_words: 0,
    };
    assert_eq!(assess(&equal, "c'est inégal").score, Score::ZERO);
    assert_eq!(assess(&equal, "c'est égal").score, Score::FULL);
    // A word start, not a whole word: stems still match their longer forms.
    assert_eq!(
        assess(
            &rubric(),
            "I multiplied the top and bottom by the same number"
        )
        .score,
        Score::FULL
    );
    assert_eq!(
        assess(&rubric(), "on les multiplie par le même nombre").score,
        Score::FULL
    );
}

#[test]
fn arabic_and_symbol_terms_still_match_inside_words() {
    // Arabic roots sit inside conjugated words: ضرب in نضرب.
    assert_eq!(
        assess(&rubric(), "نضرب الاثنين في نفس العدد").score,
        Score::FULL
    );
    let times = Rubric {
        criteria: vec![Criterion {
            any_of: vec!["×".into()],
            weight: 1,
        }],
        min_words: 0,
    };
    assert_eq!(assess(&times, "1×2 and 2×2").score, Score::FULL);
}
