//! The wasm-bindgen face, run on wasm32 (Node). Natively, JsError cannot be built, so these
//! tests exist only on wasm32; `Core` carries the contract tests on both targets.
#![cfg(target_arch = "wasm32")]

use wasm_bindgen_test::wasm_bindgen_test as test;
use zidara_wasm::Shell;

const PACK: &str = include_str!("../../../content/lessons/equivalent-fractions.json");
const NOW: f64 = 1_791_162_000_000.0;

fn setup() -> String {
    format!(
        r#"{{"lesson":{PACK},"framework":"mastery","tenant":"00000000-0000-0000-0000-000007e40001","cohort":"00000000-0000-0000-0000-00000000c00a"}}"#
    )
}

#[test]
fn the_shell_forwards_to_the_kernel() {
    let mut shell = Shell::new(&setup()).unwrap();
    let actor =
        r#"{"kind":"human","user":"00000000-0000-0000-0000-0000001ea001","role":"learner"}"#;
    let envelope = r#"{"id":"00000000-0000-0000-0000-000000000001","at":1791162000000,"command":{"type":"submit_quiz","activity":"quiz-f1","responses":[{"type":"choice","index":0},{"type":"fraction","num":3,"den":8}]}}"#;
    assert!(
        shell
            .run(actor, envelope)
            .unwrap()
            .contains(r#""status":"applied""#)
    );
    let view = shell
        .learner_view("00000000-0000-0000-0000-0000001ea001", "en", NOW)
        .unwrap();
    assert!(view.contains(r#""state":"achieved""#));
    let studio = shell
        .studio_view(r#"["00000000-0000-0000-0000-0000001ea001"]"#, "en", NOW)
        .unwrap();
    assert!(studio.contains(r#""achieved":1"#));
    let log = shell.events();
    shell.load(&log).unwrap();
    assert_eq!(shell.events(), log);
}

#[test]
fn errors_are_thrown_not_panicked() {
    assert!(Shell::new("{").is_err());
    let mut shell = Shell::new(&setup()).unwrap();
    assert!(shell.load("{").is_err());
    assert!(shell.run("{", "{}").is_err());
    assert!(shell.learner_view("x", "en", NOW).is_err());
    assert!(shell.studio_view("{", "en", NOW).is_err());
}
