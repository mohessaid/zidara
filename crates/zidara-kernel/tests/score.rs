mod support;
#[cfg(target_arch = "wasm32")]
use wasm_bindgen_test::wasm_bindgen_test as test;

use proptest::prelude::*;
use zidara_kernel::score::{Score, ScoreError};

#[test]
fn accepts_the_full_range() {
    assert_eq!(Score::new(0).unwrap().bp(), 0);
    assert_eq!(Score::new(10_000).unwrap().bp(), 10_000);
    assert_eq!(Score::ZERO.bp(), 0);
    assert_eq!(Score::FULL.bp(), Score::MAX_BP);
    assert_eq!(Score::of(8_000).bp(), 8_000);
}

#[test]
fn rejects_scores_above_full() {
    assert_eq!(Score::new(10_001), Err(ScoreError::OutOfRange(10_001)));
}

#[test]
#[should_panic(expected = "basis points")]
fn const_scores_above_full_panic() {
    let _ = Score::of(10_001);
}

#[test]
fn fractions_round_half_up() {
    assert_eq!(Score::from_fraction(1, 3).unwrap().bp(), 3_333);
    assert_eq!(Score::from_fraction(2, 3).unwrap().bp(), 6_667);
    assert_eq!(Score::from_fraction(1, 2).unwrap().bp(), 5_000);
    assert_eq!(Score::from_fraction(1, 20_000).unwrap().bp(), 1);
    assert_eq!(Score::from_fraction(0, 7).unwrap(), Score::ZERO);
    assert_eq!(Score::from_fraction(7, 7).unwrap(), Score::FULL);
    assert_eq!(
        Score::from_fraction(u32::MAX, u32::MAX).unwrap(),
        Score::FULL
    );
}

#[test]
fn bad_fractions_are_errors() {
    assert_eq!(Score::from_fraction(1, 0), Err(ScoreError::ZeroDenominator));
    assert_eq!(
        Score::from_fraction(4, 3),
        Err(ScoreError::FractionAboveOne { num: 4, den: 3 })
    );
}

#[test]
fn mean_rounds_half_up_and_is_none_when_empty() {
    let s = Score::of;
    assert_eq!(Score::mean(&[]), None);
    assert_eq!(Score::mean(&[s(1), s(2)]), Some(s(2)));
    assert_eq!(Score::mean(&[s(0), s(1), s(1)]), Some(s(1)));
    assert_eq!(Score::mean(&[s(0), s(0), s(1)]), Some(s(0)));
    assert_eq!(Score::mean(&[s(10_000), s(10_000)]), Some(Score::FULL));
}

#[test]
fn serde_uses_plain_basis_points_and_validates() {
    assert_eq!(serde_json::to_string(&Score::of(8_000)).unwrap(), "8000");
    assert_eq!(
        serde_json::from_str::<Score>("6500").unwrap(),
        Score::of(6_500)
    );
    assert!(serde_json::from_str::<Score>("10001").is_err());
}

#[test]
fn errors_explain_themselves() {
    assert_eq!(
        ScoreError::OutOfRange(10_001).to_string(),
        "10001 basis points is above 10000"
    );
    assert_eq!(
        ScoreError::ZeroDenominator.to_string(),
        "a fraction cannot have a zero denominator"
    );
    assert_eq!(
        ScoreError::FractionAboveOne { num: 4, den: 3 }.to_string(),
        "4/3 is above one"
    );
}

#[test]
fn mean_stays_between_min_and_max() {
    support::check(proptest::collection::vec(0u16..=10_000, 1..20), |raw| {
        let scores: Vec<Score> = raw.into_iter().map(Score::of).collect();
        let mean = Score::mean(&scores).unwrap();
        prop_assert!(mean >= *scores.iter().min().unwrap());
        prop_assert!(mean <= *scores.iter().max().unwrap());
        Ok(())
    });
}
