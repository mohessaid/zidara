#[cfg(target_arch = "wasm32")]
use wasm_bindgen_test::wasm_bindgen_test as test;

use zidara_kernel::locale::Localized;

#[test]
fn picks_the_requested_locale() {
    let title = Localized::from_pairs(&[("ar", "الكسور"), ("en", "Fractions")]);
    assert_eq!(title.get("ar"), "الكسور");
}

#[test]
fn falls_back_to_english() {
    let title = Localized::from_pairs(&[("ar", "الكسور"), ("en", "Fractions")]);
    assert_eq!(title.get("fr"), "Fractions");
}

#[test]
fn falls_back_to_any_language_without_english() {
    let title = Localized::from_pairs(&[("fr", "Fractions"), ("ar", "الكسور")]);
    assert_eq!(title.get("de"), "الكسور");
}

#[test]
fn empty_text_is_empty() {
    assert_eq!(Localized::default().get("en"), "");
}

#[test]
fn serializes_as_a_plain_map() {
    let title = Localized::from_pairs(&[("en", "Fractions")]);
    assert_eq!(
        serde_json::to_string(&title).unwrap(),
        r#"{"en":"Fractions"}"#
    );
}
