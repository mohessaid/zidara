//! The golden session: one scripted lesson, run here natively through `Core`, recorded in
//! `golden/session.json`. The same file is replayed through the wasm build in Node
//! (`web/tests/bridge.test.mjs`) and through the server on PostgreSQL
//! (`crates/zidara-server/tests/golden.rs`); all three must agree.
//!
//! Regenerate after an intended change: `ZIDARA_BLESS=1 cargo test -p zidara-wasm --test golden`.
#![cfg(not(target_arch = "wasm32"))]

use serde_json::{Value, json};
use zidara_wasm::Core;

const PACK: &str = include_str!("../../../content/lessons/equivalent-fractions.json");
const GOLDEN: &str = concat!(env!("CARGO_MANIFEST_DIR"), "/tests/golden/session.json");
const T0: i64 = 1_791_158_400_000 + 3_600_000;
const AMINA: &str = "00000000-0000-0000-0000-0000001ea001";
const KARIM: &str = "00000000-0000-0000-0000-0000007ea001";

fn id(n: u32) -> String {
    format!("0193b000-0000-7000-8000-{n:012x}")
}

fn script() -> Vec<(Value, Value)> {
    let amina = json!({ "kind": "human", "user": AMINA, "role": "learner" });
    let karim = json!({ "kind": "human", "user": KARIM, "role": "teacher" });
    let fraction = |num: u32, den: u32| json!({ "type": "fraction", "num": num, "den": den });
    let step = |n: u32, minutes: i64, command: Value| json!({ "id": id(n), "at": T0 + minutes * 60_000, "command": command });
    let quiz_f2 = |variant: u32, responses: Value| json!({ "type": "submit_quiz", "activity": "quiz-f2", "variant": variant, "responses": responses });
    vec![
        // F1 mastered at the first try.
        (
            amina.clone(),
            step(
                1,
                0,
                json!({ "type": "submit_quiz", "activity": "quiz-f1", "variant": 0,
            "responses": [{ "type": "choice", "index": 0 }, fraction(3, 8)] }),
            ),
        ),
        // F2: one of three right, on versions 0 and 1 (each retake is a new version).
        (
            amina.clone(),
            step(
                2,
                5,
                quiz_f2(0, json!([fraction(2, 4), fraction(0, 1), fraction(0, 1)])),
            ),
        ),
        (
            amina.clone(),
            step(
                3,
                9,
                quiz_f2(1, json!([fraction(2, 6), fraction(0, 1), fraction(0, 1)])),
            ),
        ),
        (
            amina.clone(),
            step(
                4,
                12,
                json!({ "type": "submit_answer", "activity": "explain-f2",
            "answer": "نضرب البسط والمقام في نفس العدد" }),
            ),
        ),
        // After remediation, all three right on version 2.
        (
            amina.clone(),
            step(
                5,
                20,
                quiz_f2(
                    2,
                    json!([fraction(3, 12), fraction(4, 10), fraction(10, 12)]),
                ),
            ),
        ),
        (
            karim,
            step(
                6,
                30,
                json!({ "type": "change_framework", "framework": "cambridge-style" }),
            ),
        ),
    ]
}

/// Runs the script; returns the session record the other two runners replay.
fn session() -> Value {
    let lesson: Value = serde_json::from_str(PACK).unwrap();
    let setup = json!({ "lesson": lesson, "framework": "mastery",
        "tenant": "00000000-0000-0000-0000-000007e40001", "cohort": "00000000-0000-0000-0000-00000000c00a" });
    let mut core = Core::new(&setup.to_string()).unwrap();
    let mut steps = Vec::new();
    let mut views = Vec::new();
    for (actor, envelope) in script() {
        let result: Value =
            serde_json::from_str(&core.run(&actor.to_string(), &envelope.to_string()).unwrap())
                .unwrap();
        assert_eq!(result["status"], "applied", "{result}");
        let at = envelope["at"].as_i64().unwrap();
        let view: Value =
            serde_json::from_str(&core.learner_view(AMINA, "ar", at).unwrap()).unwrap();
        views.push(view);
        steps.push(json!({ "actor": actor, "envelope": envelope, "events": result["events"].as_array().unwrap().len() }));
    }
    let studio: Value = serde_json::from_str(
        &core
            .studio_view(&json!([AMINA]).to_string(), "fr", T0 + 31 * 60_000)
            .unwrap(),
    )
    .unwrap();
    json!({ "setup": setup, "learner": AMINA, "steps": steps, "views_ar": views,
            "studio_fr": { "now": T0 + 31 * 60_000, "roster": [AMINA], "view": studio } })
}

#[test]
fn the_golden_session_is_current() {
    let fresh = session();
    if std::env::var_os("ZIDARA_BLESS").is_some() {
        std::fs::write(GOLDEN, serde_json::to_string_pretty(&fresh).unwrap() + "\n").unwrap();
    }
    let recorded: Value = serde_json::from_str(
        &std::fs::read_to_string(GOLDEN).expect("run with ZIDARA_BLESS=1 once"),
    )
    .unwrap();
    assert_eq!(
        fresh, recorded,
        "the kernel's behaviour changed; review, then re-bless"
    );
}

#[test]
fn the_session_tells_the_story_it_is_meant_to() {
    let s = session();
    let state = |step: usize, outcome: usize| {
        s["views_ar"][step]["outcomes"][outcome]["judgement"]["state"].clone()
    };
    assert_eq!(state(0, 0), "achieved");
    assert_eq!(s["views_ar"][2]["assigned"], json!(["remedial-f2"]));
    // Full marks after two weak tries: the decaying average (ADR 0007) gives 7 667, so the
    // learner is back "in progress", not still "needs support" as with a plain mean (G5).
    assert_eq!(state(4, 1), "in_progress");
    assert_eq!(s["views_ar"][5]["framework"], "cambridge-style");
    assert_eq!(s["studio_fr"]["view"]["flags"][0]["reason"], "struggling");
}
