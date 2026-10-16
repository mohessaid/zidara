//! The bridge's contract, in JSON, exactly as JavaScript sees it.
#[cfg(target_arch = "wasm32")]
use wasm_bindgen_test::wasm_bindgen_test as test;

use serde_json::{Value, json};
use zidara_wasm::Core;

const PACK: &str = include_str!("../../../content/lessons/equivalent-fractions.json");
const NOW: i64 = 1_791_158_400_000 + 3_600_000;
const AMINA: &str = "00000000-0000-0000-0000-0000001ea001";
const KARIM: &str = "00000000-0000-0000-0000-0000007ea001";

fn setup(framework: &str) -> String {
    let lesson: Value = serde_json::from_str(PACK).unwrap();
    json!({
        "lesson": lesson,
        "framework": framework,
        "tenant": "00000000-0000-0000-0000-000007e40001",
        "cohort": "00000000-0000-0000-0000-00000000c00a"
    })
    .to_string()
}

fn core() -> Core {
    Core::new(&setup("mastery")).unwrap()
}

fn learner() -> String {
    json!({ "kind": "human", "user": AMINA, "role": "learner" }).to_string()
}

fn teacher() -> String {
    json!({ "kind": "human", "user": KARIM, "role": "teacher" }).to_string()
}

fn quiz_f1(id: u128, right: usize) -> String {
    quiz_f1_version(id, 0, right)
}

/// quiz-f1, version `variant`, with both answers right (`right == 2`) or both wrong.
fn quiz_f1_version(id: u128, variant: u32, right: usize) -> String {
    let answers = [(0, (3, 8)), (1, (2, 5)), (2, (4, 7))];
    let (choice, (num, den)) = answers[variant as usize];
    let responses = if right == 2 {
        json!([{ "type": "choice", "index": choice }, { "type": "fraction", "num": num, "den": den }])
    } else {
        json!([{ "type": "choice", "index": (choice + 1) % 3 }, { "type": "fraction", "num": 0, "den": 1 }])
    };
    let id = format!("{:032x}", id);
    let id = format!(
        "{}-{}-{}-{}-{}",
        &id[0..8],
        &id[8..12],
        &id[12..16],
        &id[16..20],
        &id[20..]
    );
    json!({ "id": id, "at": NOW, "command": { "type": "submit_quiz", "activity": "quiz-f1", "variant": variant, "responses": responses } }).to_string()
}

fn parse(s: &str) -> Value {
    serde_json::from_str(s).unwrap()
}

#[test]
fn a_quiz_runs_offline_and_changes_the_view() {
    let mut core = core();
    let result = parse(&core.run(&learner(), &quiz_f1(1, 2)).unwrap());
    assert_eq!(result["status"], "applied");
    assert_eq!(result["events"].as_array().unwrap().len(), 2);
    assert_eq!(result["events"][0]["at"], NOW);
    let view = parse(&core.learner_view(AMINA, "ar", NOW).unwrap());
    assert_eq!(view["outcomes"][0]["judgement"]["state"], "achieved");
    assert_eq!(view["outcomes"][0]["title"], "أتعرّف على الكسر كأجزاء من كلّ");
    assert_eq!(view["next"], "F2");
    assert_eq!(parse(&core.events()).as_array().unwrap().len(), 2);
}

#[test]
fn ids_come_from_the_envelope_and_never_repeat() {
    let mut core = core();
    let first = parse(&core.run(&learner(), &quiz_f1(0x10, 0)).unwrap());
    assert_eq!(core.next_variant(AMINA, "quiz-f1").unwrap(), 1);
    let second = parse(&core.run(&learner(), &quiz_f1_version(0x20, 1, 0)).unwrap());
    let ids: Vec<String> = first["events"]
        .as_array()
        .unwrap()
        .iter()
        .chain(second["events"].as_array().unwrap())
        .map(|e| e["id"].as_str().unwrap().to_owned())
        .collect();
    assert_eq!(ids[0], "00000000-0000-0000-0000-000000000011");
    assert_eq!(ids[1], "00000000-0000-0000-0000-000000000012");
    assert_eq!(ids[2], "00000000-0000-0000-0000-000000000021");
    let mut unique = ids.clone();
    unique.sort();
    unique.dedup();
    assert_eq!(unique.len(), ids.len());
    // The same session replayed gives the same ids: the phone is deterministic.
    let mut again = Core::new(&setup("mastery")).unwrap();
    again.run(&learner(), &quiz_f1(0x10, 0)).unwrap();
    assert_eq!(parse(&again.events())[0]["id"], first["events"][0]["id"]);
}

#[test]
fn a_reused_envelope_is_a_no_op() {
    let mut core = core();
    core.run(&learner(), &quiz_f1(1, 2)).unwrap();
    assert_eq!(
        parse(&core.run(&learner(), &quiz_f1(1, 2)).unwrap()),
        json!({ "status": "duplicate" })
    );
    assert_eq!(parse(&core.events()).as_array().unwrap().len(), 2);
}

#[test]
fn refusals_are_results_not_errors() {
    let mut core = core();
    let switch = json!({ "id": "00000000-0000-0000-0000-000000000005", "at": NOW, "command": { "type": "change_framework", "framework": "cambridge-style" } }).to_string();
    let refused = parse(&core.run(&learner(), &switch).unwrap());
    assert_eq!(
        refused,
        json!({ "status": "refused", "error": { "type": "forbidden", "role": "learner", "permission": "change_framework" } })
    );
    assert_eq!(parse(&core.events()), json!([]));
    // A refused envelope can be retried, e.g. by a teacher with the same id.
    assert_eq!(
        parse(&core.run(&teacher(), &switch).unwrap())["status"],
        "applied"
    );
}

#[test]
fn a_framework_switch_redraws_the_same_evidence() {
    let mut core = core();
    core.run(&learner(), &quiz_f1(1, 2)).unwrap();
    assert_eq!(
        parse(&core.learner_view(AMINA, "en", NOW).unwrap())["outcomes"][0]["judgement"]["state"],
        "achieved"
    );
    let switch = json!({ "id": "00000000-0000-0000-0000-000000000009", "at": NOW, "command": { "type": "change_framework", "framework": "cambridge-style" } }).to_string();
    core.run(&teacher(), &switch).unwrap();
    let view = parse(&core.learner_view(AMINA, "en", NOW).unwrap());
    assert_eq!(view["framework"], "cambridge-style");
    assert_eq!(view["outcomes"][0]["judgement"]["state"], "in_progress");
    assert_eq!(view["outcomes"][1]["lock"]["type"], "opens_at");
    // A week later, by the shell's clock, the scheme of work opens F2.
    let later = parse(
        &core
            .learner_view(AMINA, "en", NOW + 7 * 86_400_000)
            .unwrap(),
    );
    assert_eq!(later["outcomes"][1]["lock"], Value::Null);
}

#[test]
fn load_replaces_the_log_with_the_servers() {
    let mut core = core();
    core.run(&learner(), &quiz_f1(1, 2)).unwrap();
    let server_log = core.events();
    let mut fresh = Core::new(&setup("mastery")).unwrap();
    fresh.load(&server_log).unwrap();
    assert_eq!(fresh.events(), server_log);
    fresh.load("[]").unwrap();
    assert_eq!(fresh.events(), "[]");
    assert_eq!(
        parse(&fresh.learner_view(AMINA, "en", NOW).unwrap())["outcomes"][0]["judgement"]["state"],
        "not_started"
    );
}

#[test]
fn the_studio_counts_the_roster() {
    let mut core = core();
    core.run(&learner(), &quiz_f1(1, 2)).unwrap();
    let roster = json!([AMINA, "00000000-0000-0000-0000-0000001ea002"]).to_string();
    let studio = parse(&core.studio_view(&roster, "fr", NOW).unwrap());
    assert_eq!(studio["outcomes"][0]["achieved"], 1);
    assert_eq!(studio["outcomes"][0]["of"], 2);
    assert_eq!(
        studio["outcomes"][0]["title"],
        "Reconnaître une fraction comme des parts d'un tout"
    );
}

#[test]
fn bad_input_is_an_error_not_a_panic() {
    assert!(Core::new("{").unwrap_err().starts_with("setup: "));
    let unknown = setup("ib");
    assert_eq!(
        Core::new(&unknown).unwrap_err(),
        "setup: unknown framework \"ib\""
    );
    let mut core = core();
    assert!(
        core.run("{", &quiz_f1(1, 2))
            .unwrap_err()
            .starts_with("actor: ")
    );
    assert!(
        core.run(&learner(), "{")
            .unwrap_err()
            .starts_with("envelope: ")
    );
    assert!(core.load("{").unwrap_err().starts_with("events: "));
    assert!(
        core.learner_view("not-a-uuid", "en", NOW)
            .unwrap_err()
            .starts_with("user: ")
    );
    assert!(
        core.studio_view("{", "en", NOW)
            .unwrap_err()
            .starts_with("roster: ")
    );
}

#[test]
fn a_log_naming_an_unknown_framework_cannot_be_viewed() {
    let mut core = core();
    let log = json!([{
        "id": "00000000-0000-0000-0000-000000000001", "at": NOW,
        "tenant": "00000000-0000-0000-0000-000007e40001", "cohort": "00000000-0000-0000-0000-00000000c00a",
        "actor": { "kind": "system" }, "body": { "type": "framework_changed", "framework": "ib" }
    }]);
    core.load(&log.to_string()).unwrap();
    assert_eq!(
        core.learner_view(AMINA, "en", NOW).unwrap_err(),
        "view: unknown framework FrameworkId(\"ib\")"
    );
    assert_eq!(
        core.studio_view("[]", "en", NOW).unwrap_err(),
        "view: unknown framework FrameworkId(\"ib\")"
    );
}

#[test]
fn the_phone_asks_the_kernel_which_version_is_due() {
    let mut core = core();
    assert_eq!(core.next_variant(AMINA, "quiz-f1").unwrap(), 0);
    core.run(&learner(), &quiz_f1(1, 2)).unwrap();
    assert_eq!(core.next_variant(AMINA, "quiz-f1").unwrap(), 1);
    assert_eq!(core.next_variant(KARIM, "quiz-f1").unwrap(), 0);
    assert_eq!(core.next_variant(AMINA, "reading-halves").unwrap(), 0);
    assert!(
        core.next_variant("nope", "quiz-f1")
            .unwrap_err()
            .starts_with("user: ")
    );
    // Retaking the version already seen is refused, with the version that is due.
    let again = parse(&core.run(&learner(), &quiz_f1(2, 2)).unwrap());
    assert_eq!(
        again,
        json!({ "status": "refused", "error": { "type": "wrong_variant", "expected": 1 } })
    );
}
